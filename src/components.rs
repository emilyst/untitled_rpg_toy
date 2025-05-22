pub(crate) mod prelude {
    pub(crate) use super::*;
}

use crate::prelude::*;

#[derive(Component, Copy, Clone, Debug)]
pub(crate) struct Health(pub(crate) usize);

impl Default for Health {
    fn default() -> Self {
        Self(100)
    }
}

#[derive(Component, Copy, Clone, Debug, Default)]
pub(crate) struct Experience(pub(crate) usize);

#[derive(Component, Copy, Clone, Debug)]
pub(crate) struct Strength(pub(crate) usize);

impl Default for Strength {
    fn default() -> Self {
        Self(1)
    }
}

impl From<usize> for Strength {
    fn from(amount: usize) -> Self {
        Self(amount)
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub(crate) struct Defense(pub(crate) usize);

impl Default for Defense {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Component, Clone, Debug, Default)]
pub(crate) struct Cooldown(pub(crate) Timer);

#[derive(Component, Copy, Clone, Debug, Default)]
#[require(Health, Experience, Strength, Defense, Cooldown)]
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

impl From<String> for Action {
    fn from(s: String) -> Self {
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

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}
