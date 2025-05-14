#![allow(dead_code)]
#![allow(unused_imports)]

use bevy::MinimalPlugins;
use bevy::app::{App, PluginGroup, Startup};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::default;
use std::io::stdin;
use std::sync::mpsc::channel;

mod components;
mod events;
mod resources;
mod systems;

fn main() {
    let (sender, receiver) = channel::<resources::Input>();

    std::thread::spawn(move || {
        loop {
            let mut string = String::new();
            stdin().read_line(&mut string).unwrap();
            sender.send(resources::Input::from(&string)).unwrap();
        }
    });

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin { ..default() })
        .add_event::<events::InputReceived>()
        .add_event::<events::ActionTaken>()
        .add_systems(Startup, systems::spawn_player)
        .add_systems(Startup, systems::spawn_enemies)
        .add_systems(PreUpdate, systems::target_next_enemy)
        .add_systems(PreUpdate, systems::prompt_for_input)
        .add_systems(PreUpdate, systems::receive_input.after(systems::prompt_for_input))
        .add_systems(
            Update,
            systems::handle_input_received.run_if(on_event::<events::InputReceived>),
        )
        .add_systems(
            Update,
            systems::handle_action_used
                .run_if(on_event::<events::ActionTaken>)
                .after(systems::handle_input_received),
        )
        .insert_resource(resources::InputReceiver(receiver))
        .run();
}
