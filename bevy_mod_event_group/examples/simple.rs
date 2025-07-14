use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_mod_event_group::{EventGroupAppExt, event_group};

#[derive(Debug, Default, Clone, Copy)]
pub enum EventType {
	#[default]
    House,
    Car,
    Brick,
}

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
	(Debug, Default, Clone, Component),
)]
pub struct MyEvent {
    #[events(House, Car, Brick)]
    pub my_event_type: EventType,
	pub y: String,
    pub x: Option<i32>,
}
// impl MyEvents {
//     pub fn event_group_system(
//         mut reader: EventReader<MyEvents>,
//         (mut house, mut car, mut brick): (
//             EventWriter<MyEvents<House>>,
//             EventWriter<MyEvents<Car>>,
//             EventWriter<MyEvents<Brick>>,
//         ),
//     ) {
//         for event in reader.read() {
//             match event.my_event_type {
//                 EventType::House => {
//                     house.write(event.clone().into());
//                 }
//                 EventType::Car => {
//                     car.write(event.clone().into());
//                 }
//                 EventType::Brick => {
//                     brick.write(event.clone().into());
//                 }
//                 _ => {}
//             }
//         }
//     }
// }
// impl bevy_mod_event_group::EventGroup for MyEvents {
//     fn add_event_group(app: &mut App) -> &mut App {
//         app.add_event::<MyEvents>()
//             .add_event::<MyEvents<House>>()
//             .add_event::<MyEvents<Car>>()
//             .add_event::<MyEvents<Brick>>()
//             .add_systems(Update, Self::event_group_system)
//     }
// }
// impl From<MyEvents> for MyEvents<House> {
//     fn from(value: MyEvents) -> MyEvents<House> {
//         Self {
//             my_event_type: value.my_event_type,
//             x: value.x,
//             phantom_data: PhantomData,
//         }
//     }
// }
// impl From<MyEvents<House>> for MyEvents {
//     fn from(value: MyEvents<House>) -> MyEvents {
//         Self {
//             my_event_type: value.my_event_type,
//             x: value.x,
//             phantom_data: PhantomData
//         }
//     }
// }
// impl From<MyEvents> for MyEvents<Car> {
//     fn from(value: MyEvents) -> MyEvents<Car> {
//         Self {
//             my_event_type: value.my_event_type,
//             x: value.x,
//             phantom_data: PhantomData,
//         }
//     }
// }
// impl From<MyEvents<Car>> for MyEvents {
//     fn from(value: MyEvents<Car>) -> MyEvents {
//         Self {
//             my_event_type: value.my_event_type,
//             x: value.x,
//             phantom_data: PhantomData
//         }
//     }
// }
// impl From<MyEvents> for MyEvents<Brick> {
//     fn from(value: MyEvents) -> MyEvents<Brick> {
//         Self {
//             my_event_type: value.my_event_type,
//             x: value.x,
//             phantom_data: PhantomData,
//         }
//     }
// }
// impl From<MyEvents<Brick>> for MyEvents {
//     fn from(value: MyEvents<Brick>) -> MyEvents {
//         Self {
//             my_event_type: value.my_event_type,
//             x: value.x,
//             phantom_data: PhantomData
//         }
//     }
// }