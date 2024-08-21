use sea_orm::prelude::*;
use serde::{Serialize, Deserialize};

use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "u8", db_type = "TinyUnsigned")]
pub enum Position {
    UNKNOW = 0,
    MELEE = 1,
    RANGED = 2,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        if value == "MELEE" {
            Position::MELEE
        } else if value == "RANGED" {
            Position::RANGED
        } else {
            Position::UNKNOW
        }
    }
}

impl From<&Position> for &'static str {
    fn from(value: &Position) -> Self {
        match value {
            Position::UNKNOW => "UNKNOW",
            Position::MELEE => "MELEE",
            Position::RANGED => "RANGED",
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&Position as Into<&'static str>>::into(self))
    }
}
