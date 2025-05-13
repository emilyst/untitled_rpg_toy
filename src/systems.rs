use bevy::prelude::*;
use std::io::*;

use crate::components::*;
use crate::entities::*;
use crate::events::*;
use crate::resources::*;

pub(crate) fn prompt_for_input() {
    print!(">> ");
    stdout().flush().unwrap();
}

pub(crate) fn receive_input(
    input_receiver: Res<InputReceiver>,
    mut input_events: EventWriter<InputReceived>,
) {
    input_events.write(match input_receiver.0.recv() {
        Ok(input) => InputReceived { input },
        Err(_) => InputReceived { input: Input::Disconnect },
    });
}

pub(crate) fn handle_input_received(
    mut input_events: EventReader<InputReceived>,
    mut action_events: EventWriter<ActionUsed>,
) {
    for event in input_events.read() {
        let InputReceived { input } = event;
        action_events.write(ActionUsed { action: Action::from(input) });
    }
}

pub(crate) fn handle_action_used(
    mut action_events: EventReader<ActionUsed>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for event in action_events.read() {
        match event {
            ActionUsed { action: Action::Attack } => println!("Attack used!"),
            ActionUsed { action: Action::Defend } => println!("Defend used!"),
            ActionUsed { action: Action::Help } => println!("Help used!"),
            ActionUsed { action: Action::Quit } => {
                println!("Quitting!");
                app_exit_writer.write_default();
            }
            ActionUsed { action: Action::None } => println!("Nothing used!"),
            ActionUsed { action: Action::Unknown { string } } => {
                println!(r#"Ignoring unrecognized action! ("{string}")"#)
            }
        }
    }
}
