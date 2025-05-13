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
        .add_systems(PostStartup, print_preamble)
        .add_systems(PreUpdate, (prompt_for_input, receive_input).chain())
        .add_systems(Update, (dispatch_input_to_action, handle_action).chain())
        .add_event::<InputReceived>()
        .add_event::<InputPromptable>()
        .add_event::<ActionUsed>()
        .insert_resource(InputReceiver(receiver))
        .run();
}
