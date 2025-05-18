pub(crate) mod prelude {
    pub(crate) use super::{
        Action, Defense, Enemy, Experience, Focus, Health, Player, Slime, Strength,
    };
}

use crate::prelude::*;

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
#[require(Name, Health, Experience, Strength, Defense)]
pub(crate) struct Enemy;

#[derive(Component, Debug, Default)]
#[require(Enemy)]
pub(crate) struct Slime;

#[derive(Component, Debug, Default)]
pub(crate) struct Focus;

#[derive(Component, Debug, Default)]
pub(crate) enum Action {
    Attack,
    Defend,
    Help,
    Quit,
    Unknown(String),
    #[default]
    None,
}

impl From<&String> for Action {
    fn from(s: &String) -> Self {
        if s.starts_with("a") {
            Action::Attack
        } else if s.starts_with("d") {
            Action::Defend
        } else if s.starts_with("h") {
            Action::Help
        } else if s.starts_with("q") {
            Action::Quit
        } else if s.is_empty() {
            Action::None
        } else {
            Action::Unknown(s.trim().into())
        }
    }
}
