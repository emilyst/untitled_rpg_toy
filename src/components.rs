pub(crate) mod prelude {
    pub(crate) use super::{
        Action, Character, Defense, Enemy, Experience, Focus, Health, Player, Slime, Strength,
    };
}

use crate::prelude::*;

#[derive(Component, Copy, Clone, Debug)]
pub(crate) struct Health {
    pub(crate) amount: usize,
}

impl Health {
    pub(crate) fn take_damage(&mut self, amount: usize) {
        self.amount -= amount;
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.amount == 0
    }
}

impl Default for Health {
    fn default() -> Self {
        Health { amount: 100 }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub(crate) struct Experience {
    pub(crate) amount: usize,
}

impl Default for Experience {
    fn default() -> Self {
        Experience { amount: 1 }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub(crate) struct Strength {
    pub(crate) amount: usize,
}

impl Default for Strength {
    fn default() -> Self {
        Strength { amount: 1 }
    }
}

impl From<usize> for Strength {
    fn from(amount: usize) -> Self {
        Strength { amount }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub(crate) struct Defense(pub(crate) usize);

impl Default for Defense {
    fn default() -> Self {
        Defense(1)
    }
}

#[derive(Component, Copy, Clone, Debug, Default)]
#[require(Health, Experience, Strength, Defense)]
pub(crate) struct Character;

#[derive(Component, Copy, Clone, Debug, Default)]
#[require(Name, Character)]
pub(crate) struct Player;

#[derive(Component, Copy, Clone, Debug, Default)]
#[require(Name, Character)]
pub(crate) struct Enemy;

#[derive(Component, Copy, Clone, Debug, Default)]
#[require(Enemy)]
pub(crate) struct Slime;

#[derive(Component, Copy, Clone, Debug, Default)]
pub(crate) struct Focus;

#[derive(Component, Clone, Debug, Default)]
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
