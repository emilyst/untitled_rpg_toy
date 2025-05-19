pub(crate) mod prelude {
    pub(crate) use super::*;
}

use crate::prelude::*;

#[derive(Event, Debug)]
pub(crate) struct InputRead(pub(crate) String);

#[derive(Event, Debug)]
pub(crate) struct ActionUsed {
    pub(crate) action: Action,
    pub(crate) actor: Option<Entity>,
    pub(crate) target: Option<Entity>,
}

#[derive(Event, Debug)]
pub(crate) struct TargetDamaged {
    pub(crate) amount: usize,
    pub(crate) target: Entity,
}

#[derive(Event, Debug)]
pub(crate) struct TargetDefeated {
    pub(crate) target: Entity,
}
