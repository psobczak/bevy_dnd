use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use ability::{Abilities, AbilityPlugin, AbilityScore};
use alignment::Alignement;
use dice::{DicePlugin, RollDiceEvent, RollResultsEvent};
use stats::{Health, Initiative, Level, StatsPlugin};

mod ability;
mod alignment;
mod dice;
mod skill;
mod stats;

pub struct DndPlugin;

impl Plugin for DndPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DicePlugin,
            WorldInspectorPlugin::new(),
            AbilityPlugin,
            StatsPlugin,
        ))
        .add_systems(Startup, spawn_monter)
        .add_systems(
            Update,
            (
                roll_for_initiative.run_if(input_just_pressed(KeyCode::I)),
                assign_initiative,
            ),
        );
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    name: Name,
    abilities: Abilities,
    alignement: Alignement,
    health: Health,
    level: Level,
}

#[derive(Debug, thiserror::Error)]
pub enum DnDError {
    #[error("ability score must be between 1 and 30")]
    AbilityScore,
    #[error("unknown alignment {0}")]
    Alignement(String),
}

#[derive(Debug, Component)]
pub struct Creature;

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
    for event in reader.iter() {
        if let Some(initiative_roll) = event.1.first() {
            if let Ok((abilities, _)) = creatures.get_mut(event.0) {
                let modifier = abilities.dexterity.get_modifier();
                commands
                    .entity(event.0)
                    .insert(Initiative(*initiative_roll as i32 + modifier));
            }
        }
    }
}
