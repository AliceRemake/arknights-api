use core::fmt;

use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum Race {
    Unknow = 0,
    Infection = 1,
    Drone = 2,
    Sarkaz = 3,
    Mutant = 4,
    SeaMonster = 5,
    OriginIumartScarft = 6,
    Animated = 7,
    Machine = 8,
    WildAnimal = 9,
    Collapsal = 10,
}

impl From<&str> for Race {
    fn from(value: &str) -> Self {
        if value == "infection" {
            Race::Infection
        } else if value == "drone" {
            Race::Drone
        } else if value == "sarkaz" {
            Race::Sarkaz
        } else if value == "mutant" {
            Race::Mutant
        } else if value == "seamonster" {
            Race::SeaMonster
        } else if value == "originiumartscarft" {
            Race::OriginIumartScarft
        } else if value == "animated" {
            Race::Animated
        } else if value == "machine" {
            Race::Machine
        } else if value == "wildanimal" {
            Race::WildAnimal
        } else if value == "collapsal" {
            Race::Collapsal
        } else {
            Race::Unknow
        }
    }
}

impl From<&Race> for &'static str {
    fn from(value: &Race) -> Self {
        match value {
            Race::Unknow => "unknow",
            Race::Infection => "infection",
            Race::Drone => "drone",
            Race::Sarkaz => "sarkaz",
            Race::Mutant => "mutant",
            Race::SeaMonster => "seamonster",
            Race::OriginIumartScarft => "originiumartscarft",
            Race::Animated => "animated",
            Race::Machine => "machine",
            Race::WildAnimal => "wildanimal",
            Race::Collapsal => "collapsal",
        }
    }
}

impl From<Race> for String {
    fn from(value: Race) -> Self {
        String::from(<&Race as Into<&'static str>>::into(&value))
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&Race as Into<&'static str>>::into(self))
    }
}
