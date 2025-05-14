use bevy::prelude::*;
use std::fmt::*;

use crate::events::*;
use crate::resources::*;
use crate::systems::*;

#[derive(Component, Debug)]
pub(crate) struct Player;

#[derive(Component, Debug)]
pub(crate) struct Enemy;

#[derive(Component, Debug)]
pub(crate) struct Friendly;

#[derive(Component, Debug)]
pub(crate) struct Name(pub(crate) String);

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Component, Debug)]
pub(crate) struct Health(pub(crate) usize);

#[derive(Component, Debug)]
pub(crate) struct Experience(pub(crate) usize);

#[derive(Component, Debug)]
pub(crate) struct Strength(pub(crate) usize);

#[derive(Component, Debug)]
pub(crate) struct Defense(pub(crate) usize);
