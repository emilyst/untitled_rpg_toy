use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Health(pub(crate) usize);

impl Default for Health {
    fn default() -> Self {
        Health(100)
    }
}

#[derive(Component, Default)]
pub(crate) struct Experience(pub(crate) usize);

#[derive(Component, Default)]
pub(crate) struct Strength(pub(crate) usize);

#[derive(Component, Default)]
pub(crate) struct Defense(pub(crate) usize);

#[derive(Component, Default)]
#[require(Name, Health, Experience, Strength, Defense)]
pub(crate) struct Player;

#[derive(Component, Default)]
#[require(Name, Health, Experience, Strength, Defense)]
pub(crate) struct Enemy;

#[derive(Component, Default)]
#[require(Enemy)]
pub(crate) struct Slime;

#[derive(Component, Default)]
pub(crate) struct Focus;

#[derive(Component, Default)]
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
        match s {
            s if s.starts_with("a") => Action::Attack,
            s if s.starts_with("d") => Action::Defend,
            s if s.starts_with("h") => Action::Help,
            s if s.starts_with("q") => Action::Quit,
            s if s.is_empty() => Action::None,
            _ => Action::Unknown(s.trim().into()),
        }
    }
}
