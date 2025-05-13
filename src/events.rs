use bevy::prelude::*;

use crate::components::*;
use crate::entities::*;
use crate::resources::*;
use crate::systems::*;

#[derive(Event, Debug)]
pub(crate) struct ActionUsed(pub(crate) Action);

#[derive(Event, Debug)]
pub(crate) struct InputPromptable;

#[derive(Event, Debug)]
pub(crate) struct InputReceived(pub(crate) Input);
