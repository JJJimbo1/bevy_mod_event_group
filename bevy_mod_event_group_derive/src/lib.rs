use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::{discouraged::{AnyDelimiter}, Parse, ParseStream}, parse_macro_input, Data, DeriveInput, Field, Token};

struct EventGroupAttributes {
    main_derives: proc_macro2::TokenStream,
    sub_derives: proc_macro2::TokenStream,
}

impl Parse for EventGroupAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let main_derives = input.parse_any_delimiter()?;
        let _ = input.parse::<Token!(,)>()?;
        let sub_derives = input.parse_any_delimiter()?;
        let _ = input.parse::<Token!(,)>();
        Ok(Self{
            main_derives: main_derives.2.parse::<proc_macro2::TokenStream>()?,
            sub_derives: sub_derives.2.parse::<proc_macro2::TokenStream>()?,
        })
    }
}

#[proc_macro_attribute]
pub fn event_group(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: EventGroupAttributes = parse_macro_input!(attrs as EventGroupAttributes);
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    
    let main_derive = attrs.main_derives;
    let sub_derive = attrs.sub_derives;
    let name = ast.ident;
    
    let (event_ident, event_type, sub_events) = {
        let Data::Struct(data) = &ast.data else { return quote!(compile_error!("Item must be a struct")).into(); };
        let Some(field) = data.fields
            .iter()
            .find(|field| field.attrs.get(0).and_then(|attr| attr.meta.path().segments.get(0))
                .is_some_and(|path| path.ident.to_string() == "events")
            ) else { return quote! { compile_error!("Item must have field attribute 'event'")}.into(); };
        
        let event_ident = field.ident.clone().unwrap().to_token_stream();
        let event_type = field.ty.to_token_stream();
        let Ok(list) = field.attrs[0].meta.require_list() else { return quote!(compile_error!("Attibute 'error' must be a list of items")).into();  };
        let sub_events = list.tokens.clone().into_iter().filter_map(|token| {
            let proc_macro2::TokenTree::Ident(ident) = token else { return None; };
            Some(ident.to_token_stream())
        }).collect::<Vec<proc_macro2::TokenStream>>();
        (event_ident, event_type, sub_events)
    };

    let sub_defs = sub_events.iter().map(|event| {
        quote! {
            #[derive(#sub_derive)]
            pub struct #event;
        }
    }).collect::<proc_macro2::TokenStream>();

    let main_def = {
        let Data::Struct(data) = &ast.data else { return quote!(compile_error!("Item must be a struct")).into(); };
        let fields = data.fields.iter().map(|field| {
            let field = Field{
                attrs: vec![],
                ..field.clone()
            };
            let field = field.to_token_stream();
            quote!(#field,)
        }).collect::<proc_macro2::TokenStream>();
        quote! {
            #[derive(#main_derive)]
            pub struct #name<T = ()> {
                #fields
                phantom_data: PhantomData<T>
            }
        }
    };

    let froms = {
        let Data::Struct(data) = &ast.data else { return quote!(compile_error!("Item must be a struct")).into(); };
        let fields = data.fields.iter().map(|field| {
            let ident = field.ident.to_token_stream();
            quote!(#ident: value.#ident,)
        }).collect::<proc_macro2::TokenStream>();
        sub_events.iter().map(|ident| {
            quote! {
                impl From<#name> for #name<#ident> {
                    fn from(value: #name) -> #name<#ident> {
                        Self {
                            #fields
                            phantom_data: PhantomData,
                        }
                    }
                }
                impl From<#name<#ident>> for #name {
                    fn from(value: #name<#ident>) -> #name {
                        Self {
                            #fields
                            phantom_data: PhantomData,
                        }
                    }
                }
            }
        }).collect::<proc_macro2::TokenStream>()
    };

    let (idents, types, writers, events) = sub_events.iter().map(|token| {
        let upper_case = token;
        let lower_case = token.to_string().to_lowercase().parse::<proc_macro2::TokenStream>().unwrap();
        (
            quote!(mut #lower_case,),
            quote!(EventWriter<#name<#token>>,),
            quote!(#event_type::#upper_case => { #lower_case.write(event.clone().into()); }, ),
            quote!(.add_event::<#name<#token>>()),
        )
    }).collect::<(proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream)>();

    let result = quote! {
        #sub_defs

        #main_def

        impl #name {
            pub fn event_group_system(
                mut reader: EventReader<#name>,
                (
                    #idents
                ): (
                    #types
                )
            ) {
                for event in reader.read() {
                    match event.#event_ident {
                        #writers
                        _ => { }
                    }
                }
            }
        }

        impl bevy_mod_event_group::EventGroup for #name {
            fn add_event_group(app: &mut App) -> &mut App {
                app
                    .add_event::<#name>()
                    #events
                    .add_systems(Update, Self::event_group_system)
            }
        }

        #froms
    };
    result.into()
}