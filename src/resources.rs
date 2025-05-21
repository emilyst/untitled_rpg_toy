pub(crate) mod prelude {
    pub(crate) use super::*;
}

use crate::prelude::*;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;
use std::sync::mpsc::Receiver;

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct SharedRng(ChaCha8Rng);

impl Default for SharedRng {
    fn default() -> Self {
        Self(ChaCha8Rng::from_os_rng())
    }
}

#[derive(Resource, Debug)]
pub(crate) struct InputReceiver(pub(crate) Receiver<String>);

unsafe impl Sync for InputReceiver {}
