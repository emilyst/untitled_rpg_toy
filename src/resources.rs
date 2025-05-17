use bevy::prelude::*;
use std::sync::mpsc::Receiver;

#[derive(Resource)]
pub(crate) struct InputReceiver(pub(crate) Receiver<String>);

unsafe impl Sync for InputReceiver {}
