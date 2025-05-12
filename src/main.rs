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

fn read_input(
    input_receiver: Res<InputReceiver>,
    mut input_events: EventWriter<InputReceived>,
) {
    input_events.write(match input_receiver.0.recv() {
        Ok(input) => InputReceived(input),
        Err(_) => InputReceived(Input::Disconnect),
    });
}

fn log_input(
    mut input_events: EventReader<InputReceived>,
    mut prompt_events: EventWriter<InputPromptable>,
) {
    for event in input_events.read() {
        match event {
            InputReceived(Input::Content(line)) => {
                dbg!(line);
                prompt_events.write(InputPromptable);
            }
            _ => {}
        }
    }
}

fn print_input_prompt(mut prompt_events: EventReader<InputPromptable>) {
    for _ in prompt_events.read() {
        print!(">> ");
        io::stdout().flush().unwrap();
    }
}

fn trigger_input_prompt(mut prompt_events: EventWriter<InputPromptable>) {
    prompt_events.write(InputPromptable);
}

fn main() {
    let (transmitter, receiver) = channel::<Input>();

    thread::spawn(move || {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            transmitter.send(Input::from(input)).unwrap();
        }
    });

    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(PostStartup, trigger_input_prompt)
        .add_systems(PreUpdate, (print_input_prompt, read_input).chain())
        .add_systems(Update, log_input)
        .add_event::<InputReceived>()
        .add_event::<InputPromptable>()
        .insert_resource(InputReceiver(receiver))
        .run();
}
