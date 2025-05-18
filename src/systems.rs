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
        (Slime, Name::from(String::from("Slime 3"))),
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
    let _ = io::stdout().flush();
}

pub(crate) fn receive_input(
    input_receiver: Res<InputReceiver>,
    mut input_received_event_writer: EventWriter<InputReceived>,
) {
    input_received_event_writer.write(InputReceived(input_receiver.0.recv().unwrap()));
}

pub(crate) fn handle_input_received(
    query_player: Query<NameOrEntity, With<Player>>,
    query_target: Query<NameOrEntity, With<Focus>>,
    mut input_received_event_reader: EventReader<InputReceived>,
    mut action_used_event_writer: EventWriter<ActionUsed>,
) {
    input_received_event_reader.read().for_each(|input_received| {
        let InputReceived(input) = input_received;

        let action = Action::from(input);
        let actor = query_player.single().map(|t| t.entity).ok();
        let target = query_target.single().map(|t| t.entity).ok();

        action_used_event_writer.write(ActionUsed { action, actor, target });
    })
}

pub(crate) fn handle_action_used(
    mut action_used_event_reader: EventReader<ActionUsed>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut attacked_event_writer: EventWriter<Damaged>,
    query_names_or_entities: Query<NameOrEntity>,
) {
    action_used_event_reader.read().for_each(|action_used| {
        let unknown_name = Name::from("Unnamed");

        match action_used {
            ActionUsed { action: Action::Attack, actor: Some(actor), target: Some(target) } => {
                let actor = query_names_or_entities.get(*actor).unwrap();
                let target = query_names_or_entities.get(*target).unwrap();

                println!("{actor} attacks {target}!");

                attacked_event_writer.write(Damaged { amount: 1, entity: target.entity });
            }
            ActionUsed { action: Action::Defend, actor: Some(actor), .. } => {
                let actor = query_names_or_entities.get(*actor).unwrap();
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

pub(crate) fn handle_damaged(
    mut damaged_event_reader: EventReader<Damaged>,
    mut query_health: Query<&mut Health>,
) {
    damaged_event_reader.read().for_each(|damaged| {
        if let Ok(mut health) = query_health.get_mut(damaged.entity) {
            health.0 -= damaged.amount;
        }
    });
}

#[test]
fn dispatch_action_used_event_on_input_received() {
    let mut app = App::new();
    app.add_event::<InputReceived>();
    app.add_event::<ActionUsed>();
    app.add_systems(Update, handle_input_received);

    let event = InputReceived("attack".to_string());
    app.world_mut().send_event(event);
    app.update();

    let mut cursor = bevy::ecs::event::EventCursor::default();
    let action_used_events = app.world().get_resource::<Events<ActionUsed>>().unwrap();
    let iterator = cursor.read(action_used_events);

    assert_eq!(iterator.len(), 1);
}
