pub(crate) mod prelude {
    pub(crate) use super::*;
}

use crate::prelude::*;
use crate::print_with_prompt;

use std::io;
use std::io::Write;
use std::ops::Deref;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;

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
    commands.spawn((Player, Name::from("Heroine"), Strength(10)));
}

pub(crate) fn spawn_enemies(mut commands: Commands, shared_rng: ResMut<SharedRng>) {
    let shared_rng = shared_rng.into_inner();

    commands.spawn((
        Slime,
        Name::from("Slime"),
        Cooldown(Timer::from_seconds(1., TimerMode::Repeating)),
    ));
}

pub(crate) fn handle_focus_needed(
    mut focus_needed_event_reader: EventReader<FocusNeeded>,
    enemies_query: Query<NameOrEntity, With<Enemy>>,
    focus_query: Query<NameOrEntity, With<Focus>>,
    mut commands: Commands,
) {
    focus_needed_event_reader.read().for_each(|_| {
        // just focus first enemy for now
        let enemy_to_focus = enemies_query
            .iter()
            .min_by_key(|enemy| enemy.entity.index());
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

pub(crate) fn try_receive_input(
    input_receiver: Res<InputReceiver>,
    mut input_received_event_writer: EventWriter<InputRead>,
) {
    let InputReceiver(input_receiver) = input_receiver.into_inner();

    if let Ok(input) = input_receiver.try_recv() {
        input_received_event_writer.write(InputRead(input));
    }
}

pub(crate) fn handle_input_received(
    mut input_received_event_reader: EventReader<InputRead>,
    player: Single<NameOrEntity, With<Player>>,
    target: Single<NameOrEntity, With<Focus>>,
    mut action_used_event_writer: EventWriter<ActionUsed>,
) {
    input_received_event_reader
        .read()
        .for_each(|input_received| {
            let InputRead(input) = input_received;

            let actor = Some(player.entity);
            let target = Some(target.entity);
            let action = Action::from(input.to_owned());

            action_used_event_writer.write(ActionUsed {
                actor,
                target,
                action,
            });
        })
}

pub(crate) fn handle_action_taken(
    mut action_used_event_reader: EventReader<ActionUsed>,
    actor_and_strength_query: Query<(NameOrEntity, &Strength)>,
    target_query: Query<NameOrEntity>,
    mut damage_received_event_writer: EventWriter<TargetDamaged>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    action_used_event_reader.read().for_each(|action_used| {
        let ActionUsed {
            action,
            actor,
            target,
        } = action_used;

        match action {
            Action::Attack => {
                if let (Some(actor), Some(target)) = (actor, target) {
                    if let (Ok((actor, strength)), Ok(target)) = (
                        actor_and_strength_query.get(*actor),
                        target_query.get(*target),
                    ) {
                        print_with_prompt!("{actor} used Attack on {target}!");

                        let Strength(strength) = strength;

                        damage_received_event_writer.write(TargetDamaged {
                            target: target.entity,
                            amount: *strength,
                        });
                    }
                }
            }
            Action::Defend => {
                if let Some(actor) = actor {
                    if let Ok((actor, _)) = actor_and_strength_query.get(*actor) {
                        print_with_prompt!("{actor} used Defend!");
                    }
                }
            }
            Action::Quit => {
                app_exit_event_writer.write_default();
                print_with_prompt!("Quitting!");
            }
            Action::Help => {
                print_with_prompt!("Help!");
            }
            Action::Unknown(input) => {
                print_with_prompt!("Ignoring unrecognized input: {}", input);
            }
            _ => {}
        }
    });
}

pub(crate) fn handle_target_damaged(
    mut target_damaged_event_reader: EventReader<TargetDamaged>,
    mut query: Query<(NameOrEntity, &mut Health)>,
    mut target_defeated_event_writer: EventWriter<TargetDefeated>,
) {
    target_damaged_event_reader
        .read()
        .for_each(|target_damaged| {
            if let Ok((target, health)) = query.get_mut(target_damaged.target) {
                let Health(health) = health.into_inner();

                *health -= target_damaged.amount;
                print_with_prompt!("{target} has {} HP remaining!", health);

                if *health == 0 {
                    target_defeated_event_writer.write(TargetDefeated(target_damaged.target));
                }
            }
        });
}

pub(crate) fn handle_target_defeated(
    mut target_defeated_event_reader: EventReader<TargetDefeated>,
    query: Query<NameOrEntity>,
    mut commands: Commands,
    mut focus_needed_event_writer: EventWriter<FocusNeeded>,
) {
    target_defeated_event_reader
        .read()
        .for_each(|target_defeated| {
            let TargetDefeated(target) = target_defeated;

            if let Ok(target) = query.get(*target) {
                print_with_prompt!("{target} has been defeated!");

                commands.entity(target.entity).despawn();
                focus_needed_event_writer.write_default();
            }
        });
}

pub(crate) fn trigger_enemy_turns(
    mut query: Query<(NameOrEntity, &mut Cooldown), With<Enemy>>,
    time: Res<Time>,
    player: Single<NameOrEntity, With<Player>>,
    mut action_used_event_writer: EventWriter<ActionUsed>,
) {
    query.iter_mut().for_each(|(enemy, cooldown)| {
        let Cooldown(cooldown_timer) = cooldown.into_inner();

        if cooldown_timer.tick(time.delta()).finished() {
            let action = Action::Attack;
            let actor = Some(enemy.entity);
            let target = Some(player.entity);

            action_used_event_writer.write(ActionUsed {
                action,
                actor,
                target,
            });
        }
    })
}
