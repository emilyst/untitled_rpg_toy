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

use bevy::log::LogPlugin;
use bevy::state::app::StatesPlugin;

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
    app.add_event::<InputRead>();
    app.add_event::<ActionUsed>();
    app.add_event::<TargetDamaged>();
    app.add_event::<TargetDefeated>();
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
    app.add_systems(
        PreUpdate,
        (
            focus_next_enemy.run_if(in_state(GameState::Running)),
            prompt_for_input.run_if(in_state(GameState::Running)).after(focus_next_enemy),
            receive_input.run_if(in_state(GameState::Running)).after(prompt_for_input),
        )
            .chain(),
    );
}

fn initialize_update_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_input_received
                .run_if(on_event::<InputRead>)
                .run_if(in_state(GameState::Running)),
            handle_action_taken.run_if(on_event::<ActionUsed>).run_if(in_state(GameState::Running)),
            handle_target_damaged
                .run_if(on_event::<TargetDamaged>)
                .run_if(in_state(GameState::Running)),
            handle_target_defeated
                .run_if(on_event::<TargetDefeated>)
                .run_if(in_state(GameState::Running)),
        )
            .chain(),
    );
}
