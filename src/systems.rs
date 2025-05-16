use crate::{components, events, resources};
use bevy::prelude::*;
use std::io;
use std::io::Write;
use std::sync::mpsc;

pub(crate) fn spawn_input_loop_thread(mut commands: Commands) {
    let (sender, receiver) = mpsc::channel::<resources::Input>();

    std::thread::spawn(move || {
        loop {
            let mut string = String::new();
            io::stdin().read_line(&mut string).unwrap();
            sender.send(resources::Input::from(&string)).unwrap();
        }
    });

    commands.insert_resource(resources::InputReceiver(receiver));
}

pub(crate) fn spawn_player(mut commands: Commands) {
    commands.spawn((components::Player, components::Name(String::from("Heroine"))));
}

pub(crate) fn spawn_enemies(mut commands: Commands) {
    commands.spawn_batch([
        (components::Slime, components::Name(String::from("Slime 1"))),
        (components::Slime, components::Name(String::from("Slime 2"))),
    ]);
}

pub(crate) fn prompt_for_input() {
    print!(">> ");
    io::stdout().flush().expect("Standard out should have flushed!");
}

pub(crate) fn receive_input(
    input_receiver: Res<resources::InputReceiver>,
    mut input_events: EventWriter<events::InputReceived>,
) {
    input_events.write(match input_receiver.0.recv() {
        Ok(input) => events::InputReceived(input),
        Err(_) => events::InputReceived(resources::Input::Disconnect),
    });
}

pub(crate) fn target_next_enemy(
    query_enemy: Query<Entity, With<components::Enemy>>,
    query_target: Query<Entity, With<components::Target>>,
    mut commands: Commands,
) {
    // just target whatever enemy for now
    let enemy = query_enemy.iter().next();
    let target = query_target.single().ok();

    match (enemy, target) {
        (Some(enemy), Some(target)) if enemy != target => {
            commands.entity(target).remove::<components::Target>();
            commands.entity(enemy).insert(components::Target);
        }
        (Some(enemy), None) => {
            commands.entity(enemy).insert(components::Target);
        }
        _ => {}
    }
}

pub(crate) fn handle_input_received(
    query_player: Query<Entity, With<components::Player>>,
    query_target: Query<Entity, With<components::Target>>,
    mut input_events: EventReader<events::InputReceived>,
    mut action_events: EventWriter<events::ActionTaken>,
) {
    for event in input_events.read() {
        let events::InputReceived(input) = event;

        action_events.write(events::ActionTaken {
            action: resources::Action::from(input),
            source: query_player.single().ok(),
            target: query_target.single().ok(),
        });
    }
}

pub(crate) fn handle_action_used(
    mut action_events: EventReader<events::ActionTaken>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for event in action_events.read() {
        match event {
            events::ActionTaken {
                action: resources::Action::Attack,
                source: Some(source),
                target: Some(target),
            } => {
                info!("{source:?} attacks {target:?}!")
            }
            events::ActionTaken {
                action: resources::Action::Defend,
                source: Some(source),
                target: _,
            } => {
                info!("{source:?} defends!")
            }
            events::ActionTaken { action: resources::Action::Help, .. } => {
                info!("Help used!")
            }
            events::ActionTaken { action: resources::Action::Quit, .. } => {
                info!("Quitting!");
                app_exit_writer.write_default();
            }
            events::ActionTaken { action: resources::Action::Unknown(string), .. } => {
                warn!(r#"Ignoring unrecognized input! ("{string}")"#)
            }
            events::ActionTaken { action: resources::Action::None, .. } => {}
            _ => todo!(),
        }
    }
}

#[test]
fn dispatch_action_event_on_input_received() {
    let mut app = App::new();
    app.add_event::<events::InputReceived>();
    app.add_event::<events::ActionTaken>();
    app.add_systems(Update, handle_input_received);

    let event = events::InputReceived(resources::Input::Content("attack".to_string()));
    app.world_mut().send_event(event);
    app.update();

    let mut cursor = bevy::ecs::event::EventCursor::default();
    let action_used_events = app.world().get_resource::<Events<events::ActionTaken>>().unwrap();
    let iterator = cursor.read(action_used_events);

    assert_eq!(iterator.len(), 1);
}
