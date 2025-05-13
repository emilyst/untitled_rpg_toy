use bevy::prelude::*;
use std::sync::mpsc::*;

use crate::components::*;
use crate::entities::*;
use crate::events::*;
use crate::systems::*;

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Input {
    Content(String),
    Disconnect,
}

impl From<String> for Input {
    fn from(input: String) -> Self {
        Input::Content(input.trim().to_lowercase().to_string())
    }
}

#[derive(Resource, Debug)]
pub(crate) struct InputReceiver(pub(crate) Receiver<Input>);

unsafe impl Sync for InputReceiver {}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Action {
    Attack,
    Defend,
    Help,
    Quit,
    Unknown(String),
    None,
}

impl From<Input> for Action {
    fn from(input: Input) -> Self {
        match input {
            Input::Content(input) => match input {
                input if input.starts_with("a") => Action::Attack,
                input if input.starts_with("d") => Action::Defend,
                input if input.starts_with("h") => Action::Help,
                input if input.starts_with("q") => Action::Quit,
                input if input.is_empty() => Action::None,
                _ => Action::Unknown(input.to_owned()),
            },
            Input::Disconnect => Action::Quit,
        }
    }
}
