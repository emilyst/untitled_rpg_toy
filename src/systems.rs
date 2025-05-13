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
        Ok(input) => InputReceived(input),
        Err(_) => InputReceived(Input::Disconnect),
    });
}

pub(crate) fn handle_input_event(
    mut input_events: EventReader<InputReceived>,
    mut action_events: EventWriter<ActionUsed>,
) {
    for event in input_events.read() {
        let InputReceived(input) = event;
        action_events.write(ActionUsed(Action::from(input.to_owned())));
    }
}

pub(crate) fn handle_action_event(mut action_events: EventReader<ActionUsed>) {
    for event in action_events.read() {
        match event {
            ActionUsed(Action::Attack) => println!("Attack used!"),
            ActionUsed(Action::Defend) => println!("Defend used!"),
            ActionUsed(Action::Help) => println!("Help used!"),
            ActionUsed(Action::Quit) => println!("Quit used!"),
            ActionUsed(Action::None) => println!("Nothing used!"),
            ActionUsed(Action::Unknown(action)) => println!("Unrecognized action! ({})", action),
        }
    }
}
