use bevy::prelude::*;
use bevy_mod_event_group::{EventGroupAppExt, event_group};

pub struct House;
pub struct Car;
pub struct Brick;

#[derive(Debug, Default, Clone, Copy)]
pub enum EventType {
	#[default]
    House,
    Car,
    Brick,
}

#[derive(Debug, Default, Clone, Event)]
#[event_group(
    event_type(my_event_type)
    group(MyEvent),
    events(House, Car, Brick)
)]
pub struct MyEvents {
    pub my_event_type: EventType,
    pub x: i32,
    pub y: String,
}

fn main() {
	App::new()
		.add_plugins(MinimalPlugins)
		.add_event_group::<MyEvents>()
		.add_systems(Startup, send_event.before(MyEvents::event_group_system))
		.add_systems(Update, (house_events, car_events, brick_events).after(MyEvents::event_group_system))
	.run();
}

fn send_event(
	mut events: EventWriter<MyEvents>,
) {
	events.write(MyEvents { my_event_type: EventType::House, y: String::from("My Home"), ..Default::default() });
	events.write(MyEvents { my_event_type: EventType::Car, ..Default::default() });
	events.write(MyEvents { my_event_type: EventType::Brick, x: 500, ..Default::default() });
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
		println!("{} Bricks", event.x);
	}
}