use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use character::CharacterPlugin;
use combat::{CombatState, TurnPlugin};
use dice::DicePlugin;

mod character;
mod combat;
mod dice;

pub struct DndPlugin;

impl Plugin for DndPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WorldInspectorPlugin::new(),
            DicePlugin,
            TurnPlugin,
            CharacterPlugin,
        ))
        .add_state::<AppState>()
        .add_systems(Update, change_status);
    }
}

fn change_status(mut next_state: ResMut<NextState<CombatState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Right) {
        next_state.set(CombatState::RoundEnd)
    }

    if input.just_pressed(KeyCode::Left) {
        next_state.set(CombatState::RoundStart)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    Combat,
}

#[derive(Debug, thiserror::Error)]
pub enum DnDError {
    #[error("ability score must be between 1 and 30")]
    AbilityScore,
    #[error("unknown alignment {0}")]
    Alignement(String),
}
