use bevy::prelude::*;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub(crate) enum GameState {
    #[default]
    InGame,
    Paused,
}

#[derive(Component, Debug, Default)]
pub(crate) struct Name(pub(crate) String);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Component, Debug)]
pub(crate) struct Health(pub(crate) usize);

impl Default for Health {
    fn default() -> Self {
        Health(100)
    }
}

#[derive(Component, Debug, Default)]
pub(crate) struct Experience(pub(crate) usize);

#[derive(Component, Debug, Default)]
pub(crate) struct Strength(pub(crate) usize);

#[derive(Component, Debug, Default)]
pub(crate) struct Defense(pub(crate) usize);

#[derive(Component, Debug, Default)]
#[require(Name, Health, Experience, Strength, Defense)]
pub(crate) struct Player;

#[derive(Component, Debug, Default)]
pub(crate) struct Enemy;

#[derive(Component, Debug, Default)]
#[require(Enemy, Name, Health, Experience, Strength, Defense)]
pub(crate) struct Slime;

#[derive(Component, Debug, Default)]
pub(crate) struct Target;
