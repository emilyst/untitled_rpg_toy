use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::systems::*;

#[derive(Event, Debug)]
pub(crate) struct ActionUsed {
    pub(crate) action: Action,
    // pub(crate) source: Entity,
    // pub(crate) target: Entity,
}

#[derive(Event, Debug)]
pub(crate) struct InputReceived(pub(crate) Input);
