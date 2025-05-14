use bevy::ecs::event::*;
use bevy::prelude::*;
use std::io::*;

use crate::components::*;
use crate::events::*;
use crate::resources::*;

pub(crate) fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Name(String::from("Heroine")),
        Health(100),
        Experience(0),
    ));
}

pub(crate) fn spawn_enemies(mut commands: Commands) {
    commands.spawn((
        Enemy,
        Name(String::from("Slime")),
        Health(10),
        Experience(0),
    ));
}

pub(crate) fn prompt_for_input() {
    print!(">> ");
    stdout().flush().unwrap();
}

pub(crate) fn receive_input(
    input_receiver: Res<InputReceiver>,
    mut input_events: EventWriter<InputReceived>,
) {
    input_events.write(match input_receiver.0.recv() {
        Ok(input) => InputReceived(input),
        Err(_) => InputReceived(Input::Disconnect),
    });
}

pub(crate) fn target_enemy(
    query_enemy: Query<Entity, With<Enemy>>,
    mut target: ResMut<Target>,
) {
    // just target whatever enemy for now
    let enemy = query_enemy.iter().next();
    target.0 = enemy;
}

// fn print_selected_character_name_system(
//     query: Query<&Character>,
//     selection: Res<SelectedCharacter>,
// ) {
//     if let Ok(selected_character) = query.get(selection.entity) {
//         println!("{}", selected_character.name);
//     }
// }

pub(crate) fn handle_input_received(
    mut input_events: EventReader<InputReceived>,
    mut action_events: EventWriter<ActionTaken>,
    query_player: Query<Entity, With<Player>>,
    target: Res<Target>,
) {
    let source = query_player.single().unwrap();
    for event in input_events.read() {
        let InputReceived(input) = event;

        action_events.write(ActionTaken {
            action: Action::from(input),
            source,
            target: target.0,
        });
    }
}

pub(crate) fn handle_action_used(
    mut action_events: EventReader<ActionTaken>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for event in action_events.read() {
        match event {
            ActionTaken { action: Action::Attack, source, target } => {
                println!("{source:?} attacked {target:?}!")
            }
            ActionTaken { action: Action::Defend, source, target } => {
                println!("Defend used!")
            }
            ActionTaken { action: Action::Help, source, target } => {
                println!("Help used!")
            }
            ActionTaken { action: Action::Quit, source, target } => {
                println!("Quitting!");
                app_exit_writer.write_default();
            }
            ActionTaken { action: Action::None, source, target } => {
                println!("Nothing used!")
            }
            ActionTaken { action: Action::Unknown(string), source, target } => {
                println!(r#"Ignoring unrecognized action! ("{string}")"#)
            }
        }
    }
}

#[test]
fn update_score_on_event() {
    let mut app = App::new();
    app.add_event::<InputReceived>();
    app.add_event::<ActionTaken>();
    app.add_systems(Update, handle_input_received);

    let event = InputReceived(Input::Content("attack".to_string()));
    app.world_mut().send_event(event);
    app.update();

    let mut cursor = EventCursor::default();
    let action_used_events =
        app.world().get_resource::<Events<ActionTaken>>().unwrap();
    let iterator = cursor.read(action_used_events);

    assert_eq!(iterator.len(), 1);
}
