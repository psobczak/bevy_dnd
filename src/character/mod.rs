mod ability;
mod alignment;
mod skill;
mod stats;

use bevy::prelude::*;

use crate::{
    combat::{CombatState, Round},
    dice::{RollDiceEvent, RollResultsEvent},
};

use self::{
    ability::{Abilities, AbilityPlugin, AbilityScore},
    alignment::Alignement,
    stats::{Health, Initiative, Level, StatsPlugin},
};

#[derive(Debug, Component)]
pub struct Creature;

#[derive(Bundle)]
pub struct CharacterBundle {
    name: Name,
    abilities: Abilities,
    alignement: Alignement,
    health: Health,
    level: Level,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AbilityPlugin, StatsPlugin))
            .add_systems(Startup, spawn_monter)
            .add_systems(
                OnEnter(CombatState::RoundStart),
                (roll_for_initiative, assign_initiative, assign_round_order).chain(),
            );
    }
}

fn spawn_monter(mut commands: Commands) {
    commands.spawn((
        CharacterBundle {
            name: "Aboleth".into(),
            abilities: Abilities {
                strength: AbilityScore::new(21).unwrap(),
                dexterity: AbilityScore::new(9).unwrap(),
                constitution: AbilityScore::new(15).unwrap(),
                intelligence: AbilityScore::new(18).unwrap(),
                wisdom: AbilityScore::new(15).unwrap(),
                charisma: AbilityScore::new(18).unwrap(),
            },
            alignement: Alignement::LawfulEvil,
            health: Health(135),
            level: Level { level: 10, xp: 0 },
        },
        Creature,
    ));

    commands.spawn((
        CharacterBundle {
            name: "Centaru".into(),
            abilities: Abilities {
                strength: AbilityScore::new(18).unwrap(),
                dexterity: AbilityScore::new(14).unwrap(),
                constitution: AbilityScore::new(14).unwrap(),
                intelligence: AbilityScore::new(9).unwrap(),
                wisdom: AbilityScore::new(13).unwrap(),
                charisma: AbilityScore::new(11).unwrap(),
            },
            alignement: Alignement::NeutralGood,
            health: Health(45),
            level: Level { level: 2, xp: 0 },
        },
        Creature,
    ));
}

fn roll_for_initiative(
    creatures: Query<Entity, (With<Creature>, With<Abilities>)>,
    mut writer: EventWriter<RollDiceEvent>,
) {
    for entity in &creatures {
        writer.send(RollDiceEvent::Initiative(entity));
    }
}

fn assign_initiative(
    mut commands: Commands,
    mut creatures: Query<(&Abilities, With<Creature>)>,
    mut reader: EventReader<RollResultsEvent>,
) {
    for event in reader.into_iter() {
        let initiative_roll = event.1.first().unwrap();
        if let Ok((abilities, _)) = creatures.get_mut(event.0) {
            let modifier = abilities.dexterity.get_modifier();
            commands
                .entity(event.0)
                .insert(Initiative(*initiative_roll as i32 + modifier));
        }
    }
}

fn assign_round_order(creatures: Query<(Entity, &Initiative)>, mut round: ResMut<Round>) {
    let mut order = creatures
        .into_iter()
        .map(|(entity, initiative)| (entity, initiative.0))
        .collect::<Vec<_>>();

    order.sort_unstable_by(|(_, initiative_a), (_, initiative_b)| {
        initiative_b.partial_cmp(initiative_a).unwrap()
    });

    info!("{:?}", &order);

    round.order = order
}
