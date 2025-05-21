pub(crate) mod prelude {
    pub(crate) use super::*;
}

use crate::prelude::*;

#[derive(Event, Copy, Clone, Debug, Default)]
pub(crate) struct FocusNeeded;

#[derive(Event, Clone, Debug)]
pub(crate) struct InputRead(pub(crate) String);

#[derive(Event, Clone, Debug)]
pub(crate) struct ActionUsed {
    pub(crate) actor: Option<Entity>,
    pub(crate) target: Option<Entity>,
    pub(crate) action: Action,
}

#[derive(Event, Copy, Clone, Debug)]
pub(crate) struct TargetDamaged {
    pub(crate) target: Entity,
    pub(crate) amount: usize,
}

#[derive(Event, Copy, Clone, Debug)]
pub(crate) struct TargetDefeated {
    pub(crate) target: Entity,
}
