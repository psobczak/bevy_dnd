use bevy::prelude::*;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<Level>()
            .register_type::<XP>()
            .register_type::<Initiative>();
    }
}

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct Health(pub i32);

#[derive(Component, Reflect)]
pub struct Level {
    pub level: u8,
    pub xp: u32,
}

impl Default for Level {
    fn default() -> Self {
        Self { level: 1, xp: 0 }
    }
}

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct XP(pub u32);

#[derive(Component, Deref, DerefMut, Default, Reflect)]
pub struct Initiative(pub i32);
