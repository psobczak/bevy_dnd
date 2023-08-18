use bevy::{
    prelude::{Component, Deref, DerefMut, Plugin},
    reflect::Reflect,
};

use crate::DnDError;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Abilities>()
            .register_type::<AbilityScore>();
    }
}

#[derive(Debug, Component, Reflect)]
/// Six abilities provide a quick description of every creature’s physical and mental characteristics
pub struct Abilities {
    /// Measuring physical power
    pub strength: AbilityScore,
    /// Measuring agility
    pub dexterity: AbilityScore,
    /// Measuring endurance
    pub constitution: AbilityScore,
    /// Measuring reasoning and memory
    pub intelligence: AbilityScore,
    /// Measuring perception and insight
    pub wisdom: AbilityScore,
    /// Measuring force of personality
    pub charisma: AbilityScore,
}

#[derive(Debug, Deref, DerefMut, Component, Reflect)]
pub struct AbilityScore(pub u8);

impl AbilityScore {
    pub fn get_modifier(&self) -> i32 {
        match self.0 {
            1 => -5,
            (2..=3) => -4,
            (4..=5) => -3,
            (6..=7) => -2,
            (8..=9) => -1,
            (10..=11) => 0,
            (12..=13) => 1,
            (14..=15) => 2,
            (16..=17) => 3,
            (18..=19) => 4,
            (20..=21) => 5,
            (22..=23) => 6,
            (24..=25) => 7,
            (26..=27) => 8,
            (28..=29) => 9,
            _ => 10,
        }
    }

    pub fn new(value: u8) -> Result<Self, DnDError> {
        Self::try_from(value)
    }
}

impl TryFrom<u8> for AbilityScore {
    type Error = DnDError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..30).contains(&value) {
            return Err(DnDError::AbilityScore);
        }

        Ok(AbilityScore(value))
    }
}
