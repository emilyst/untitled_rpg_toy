use bevy::prelude::*;

use crate::components::*;
use crate::entities::*;
use crate::resources::*;
use crate::systems::*;

#[derive(Event, Deref, DerefMut, Debug)]
pub(crate) struct ActionUsed(pub(crate) Action);

#[derive(Event, Deref, DerefMut, Debug)]
pub(crate) struct InputReceived(pub(crate) Input);
