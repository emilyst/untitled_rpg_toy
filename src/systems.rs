use crate::components::*;
use crate::events::*;
use crate::resources::*;

use bevy::prelude::*;

use std::io;
use std::io::Write;
use std::sync::mpsc::sync_channel;

pub(crate) fn spawn_input_loop_thread(mut commands: Commands) {
    let (sender, receiver) = sync_channel::<String>(1);

    std::thread::spawn(move || {
        loop {
            let mut string = String::new();
            io::stdin().read_line(&mut string).unwrap();
            sender.send(string).unwrap();
        }
    });

    commands.insert_resource(InputReceiver(receiver));
}

pub(crate) fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Name::from(String::from("Heroine"))));
}

pub(crate) fn spawn_enemies(mut commands: Commands) {
    commands.spawn_batch([
        (Slime, Name::from(String::from("Slime 1"))),
        (Slime, Name::from(String::from("Slime 2"))),
    ]);
}

pub(crate) fn focus_next_enemy(
    query_enemy: Query<Entity, With<Enemy>>,
    query_focus: Query<Entity, With<Focus>>,
    mut commands: Commands,
) {
    // just target whatever enemy for now
    let enemy = query_enemy.iter().next();
    let focus = query_focus.single().ok();

    match (enemy, focus) {
        (Some(enemy), Some(focus)) if enemy != focus => {
            commands.entity(focus).remove::<Focus>();
            commands.entity(enemy).insert(Focus);
        }
        (Some(enemy), None) => {
            commands.entity(enemy).insert(Focus);
        }
        _ => {}
    }
}

pub(crate) fn prompt_for_input() {
    print!(">> ");
    io::stdout().flush().expect("Standard out should have flushed!");
}

pub(crate) fn receive_input(
    input_receiver: Res<InputReceiver>,
    mut input_received_event_writer: EventWriter<InputReceived>,
) {
    input_received_event_writer.write(InputReceived { input: input_receiver.0.recv().unwrap() });
}

pub(crate) fn handle_input_received(
    query_player: Query<NameOrEntity, With<Player>>,
    query_target: Query<NameOrEntity, With<Focus>>,
    mut input_received_event_reader: EventReader<InputReceived>,
    mut action_used_event_writer: EventWriter<ActionUsed>,
) {
    input_received_event_reader.read().for_each(|input_received| {
        let InputReceived { input: content } = input_received;

        action_used_event_writer.write(ActionUsed {
            action: Action::from(content),
            actor: query_player.single().ok().map(|t| t.entity),
            target: query_target.single().ok().map(|t| t.entity),
        });
    })
}

pub(crate) fn handle_action_used(
    mut action_used_event_reader: EventReader<ActionUsed>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    query_names: Query<NameOrEntity>,
) {
    action_used_event_reader.read().for_each(|action_used| {
        let unknown_name = Name::from("Unnamed");

        match action_used {
            ActionUsed { action: Action::Attack, actor: Some(actor), target: Some(target) } => {
                let actor = query_names.get(*actor).unwrap();
                let target = query_names.get(*target).unwrap();
                println!("{actor} attacks {target}!");
            }
            ActionUsed { action: Action::Defend, actor: Some(actor), .. } => {
                let actor = query_names.get(*actor).unwrap();
                println!("{actor} defends!")
            }
            ActionUsed { action: Action::Quit, .. } => {
                app_exit_event_writer.write_default();
                println!("Quitting!");
            }
            ActionUsed { action: Action::Help, .. } => {
                println!("Help used!")
            }
            ActionUsed { action: Action::Unknown(input), .. } => {
                warn!(r#"Ignoring unrecognized input! ("{}")"#, input)
            }
            _ => {}
        }
    });
}

#[test]
fn dispatch_action_event_on_input_received() {
    let mut app = App::new();
    app.add_event::<InputReceived>();
    app.add_event::<ActionUsed>();
    app.add_systems(Update, handle_input_received);

    let event = InputReceived { input: "attack".into() };
    app.world_mut().send_event(event);
    app.update();

    let mut cursor = bevy::ecs::event::EventCursor::default();
    let action_used_events = app.world().get_resource::<Events<ActionUsed>>().unwrap();
    let iterator = cursor.read(action_used_events);

    assert_eq!(iterator.len(), 1);
}
