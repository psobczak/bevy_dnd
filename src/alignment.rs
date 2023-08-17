use std::str::FromStr;

use crate::DnDError;
use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Reflect)]
pub enum Alignement {
    /// Lawful good (LG) creatures can be counted on to do the right thing as expected by society.
    /// Gold dragons, paladins, and most dwarves are lawful good.
    LawfulGood,
    /// Neutral good (NG) folk do the best they can to help others according to their needs.
    /// Many celestials, some cloud giants, and most gnomes are neutral good.
    NeutralGood,
    /// Chaotic good (CG) creatures act as their conscience directs, with little regard for what others expect.
    ///  Copper dragons, many elves, and unicorns are chaotic good.
    ChaoticGood,
    /// Lawful neutral (LN) individuals act in accordance with law, tradition, or personal codes.
    /// Many monks and some wizards are lawful neutral.
    LawfulNeutral,
    /// Neutral (N) is the alignment of those who prefer to steer clear of moral questions and don’t take sides, doing what seems best at the time.
    /// Lizardfolk, most druids, and many humans are neutral.
    Neutral,
    /// Chaotic neutral (CN) creatures follow their whims, holding their personal freedom above all else.
    /// Many barbarians and rogues, and some bards, are chaotic neutral.
    ChaoticNeutral,
    /// Lawful evil (LE) creatures methodically take what they want, within the limits of a code of tradition, loyalty, or order.
    /// Devils, blue dragons, and hobgoblins are lawful evil.
    LawfulEvil,
    /// Neutral evil (NE) is the alignment of those who do whatever they can get away with, without compassion or qualms.
    /// Many drow, some cloud giants, and goblins are neutral evil.
    NeutralEvil,
    /// Chaotic evil (CE) creatures act with arbitrary violence, spurred by their greed, hatred, or bloodlust.
    /// Demons, red dragons, and orcs are chaotic evil.
    ChaoticEvil,
}

impl FromStr for Alignement {
    type Err = DnDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LG" => Ok(Self::LawfulGood),
            "NG" => Ok(Self::NeutralGood),
            "CG" => Ok(Self::ChaoticGood),
            "LN" => Ok(Self::LawfulNeutral),
            "N" => Ok(Self::Neutral),
            "CN" => Ok(Self::ChaoticNeutral),
            "LE" => Ok(Self::LawfulEvil),
            "NE" => Ok(Self::NeutralEvil),
            "CE" => Ok(Self::ChaoticEvil),
            _ => Err(DnDError::Alignement(s.into())),
        }
    }
}
