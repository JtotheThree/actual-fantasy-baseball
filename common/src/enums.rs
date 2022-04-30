use core::fmt;

use async_graphql::*;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString, EnumCount as EnumCountMacro};
use strum::EnumCount;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Copy, Clone, Eq, EnumIter, EnumCountMacro, PartialEq, Enum, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Gender {
    Male,
    Female,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
        }
    }
}

impl fmt::Debug for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Gender::Male => write!(f, "MALE"),
            Gender::Female => write!(f, "FEMALE"),
        }
    }
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        match rng.gen_range(0..=6) {
            0 | 1 | 2 | 3 => Gender::Male,
            _ => Gender::Female,
        }
    }
}

#[derive(Copy, Clone, Eq, EnumIter, PartialEq, Enum, EnumCountMacro, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Race {
    Dwarf,
    Elf,
    Goblin,
    Halfling,
    Human,
    Orc,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Race::Dwarf => write!(f, "Dwarf"),
            Race::Elf => write!(f, "Elf"),
            Race::Goblin => write!(f, "Goblin"),
            Race::Halfling => write!(f, "Hafling"),
            Race::Human => write!(f, "Human"),
            Race::Orc => write!(f, "Orc"),
        }
    }
}


impl fmt::Debug for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Race::Dwarf => write!(f, "DWARF"),
            Race::Elf => write!(f, "ELF"),
            Race::Goblin => write!(f, "GOBLIN"),
            Race::Halfling => write!(f, "HALFLING"),
            Race::Human => write!(f, "HUMAN"),
            Race::Orc => write!(f, "ORC"),
        }
    }
}

impl Distribution<Race> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Race {
        match rng.gen_range(0..Race::COUNT) {
            0 => Race::Dwarf,
            1 => Race::Elf,
            2 => Race::Goblin,
            3 => Race::Halfling,
            4 => Race::Human,
            5 => Race::Orc,
            _ => Race::Human,
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Enum, EnumCountMacro, EnumString, Serialize, Deserialize, EnumIter)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Class {
    Bard,
    Cleric,
    Fighter,
    Paladin,
    Ranger,
    Rogue,
    Wizard,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Class::Bard => write!(f, "Bard"),
            Class::Cleric => write!(f, "Cleric"),
            Class::Fighter => write!(f, "Fighter"),
            Class::Paladin => write!(f, "Paladin"),
            Class::Ranger => write!(f, "Ranger"),
            Class::Rogue => write!(f, "Rogue"),
            Class::Wizard => write!(f, "Wizard"),
        }
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Class::Bard => write!(f, "BARD"),
            Class::Cleric => write!(f, "CLERIC"),
            Class::Fighter => write!(f, "FIGHTER"),
            Class::Paladin => write!(f, "PALADIN"),
            Class::Ranger => write!(f, "RANGER"),
            Class::Rogue => write!(f, "ROGUE"),
            Class::Wizard => write!(f, "WIZARD"),
        }
    }
}

impl Distribution<Class> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Class {
        match rng.gen_range(0..Class::COUNT) {
            0 => Class::Bard,
            1 => Class::Cleric,
            2 => Class::Fighter,
            3 => Class::Paladin,
            4 => Class::Ranger,
            5 => Class::Rogue,
            6 => Class::Wizard,
            _ => Class::Fighter,
        }
    }
}


#[derive(Copy, Clone, Debug, Eq, EnumIter, PartialEq, Enum, EnumCountMacro, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Trait {
    HotTemper,
    Lucky,
    Boring,
    SuckUp,
    Quick,
    Greedy,
    Cleptomaniac,
    Tough,
    Clumsy,
    SureShot,
    LightningArm,
    Dirty,
    Lazy,
    Fat,
    Belligerent,
    QuickWitted,
    Goon,
    Timid,
}

/*impl fmt::Debug for Trait {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Trait::HotTemper => write!(f, "HOT_TEMPER"),
            Trait::Lucky => write!(f, "LUCKY"),
            Trait::Boring => write!(f, "BORING"),
        }
    }
}*/

impl Distribution<Trait> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Trait {
        match rng.gen_range(0..Trait::COUNT) {
            0 => Trait::HotTemper,
            1 => Trait::Lucky,
            2 => Trait::Boring,
            3 => Trait::SuckUp,
            4 => Trait::Quick,
            5 => Trait::Greedy,
            6 => Trait::Cleptomaniac,
            7 => Trait::Tough,
            8 => Trait::Clumsy,
            9 => Trait::SureShot,
            10 => Trait::LightningArm,
            11 => Trait::Dirty,
            12 => Trait::Lazy,
            13 => Trait::Fat,
            14 => Trait::Belligerent,
            15 => Trait::QuickWitted,
            16 => Trait::Goon,
            17 => Trait::Timid,
            _ => Trait::Boring,
        }
    }
}
