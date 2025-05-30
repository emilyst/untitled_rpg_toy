pub(crate) mod prelude {
    pub(crate) use super::GameState;
}

use crate::prelude::*;

#[derive(States, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[states(scoped_entities)]
pub(crate) enum GameState {
    Running,
    Paused,
    #[default]
    Starting,
}
