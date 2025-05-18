use crate::components::*;
use bevy::prelude::*;

#[derive(Event)]
pub(crate) struct InputReceived(pub(crate) String);

#[derive(Event)]
pub(crate) struct ActionUsed {
    pub(crate) action: Action,
    pub(crate) actor: Option<Entity>,
    pub(crate) target: Option<Entity>,
}

#[derive(Event)]
pub(crate) struct Damaged {
    pub(crate) amount: usize,
    pub(crate) entity: Entity,
}
