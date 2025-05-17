use bevy::prelude::States;

#[derive(States, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[states(scoped_entities)]
pub(crate) enum GameState {
    Running,
    Paused,
    #[default]
    Starting,
}
