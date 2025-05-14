use bevy::prelude::*;
use std::sync::mpsc::*;

use crate::components::*;
use crate::events::*;
use crate::systems::*;

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Input {
    Content(String),
    Disconnect,
}

impl From<&String> for Input {
    fn from(string: &String) -> Self {
        Input::Content(string.trim().to_lowercase().to_string())
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

impl From<&Input> for Action {
    fn from(input: &Input) -> Self {
        match input {
            Input::Content(string) => match string {
                string if string.starts_with("a") => Action::Attack,
                string if string.starts_with("d") => Action::Defend,
                string if string.starts_with("h") => Action::Help,
                string if string.starts_with("q") => Action::Quit,
                string if string.is_empty() => Action::None,
                string => Action::Unknown(string.to_owned()),
            },
            Input::Disconnect => Action::Quit,
        }
    }
}
