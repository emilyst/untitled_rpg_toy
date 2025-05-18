pub(crate) mod prelude {
    pub(crate) use super::InputReceiver;
}

use crate::prelude::*;

use std::sync::mpsc::Receiver;

#[derive(Resource, Debug)]
pub(crate) struct InputReceiver(pub(crate) Receiver<String>);

unsafe impl Sync for InputReceiver {}
