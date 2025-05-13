#![allow(dead_code)]
#![allow(unused_imports)]

use bevy::prelude::*;
use std::io::*;
use std::sync::mpsc::*;
use std::thread;

mod components;
mod entities;
mod events;
mod resources;
mod systems;

use crate::components::*;
use crate::entities::*;
use crate::events::*;
use crate::resources::*;
use crate::systems::*;

fn main() {
    let (sender, receiver) = channel::<Input>();

    thread::spawn(move || {
        loop {
            let mut string = String::new();
            stdin().read_line(&mut string).unwrap();
            sender.send(Input::from(&string)).unwrap();
        }
    });

    App::new()
        .add_plugins(MinimalPlugins)
        .add_event::<InputReceived>()
        .add_event::<ActionUsed>()
        .add_systems(PreUpdate, (prompt_for_input, receive_input).chain())
        .add_systems(Update, (handle_input_event, handle_action_event).chain())
        .insert_resource(InputReceiver(receiver))
        .run();
}
