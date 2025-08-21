#![allow(dead_code)]
use bevy_app::App;
use bevy_ecs::event::Event;

pub use bevy_mod_event_group_derive::event_group;

pub trait FromGroup<T>: Sized {
    fn from_group(value: T) -> Self;
}

pub trait IntoGroup<T>: Sized {
    fn into_group(self) -> T;
}

impl<T, U> IntoGroup<U> for T where U: FromGroup<T> {
    fn into_group(self) -> U {
        U::from_group(self)
    }
}

pub trait EventGroup: Event {
    fn add_event_group(app: &mut App) -> &mut App;
}

pub trait EventGroupAppExt {
    fn add_event_group<G: EventGroup>(&mut self) -> &mut Self;
}

impl EventGroupAppExt for App {
    fn add_event_group<G: EventGroup>(&mut self) -> &mut Self {
        G::add_event_group(self)
    }
}