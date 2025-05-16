#![allow(dead_code)]

use bevy::log::LogPlugin;
use bevy::prelude::*;

mod components;
mod events;
mod resources;
mod systems;

fn main() {
    let mut app = App::new();

    initialize_plugins(&mut app);
    initialize_startup_systems(&mut app);
    initialize_pre_update_systems(&mut app);
    initialize_events(&mut app);
    initialize_input_handler_systems(&mut app);

    app.run();
}

fn initialize_plugins(app: &mut App) {
    app.add_plugins(MinimalPlugins);
    app.add_plugins(LogPlugin { ..default() });
}

fn initialize_events(app: &mut App) {
    app.add_event::<events::InputReceived>();
    app.add_event::<events::ActionTaken>();
}

fn initialize_startup_systems(app: &mut App) {
    app.add_systems(Startup, systems::spawn_input_loop_thread);
    app.add_systems(Startup, systems::spawn_player);
    app.add_systems(Startup, systems::spawn_enemies);
}

fn initialize_pre_update_systems(app: &mut App) {
    app.add_systems(PreUpdate, systems::target_next_enemy);
    app.add_systems(PreUpdate, systems::prompt_for_input);
    app.add_systems(PreUpdate, systems::receive_input.after(systems::prompt_for_input));
}

fn initialize_update_systems(app: &mut App) {
    initialize_input_handler_systems(app);
}

fn initialize_input_handler_systems(app: &mut App) {
    app.add_systems(
        Update,
        systems::handle_input_received.run_if(on_event::<events::InputReceived>),
    );
    app.add_systems(
        Update,
        systems::handle_action_used
            .run_if(on_event::<events::ActionTaken>)
            .after(systems::handle_input_received),
    );
}
