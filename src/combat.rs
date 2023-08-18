use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};

pub struct TurnPlugin;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Round {
    pub number: u32,
    pub order: Vec<(Entity, i32)>,
}

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Round>()
            .add_plugins(
                ResourceInspectorPlugin::<Round>::default()
                    .run_if(input_toggle_active(true, KeyCode::Slash)),
            )
            .add_state::<CombatState>()
            .add_systems(OnEnter(CombatState::RoundStart), increase_turn_number);
    }
}

fn increase_turn_number(mut turn: ResMut<Round>) {
    turn.number += 1;
    info!("Current round number: {}", turn.number);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum CombatState {
    #[default]
    RoundStart,
    RoundEnd,
}
