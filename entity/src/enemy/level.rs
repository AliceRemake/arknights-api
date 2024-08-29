use core::fmt;

use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum Level {
    Unknow = 0,
    Normal = 1,
    Elite = 2,
    Boss = 3,
}

impl From<&str> for Level {
    fn from(value: &str) -> Self {
        if value == "NORMAL" {
            Level::Normal
        } else if value == "ELITE" {
            Level::Elite
        } else if value == "BOSS" {
            Level::Boss
        } else {
            Level::Unknow
        }
    }
}

impl From<&Level> for &'static str {
    fn from(value: &Level) -> Self {
        match value {
            Level::Unknow => "UNKNOW",
            Level::Normal => "NORMAL",
            Level::Elite => "ELITE",
            Level::Boss => "BOSS",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&Level as Into<&'static str>>::into(self))
    }
}
