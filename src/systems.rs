use bevy::prelude::*;
use std::io::*;

use crate::components::*;
use crate::entities::*;
use crate::events::*;
use crate::resources::*;

pub(crate) fn print_preamble(mut prompt_events: EventWriter<InputPromptable>) {
    prompt_events.write(InputPromptable);
}

pub(crate) fn prompt_for_input(mut prompt_events: EventReader<InputPromptable>) {
    for _ in prompt_events.read() {
        print!(">> ");
        stdout().flush().unwrap();
    }
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

pub(crate) fn dispatch_input_to_action(
    mut input_events: EventReader<InputReceived>,
    mut prompt_events: EventWriter<InputPromptable>,
    mut action_events: EventWriter<ActionUsed>,
) {
    for event in input_events.read() {
        match event {
            InputReceived(Input::Content(input)) => match input.trim().to_lowercase().to_string() {
                input if input.starts_with("a") => action_events.write(ActionUsed(Action::Attack)),
                input if input.starts_with("d") => action_events.write(ActionUsed(Action::Defend)),
                input if input.starts_with("h") => action_events.write(ActionUsed(Action::Help)),
                input if input.starts_with("q") => action_events.write(ActionUsed(Action::Quit)),
                input if input.is_empty() => action_events.write(ActionUsed(Action::None)),
                _ => action_events.write(ActionUsed(Action::Unknown(input.clone()))),
            },
            InputReceived(Input::Disconnect) => action_events.write(ActionUsed(Action::Quit)),
        };
    }

    prompt_events.write(InputPromptable);
}

pub(crate) fn handle_action(mut action_events: EventReader<ActionUsed>) {
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
