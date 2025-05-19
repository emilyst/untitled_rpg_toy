pub(crate) mod prelude {
    pub(crate) use super::*;
}

use crate::prelude::*;

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
    commands.spawn((Player, Name::from("Heroine"), Strength(10)));
}

pub(crate) fn spawn_enemies(mut commands: Commands) {
    commands.spawn_batch([
        (Slime, Name::from("Slime 1"), Strength(1)),
        (Slime, Name::from("Slime 2"), Strength(1)),
        (Slime, Name::from("Slime 3"), Strength(1)),
        (Slime, Name::from("Slime 4"), Strength(1)),
        (Slime, Name::from("Slime 5"), Strength(1)),
    ]);
}

pub(crate) fn focus_next_enemy(
    query: Query<NameOrEntity, With<Enemy>>,
    focus: Option<Single<NameOrEntity, With<Focus>>>,
    mut commands: Commands,
) {
    // just focus first enemy for now
    let enemy = query.iter().min_by_key(|enemy| enemy.entity.index());

    match (enemy, focus) {
        (Some(enemy), Some(focus)) => {
            if enemy.entity != focus.entity {
                commands.entity(focus.entity).remove::<Focus>();
                commands.entity(enemy.entity).insert(Focus);
            }
        }
        (Some(enemy), None) => {
            commands.entity(enemy.entity).insert(Focus);
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
    mut input_received_event_writer: EventWriter<InputRead>,
) {
    input_received_event_writer.write(InputRead(input_receiver.0.recv().unwrap()));
}

pub(crate) fn handle_input_received(
    player: Single<NameOrEntity, With<Player>>,
    target: Single<NameOrEntity, With<Focus>>,
    mut input_received_event_reader: EventReader<InputRead>,
    mut action_used_event_writer: EventWriter<ActionUsed>,
) {
    input_received_event_reader.read().for_each(|input_received| {
        let InputRead(input) = input_received;

        let action = Action::from(input);
        let actor = Some(player.entity);
        let target = Some(target.entity);

        action_used_event_writer.write(ActionUsed { action, actor, target });
    })
}

pub(crate) fn handle_action_taken(
    query: Query<(NameOrEntity, &Strength)>,
    mut action_used_event_reader: EventReader<ActionUsed>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut damage_received_event_writer: EventWriter<TargetDamaged>,
) {
    action_used_event_reader.read().for_each(|action_used| match action_used {
        ActionUsed { action: Action::Attack, actor: Some(actor), target: Some(target) } => {
            if let (Ok((actor_name, actor_strength)), Ok((target_name, _))) =
                (query.get(*actor), query.get(*target))
            {
                println!("{actor_name} used Attack on {target_name}!");

                damage_received_event_writer
                    .write(TargetDamaged { target: target_name.entity, amount: actor_strength.0 });
            }
        }
        ActionUsed { action: Action::Defend, actor: Some(actor), .. } => {
            if let Ok((actor_name, actor_strength)) = query.get(*actor) {
                println!("{actor} used Defend!");
            }
        }
        ActionUsed { action: Action::Quit, .. } => {
            app_exit_event_writer.write_default();
            println!("Quitting!");
        }
        ActionUsed { action: Action::Help, .. } => {
            println!("Help!");
        }
        ActionUsed { action: Action::Unknown(input), .. } => {
            warn!(r#"Ignoring unrecognized input! ("{}")"#, input);
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

            println!("{target} has {} HP remaining!", health.0);

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
    mut target_defeated_event_reader: EventReader<TargetDefeated>,
) {
    target_defeated_event_reader.read().for_each(|target_defeated| {
        if let Ok(target) = query.get(target_defeated.target) {
            println!("{target} has been defeated!");
            commands.entity(target.entity).despawn();
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
