use proc_macro::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Type};

#[proc_macro_attribute]
pub fn event_group(input: TokenStream, item: TokenStream) -> TokenStream {

    let ast: DeriveInput = syn::parse(item).unwrap();
    let name = &ast.ident;

    let tokens = input.into_iter().collect::<Vec<TokenTree>>();

    let event_type = tokens.windows(2).filter_map(|tokens| {
        let [marker, event] = tokens else { return None; };
        let TokenTree::Ident(marker) = marker else { return None; };
        if marker.to_string() != "event_type" { return None; }
        let TokenTree::Group(event) = event else { return None; };
        let Some(event) = event.stream().into_iter().find_map(|tree| {
            match tree {
                TokenTree::Ident(ident) => Some(ident),
                _ => None
            }
        }) else { return None; };
        Some(event)
    }).next();

    let Some(event_type) = event_type else { return quote!{ compile_error!("No attribute named event_type") }.into(); };

    let (type_ident, type_type) = {
        let Data::Struct(data) = &ast.data else { return quote!{ compile_error!("Item must be a struct") }.into(); };
        let Some(event_type) = data.fields.iter().find(|field| field.ident.as_ref().is_some_and(|ident| ident.to_string() == event_type.to_string())) else { return quote!{ compile_error!("No field named {}", event_type.to_string()) }.into(); };
        let Some(ident) = &event_type.ident else { return quote!{ compile_error!("No attribute named event_type") }.into(); };
        let Type::Path(event_type) = &event_type.ty else { return quote!{ compile_error!("No attribute named event_type") }.into(); };
        let Some(type_ident) = event_type.path.get_ident() else { return quote!{ compile_error!("No attribute named event_type") }.into(); };
        (ident.to_token_stream(), type_ident.to_token_stream())
    };

    let group = tokens.windows(2).filter_map(|tokens| {
        let [marker, group] = tokens else { return None; };
        let TokenTree::Ident(marker) = marker else { return None; };
        if marker.to_string() != "group" { return None; }
        let TokenTree::Group(group) = group else { return None; };
        let Ok(group) = group.stream().to_string().parse::<proc_macro2::TokenStream>() else { return None; };
        Some(group)
    }).next();

    let Some(group) = group else { return quote!{ compile_error!("No attribute named group") }.into(); };

    let events = tokens.windows(2).filter_map(|tokens| {
        let [marker, events] = tokens else { return None; };
        let TokenTree::Ident(marker) = marker else { return None; };
        if marker.to_string() != "events" { return None; }
        let TokenTree::Group(events) = events else { return None; };
        let events = events.stream().into_iter().filter_map(|event| {
            let TokenTree::Ident(event) = event else { return None; };
            let Ok(l_event) = event.to_string().to_lowercase().parse::<proc_macro2::TokenStream>() else { return None; };
            let Ok(u_event) = event.to_string().parse::<proc_macro2::TokenStream>() else { return None; };
            Some((
                quote! {
                    mut #l_event,
                },
                quote! {
                    EventWriter<#group<#u_event>>,
                },
                quote! {
                    #type_type::#u_event => { #l_event.write(event.clone().into()); },
                },
                quote! {
                    .add_event::<#group<#u_event>>()
                }
            ))
        }).collect::<(proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream)>();

        Some(events)
    }).next();


    let fields = {
        let Data::Struct(data) = &ast.data else { return quote!{ compile_error!("Item must be a struct") }.into(); };
        data.fields.iter().map(|field| {

            let Some(ident) = field.ident.as_ref().map(|ident| ident.to_token_stream()) else { return quote!{ compile_error!("No attribute named event_type") }.into(); };
            let Type::Path(field) = &field.ty else { return quote!{ compile_error!("No attribute named event_type") }.into(); };
            let Some(type_ident) = field.path.get_ident().map(|ident| ident.to_token_stream()) else { return quote!{ compile_error!("No attribute named event_type") }.into(); };
            quote! {
                pub #ident: #type_ident,
            }
        }).collect::<proc_macro2::TokenStream>()
    };

    let from_impl = {
        let Data::Struct(data) = &ast.data else { return quote!{ compile_error!("Item must be a struct") }.into(); };
        data.fields.iter().map(|field| {
            let ident = field.ident.to_token_stream();
            quote! {
                #ident: value.#ident.clone(),
            }
        }).collect::<proc_macro2::TokenStream>()
    };

    let Some((idents, types, writers, events)) = events else { return quote!{ compile_error!("No attribute named group") }.into(); };

    quote!(
        #ast

        impl #name {
            pub fn event_group_system(
                mut reader: EventReader<#name>,
                (
                    #idents
                ): (
                    #types
                ),
            ) {
                for event in reader.read() {
                    match event.#type_ident {
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

        #[derive(Debug, Clone, Event)]
        pub struct #group<T> {
            #fields
            phantom_data: std::marker::PhantomData<T>,
        }

        impl<T> From<#name> for #group<T> {
            fn from(value: #name) -> #group<T> {
                Self {
                    #from_impl
                    phantom_data: std::marker::PhantomData::<T>
                }
            }
        }

        impl<T> From<#group<T>> for #name {
            fn from(value: #group<T>) -> #name {
                Self {
                    #from_impl
                }
            }
        }
    ).into()
}