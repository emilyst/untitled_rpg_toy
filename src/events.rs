use bevy::prelude::*;

use crate::components::*;
use crate::entities::*;
use crate::resources::*;
use crate::systems::*;

#[derive(Event, Deref, DerefMut, Debug)]
pub(crate) struct ActionUsed {
    pub(crate) action: Action,
    // TODO: source
    // TODO: target
}

#[derive(Event, Deref, DerefMut, Debug)]
pub(crate) struct InputReceived {
    pub(crate) input: Input,
    // TODO: source
}
