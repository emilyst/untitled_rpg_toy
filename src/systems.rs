pub(crate) mod prelude {
    pub(crate) use super::*;
}

use crate::prelude::*;

use crate::print_with_prompt;
use std::io;
use std::io::Write;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub(crate) fn spawn_input_loop_thread(mut commands: Commands) {
    let (sender, receiver) = channel::<String>();

    thread::spawn(move || {
        loop {
            let mut string = String::new();
            let _ = io::stdin().read_line(&mut string);
            let _ = sender.send(string);
        }
    });

    commands.insert_resource(InputReceiver(receiver));
}

pub(crate) fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Name::from("Heroine"), Strength { amount: 10 }));
}

pub(crate) fn spawn_enemies(mut commands: Commands) {
    commands.spawn_batch([
        (Slime, Name::from("Slime"), Strength { amount: 1 }),
        (Slime, Name::from("Slime"), Strength { amount: 1 }),
        (Slime, Name::from("Slime"), Strength { amount: 1 }),
        (Slime, Name::from("Slime"), Strength { amount: 1 }),
        (Slime, Name::from("Slime"), Strength { amount: 1 }),
    ]);
}

pub(crate) fn handle_focus_needed(
    enemies_query: Query<NameOrEntity, With<Enemy>>,
    focus_query: Query<NameOrEntity, With<Focus>>,
    mut focus_needed_event_reader: EventReader<FocusNeeded>,
    mut commands: Commands,
) {
    focus_needed_event_reader.read().for_each(|_| {
        // just focus first enemy for now
        let enemy_to_focus = enemies_query.iter().min_by_key(|enemy| enemy.entity.index());
        let enemy_with_focus = focus_query.single().ok();

        match (enemy_to_focus, enemy_with_focus) {
            (Some(enemy), Some(focus)) => {
                if enemy.entity != focus.entity {
                    commands.entity(focus.entity).remove::<Focus>();
                    commands.entity(enemy.entity).insert(Focus);
                }
            }
            (Some(enemy_to_focus), None) => {
                commands.entity(enemy_to_focus.entity).insert(Focus);
            }
            _ => {}
        }
    });
}

pub(crate) fn receive_input(
    input_receiver: Res<InputReceiver>,
    mut input_received_event_writer: EventWriter<InputRead>,
) {
    if let Ok(input) = input_receiver.0.try_recv() {
        input_received_event_writer.write(InputRead(input));
    }
}

pub(crate) fn handle_input_received(
    player: Single<NameOrEntity, With<Player>>,
    target: Single<NameOrEntity, With<Focus>>,
    mut input_received_event_reader: EventReader<InputRead>,
    mut action_used_event_writer: EventWriter<ActionUsed>,
) {
    input_received_event_reader.read().for_each(|input_received| {
        let InputRead(input) = input_received;

        let actor = Some(player.entity);
        let target = Some(target.entity);
        let action = Action::from(input);

        action_used_event_writer.write(ActionUsed { actor, target, action });
    })
}

pub(crate) fn handle_action_taken(
    query: Query<(NameOrEntity, &Strength)>,
    mut action_used_event_reader: EventReader<ActionUsed>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut damage_received_event_writer: EventWriter<TargetDamaged>,
) {
    action_used_event_reader.read().for_each(|action_used| match action_used {
        ActionUsed { actor: Some(actor), target: Some(target), action: Action::Attack } => {
            if let (Ok((actor_name, strength)), Ok((target_name, _))) =
                (query.get(*actor), query.get(*target))
            {
                print_with_prompt!("{actor_name} used Attack on {target_name}!");

                damage_received_event_writer
                    .write(TargetDamaged { target: target_name.entity, amount: strength.amount });
            }
        }
        ActionUsed { actor: Some(actor), action: Action::Defend, .. } => {
            if let Ok((actor_name, actor_strength)) = query.get(*actor) {
                print_with_prompt!("{actor_name} used Defend!");
            }
        }
        ActionUsed { action: Action::Quit, .. } => {
            app_exit_event_writer.write_default();
            print_with_prompt!("Quitting!");
        }
        ActionUsed { action: Action::Help, .. } => {
            print_with_prompt!("Help!");
        }
        ActionUsed { action: Action::Unknown(input), .. } => {
            print_with_prompt!("Ignoring unrecognized input! ({})", input);
        }
        _ => {}
    });
}

pub(crate) fn handle_target_damaged(
    mut query: Query<(NameOrEntity, &mut Health)>,
    mut target_damaged_event_reader: EventReader<TargetDamaged>,
    mut target_defeated_event_writer: EventWriter<TargetDefeated>,
) {
    target_damaged_event_reader.read().for_each(|target_damaged| {
        if let Ok((target, mut health)) = query.get_mut(target_damaged.target) {
            health.take_damage(target_damaged.amount);

            print_with_prompt!("{target} has {} HP remaining!", health.amount);

            if health.is_zero() {
                target_defeated_event_writer
                    .write(TargetDefeated { target: target_damaged.target });
            }
        }
    });
}

pub(crate) fn handle_target_defeated(
    query: Query<NameOrEntity>,
    mut commands: Commands,
    mut focus_needed_event_writer: EventWriter<FocusNeeded>,
    mut target_defeated_event_reader: EventReader<TargetDefeated>,
) {
    target_defeated_event_reader.read().for_each(|target_defeated| {
        if let Ok(target) = query.get(target_defeated.target) {
            print_with_prompt!("{target} has been defeated!");

            commands.entity(target.entity).despawn();
            focus_needed_event_writer.write_default();
        }
    });
}

pub(crate) fn trigger_enemy_actions(
    query: Query<NameOrEntity, With<Enemy>>,
    player: Single<NameOrEntity, With<Player>>,
    mut action_used_event_writer: EventWriter<ActionUsed>,
) {
    query.iter().for_each(|enemy| {
        let action = Action::Attack;
        let actor = Some(enemy.entity);
        let target = Some(player.entity);

        action_used_event_writer.write(ActionUsed { action, actor, target });
    })
}
