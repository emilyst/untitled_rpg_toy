pub(crate) mod prelude {
    pub(crate) use super::*;
}

use crate::prelude::*;

use crate::events::TargetDefeated;
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
    query_enemy: Query<Entity, With<Enemy>>,
    query_focus: Query<Entity, With<Focus>>,
    mut commands: Commands,
) {
    // just focus first enemy for now
    let enemy = query_enemy.iter().min();
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
    mut input_received_event_writer: EventWriter<InputRead>,
) {
    input_received_event_writer.write(InputRead(input_receiver.0.recv().unwrap()));
}

pub(crate) fn handle_input_received(
    query_player: Query<NameOrEntity, With<Player>>,
    query_target: Query<NameOrEntity, With<Focus>>,
    mut input_received_event_reader: EventReader<InputRead>,
    mut action_used_event_writer: EventWriter<ActionUsed>,
) {
    input_received_event_reader.read().for_each(|input_received| {
        let InputRead(input) = input_received;

        let action = Action::from(input);
        let actor = query_player.single().map(|t| t.entity).ok();
        let target = query_target.single().map(|t| t.entity).ok();

        action_used_event_writer.write(ActionUsed { action, actor, target });
    })
}

pub(crate) fn handle_action_taken(
    mut action_used_event_reader: EventReader<ActionUsed>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut damage_received_event_writer: EventWriter<TargetDamaged>,
    query_names_or_entities: Query<NameOrEntity>,
    query_strength: Query<&Strength>,
) {
    action_used_event_reader.read().for_each(|action_used| match action_used {
        ActionUsed { action: Action::Attack, actor: Some(actor), target: Some(target) } => {
            let actor_name = query_names_or_entities.get(*actor).unwrap();
            let target_name = query_names_or_entities.get(*target).unwrap();
            let strength = query_strength.get_inner(actor_name.entity).ok();

            println!("{actor_name} uses Attack on {target_name}!");

            damage_received_event_writer
                .write(TargetDamaged { amount: strength.unwrap().0, target: target_name.entity });
        }
        ActionUsed { action: Action::Defend, actor: Some(actor), .. } => {
            let actor = query_names_or_entities.get(*actor).unwrap();
            println!("{actor} uses Defend!")
        }
        ActionUsed { action: Action::Quit, .. } => {
            app_exit_event_writer.write_default();
            println!("Quitting!");
        }
        ActionUsed { action: Action::Help, .. } => {
            println!("Help!")
        }
        ActionUsed { action: Action::Unknown(input), .. } => {
            warn!(r#"Ignoring unrecognized input! ("{}")"#, input)
        }
        _ => {}
    });
}

pub(crate) fn handle_target_damaged(
    mut target_damaged_event_reader: EventReader<TargetDamaged>,
    mut target_defeated_event_writer: EventWriter<TargetDefeated>,
    query_names_or_entities: Query<NameOrEntity>,
    mut query_health: Query<&mut Health>,
) {
    target_damaged_event_reader.read().for_each(|target_damaged| {
        let target = target_damaged.target;
        let amount = target_damaged.amount;
        let target_name = query_names_or_entities.get(target).unwrap();

        println!("{target_name} damaged for {amount} HP!");

        if let Ok(mut health) = query_health.get_mut(target_damaged.target) {
            health.0 -= target_damaged.amount;
            let remaining = health.0;

            println!("{target_name} has {remaining} HP remaining!");

            if health.0 == 0 {
                target_defeated_event_writer.write(TargetDefeated { target });
            }
        }
    });
}

pub(crate) fn handle_target_defeated(
    mut target_defeated_event_reader: EventReader<TargetDefeated>,
    query_names_or_entities: Query<NameOrEntity>,
    mut commands: Commands,
) {
    target_defeated_event_reader.read().for_each(|target_defeated| {
        let target = target_defeated.target;
        let target_name = query_names_or_entities.get(target).unwrap();

        println!("{target_name} has been defeated!");

        commands.entity(target).despawn()
    });
}

pub(crate) fn trigger_enemy_actions(
    query_player: Query<NameOrEntity, With<Player>>,
    query_enemy: Query<NameOrEntity, With<Enemy>>,
    mut action_used_event_writer: EventWriter<ActionUsed>,
) {
    query_enemy.iter().for_each(|name_or_entity| {
        let action = Action::Attack;
        let actor = Some(name_or_entity.entity);
        let target = query_player.single().map(|t| t.entity).ok();

        action_used_event_writer.write(ActionUsed { action, actor, target });
    })
}
