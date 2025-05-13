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
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            sender.send(Input::from(input)).unwrap();
        }
    });

    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(PostStartup, prompt_for_input)
        .add_systems(PreUpdate, receive_input)
        .add_systems(Update, (handle_input_event, handle_action_event).chain())
        .add_systems(PostUpdate, prompt_for_input)
        .add_event::<InputReceived>()
        .add_event::<ActionUsed>()
        .insert_resource(InputReceiver(receiver))
        .run();
}
