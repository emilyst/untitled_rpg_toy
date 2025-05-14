use crate::resources;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub(crate) struct ActionTaken {
    pub(crate) action: resources::Action,
    pub(crate) source: Option<Entity>,
    pub(crate) target: Option<Entity>,
}

#[derive(Event, Debug)]
pub(crate) struct InputReceived(pub(crate) resources::Input);
