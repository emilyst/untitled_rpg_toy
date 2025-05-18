pub(crate) mod prelude {
    pub(crate) use super::{ActionUsed, Damaged, InputReceived};
}

use crate::prelude::*;

#[derive(Event, Debug)]
pub(crate) struct InputReceived(pub(crate) String);

#[derive(Event, Debug)]
pub(crate) struct ActionUsed {
    pub(crate) action: Action,
    pub(crate) actor: Option<Entity>,
    pub(crate) target: Option<Entity>,
}

#[derive(Event, Debug)]
pub(crate) struct Damaged {
    pub(crate) amount: usize,
    pub(crate) entity: Entity,
}
