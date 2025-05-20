#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod components;
mod events;
mod prelude;
mod resources;
mod states;
mod systems;

use crate::prelude::*;
use bevy::app::ScheduleRunnerPlugin;
use bevy::log::LogPlugin;
use bevy::state::app::StatesPlugin;
use std::time;

fn main() {
    let mut app = App::new();

    add_plugins(&mut app);
    add_resources(&mut app);
    add_startup_systems(&mut app);
    add_post_startup_systems(&mut app);
    add_update_systems(&mut app);
    add_events(&mut app);

    app.init_state::<GameState>();
    app.run();
}

fn add_resources(app: &mut App) {}

fn add_plugins(app: &mut App) {
    app.add_plugins(
        MinimalPlugins
            .set(ScheduleRunnerPlugin::run_loop(time::Duration::from_secs_f64(1.0 / 60.))),
    );
    app.add_plugins(StatesPlugin);
    app.add_plugins(LogPlugin { ..default() });
}

fn add_events(app: &mut App) {
    app.add_event::<ActionUsed>();
    app.add_event::<FocusNeeded>();
    app.add_event::<InputNeeded>();
    app.add_event::<InputRead>();
    app.add_event::<TargetDamaged>();
    app.add_event::<TargetDefeated>();
}

fn add_startup_systems(app: &mut App) {
    app.add_systems(Startup, spawn_input_loop_thread);
    app.add_systems(Startup, spawn_player);
    app.add_systems(Startup, spawn_enemies);
}

fn add_post_startup_systems(app: &mut App) {
    app.add_systems(PostStartup, |mut commands: Commands| {
        commands.set_state(GameState::Running);
    });
}

fn add_update_systems(app: &mut App) {
    app.add_systems(Update, receive_input.run_if(in_state(GameState::Running)));

    app.add_systems(
        Update,
        (
            handle_focus_needed.run_if(on_event::<FocusNeeded>),
            handle_action_taken.run_if(on_event::<ActionUsed>),
            handle_input_received.run_if(on_event::<InputRead>),
            handle_target_damaged.run_if(on_event::<TargetDamaged>),
            handle_target_defeated.run_if(on_event::<TargetDefeated>),
            handle_input_needed.run_if(on_event::<InputNeeded>),
        )
            .chain()
            .run_if(in_state(GameState::Running)),
    );
}
