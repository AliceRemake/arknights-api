use core::fmt;

use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum Dmg {
    Unknow = 0,
    Physic = 1,
    Magic = 2,
    Heal = 3,
    NoDamage = 4,
}

impl From<&str> for Dmg {
    fn from(value: &str) -> Self {
        if value == "PHYSIC" {
            Dmg::Physic
        } else if value == "MAGIC" {
            Dmg::Magic
        } else if value == "HEAL" {
            Dmg::Heal
        } else if value == "NO_DAMAGE" {
            Dmg::NoDamage
        } else {
            Dmg::Unknow
        }
    }
}

impl From<&Dmg> for &'static str {
    fn from(value: &Dmg) -> Self {
        match value {
            Dmg::Unknow => "UNKNOW",
            Dmg::Physic => "PHYSIC",
            Dmg::Magic => "MAGIC",
            Dmg::Heal => "HEAL",
            Dmg::NoDamage => "NODAMAGE",
        }
    }
}

impl From<Dmg> for String {
    fn from(value: Dmg) -> Self {
        String::from(<&Dmg as Into<&'static str>>::into(&value))
    }
}

impl fmt::Display for Dmg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&Dmg as Into<&'static str>>::into(self))
    }
}
