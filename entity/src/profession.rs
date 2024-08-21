use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "u8", db_type = "TinyUnsigned")]
pub enum Profession {
    UNKNOW = 0,
    PIONEER = 1,
    WARRIOR = 2,
    TANK = 3,
    SNIPER = 4,
    CASTER = 5,
    MEDIC = 6,
    SUPPORT = 7,
    SPECIAL = 8,
    TOKEN = 9,
    TRAP = 10,
}

impl From<&str> for Profession {
    fn from(value: &str) -> Self {
        if value == "PIONEER" {
            Profession::PIONEER
        } else if value == "WARRIOR" {
            Profession::WARRIOR
        } else if value == "TANK" {
            Profession::TANK
        } else if value == "SNIPER" {
            Profession::SNIPER
        } else if value == "CASTER" {
            Profession::CASTER
        } else if value == "MEDIC" {
            Profession::MEDIC
        } else if value == "SUPPORT" {
            Profession::SUPPORT
        } else if value == "SPECIAL" {
            Profession::SPECIAL
        } else if value == "TOKEN" {
            Profession::TOKEN
        } else if value == "TRAP" {
            Profession::TRAP
        } else {
            Profession::UNKNOW
        }
    }
}

impl From<&Profession> for &'static str {
    fn from(value: &Profession) -> Self {
        match value {
            Profession::UNKNOW => "UNKNOW",
            Profession::PIONEER => "PIONEER",
            Profession::WARRIOR => "WARRIOR",
            Profession::TANK => "TANK",
            Profession::SNIPER => "SNIPER",
            Profession::CASTER => "CASTER",
            Profession::MEDIC => "MEDIC",
            Profession::SUPPORT => "SUPPORT",
            Profession::SPECIAL => "SPECIAL",
            Profession::TOKEN => "TOKEN",
            Profession::TRAP => "TRAP",
        }
    }
}

impl fmt::Display for Profession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&Profession as Into<&'static str>>::into(self))
    }
}
