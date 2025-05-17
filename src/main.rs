#![allow(dead_code)]
#![allow(unused_variables)]

use crate::events::*;
use crate::states::*;
use crate::systems::*;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

mod components;
mod events;
mod resources;
mod states;
mod systems;

fn main() {
    let mut app = App::new();

    initialize_plugins(&mut app);
    initialize_resources(&mut app);
    initialize_startup_systems(&mut app);
    initialize_post_startup_systems(&mut app);
    initialize_pre_update_systems(&mut app);
    initialize_update_systems(&mut app);
    initialize_events(&mut app);

    app.init_state::<GameState>();
    app.run();
}

fn initialize_resources(app: &mut App) {}

fn initialize_plugins(app: &mut App) {
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.add_plugins(LogPlugin { ..default() });
}

fn initialize_events(app: &mut App) {
    app.add_event::<InputReceived>();
    app.add_event::<ActionUsed>();
}

fn initialize_startup_systems(app: &mut App) {
    app.add_systems(Startup, spawn_input_loop_thread);
    app.add_systems(Startup, spawn_player);
    app.add_systems(Startup, spawn_enemies);
}

fn initialize_post_startup_systems(app: &mut App) {
    app.add_systems(PostStartup, |mut commands: Commands| {
        commands.set_state(GameState::Running);
    });
}

fn initialize_pre_update_systems(app: &mut App) {
    app.add_systems(PreUpdate, focus_next_enemy.run_if(in_state(GameState::Running)));
    app.add_systems(
        PreUpdate,
        prompt_for_input.run_if(in_state(GameState::Running)).after(focus_next_enemy),
    );
    app.add_systems(
        PreUpdate,
        receive_input.run_if(in_state(GameState::Running)).after(prompt_for_input),
    );
}

fn initialize_update_systems(app: &mut App) {
    initialize_event_handler_systems(app);
}

fn initialize_event_handler_systems(app: &mut App) {
    app.add_systems(
        Update,
        handle_input_received
            .run_if(on_event::<InputReceived>)
            .run_if(in_state(GameState::Running))
            .before(handle_action_used),
    );
    app.add_systems(
        Update,
        handle_action_used
            .run_if(on_event::<ActionUsed>)
            .run_if(in_state(GameState::Running))
            .after(handle_input_received),
    );
}
