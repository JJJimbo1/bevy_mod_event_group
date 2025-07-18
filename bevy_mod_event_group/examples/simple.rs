use bevy::prelude::*;
use bevy_mod_event_group::{EventGroupAppExt, event_group};

#[derive(Debug, Default, Clone, Copy)]
pub enum EventType {
	#[default]
    House,
    Car,
    Brick,
}

pub struct House;
pub struct Car;
pub struct Brick;


fn main() {
	App::new()
		.add_plugins(MinimalPlugins)
		.add_event_group::<MyEvent>()
		.add_systems(Startup, send_event.before(MyEvent::event_group_system))
		.add_systems(Update, (house_events, car_events, brick_events).after(MyEvent::event_group_system))
	.run();
}

fn send_event(
	mut events: EventWriter<MyEvent>,
) {
	events.write(MyEvent { my_event_type: EventType::House, y: String::from("My Home"), ..default() });
	events.write(MyEvent { my_event_type: EventType::Car, ..default() });
	events.write(MyEvent { my_event_type: EventType::Brick, x: Some(500), ..default() });
}

fn house_events(
	mut events: EventReader<MyEvent<House>>,
) {
	for event in events.read() {
		println!("Home sweet home. {}", event.y);
	}
}

fn car_events(
	mut events: EventReader<MyEvent<Car>>,
) {
	for _event in events.read() {
		println!("Vroom vroom!");
	}
}

fn brick_events(
	mut events: EventReader<MyEvent<Brick>>,
) {
	for event in events.read() {
		println!("{} Bricks", event.x.unwrap());
	}
}

#[event_group(
	(Debug, Default, Clone, Event),
)]
pub struct MyEvent {
    #[events(House, Car, Brick)]
    pub my_event_type: EventType,
	pub y: String,
    pub x: Option<i32>,
}