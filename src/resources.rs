use bevy::prelude::*;
use std::sync::mpsc::*;

use crate::components::*;
use crate::entities::*;
use crate::events::*;
use crate::systems::*;

#[derive(Resource, Debug)]
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

#[derive(Resource, Debug)]
pub(crate) enum Action {
    Attack,
    Defend,
    Help,
    Quit,
    Unknown(String),
    None,
}
