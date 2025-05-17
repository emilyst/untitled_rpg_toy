use crate::components::*;
use bevy::prelude::*;

#[derive(Event)]
pub(crate) struct InputReceived {
    pub(crate) input: String,
}

#[derive(Event)]
pub(crate) struct ActionUsed {
    pub(crate) action: Action,
    pub(crate) actor: Option<Entity>,
    pub(crate) target: Option<Entity>,
}

#[derive(Event)]
pub(crate) struct DamageTaken {
    pub(crate) damaged: Option<Entity>,
    pub(crate) amount: usize,
}
