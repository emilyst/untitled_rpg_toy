#![allow(dead_code)]

use bevy::prelude::*;
use std::io::{stdin, Write};
use std::sync::mpsc::{channel, Receiver};
use std::{io, thread};

#[derive(Resource, Debug)]
struct InputReceiver(Receiver<Input>);

unsafe impl Sync for InputReceiver {}

#[derive(Resource, Debug)]
enum Input {
    Content(String),
    Disconnect,
}

impl From<String> for Input {
    fn from(s: String) -> Self {
        Input::Content(s.trim_end().to_string())
    }
}

#[derive(Event, Debug)]
struct InputReceived(Input);

#[derive(Event, Debug)]
struct InputPromptable;

#[derive(Resource, Debug)]
enum Action {
    Attack,
    Defend,
    Help,
    Quit,
    Unknown(String),
    None,
}

#[derive(Event, Debug)]
struct ActionUsed(Action);

fn receive_input(input_receiver: Res<InputReceiver>, mut input_events: EventWriter<InputReceived>) {
    input_events.write(match input_receiver.0.recv() {
        Ok(input) => InputReceived(input),
        Err(_) => InputReceived(Input::Disconnect),
    });
}

fn dispatch_input_to_action(
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

fn print_preamble(mut prompt_events: EventWriter<InputPromptable>) {
    prompt_events.write(InputPromptable);
}

fn prompt_for_input(mut prompt_events: EventReader<InputPromptable>) {
    for _ in prompt_events.read() {
        print!(">> ");
        io::stdout().flush().unwrap();
    }
}

fn handle_action(mut action_events: EventReader<ActionUsed>) {
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
