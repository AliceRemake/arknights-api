use sea_orm::prelude::*;
use serde::{Serialize, Deserialize};

use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum SubProfession {
    UNKNOW = 0,
    CHARGER = 1,
    PIONEER = 2,
    AGENT = 3,
    TACTICIAN = 4,
    BEARER = 5,
    SWORD = 6,
    CENTURION = 7,
    REAPER = 8,
    INSTRUCTOR = 9,
    FIGHTER = 10,
    FEARLESS = 11,
    ARTSFGHTER = 12,
    MUSHA = 13,
    LIBRATOR = 14,
    CRUSHER = 15,
    LORD = 16,
    HAMMER = 17,
    UNYIELD = 18,
    DUELIST = 19,
    SHOTPROTECTOR = 20,
    GUARDIAN = 21,
    FORTRESS = 22,
    PROTECTOR = 23,
    ARTSPROTECTOR = 24,
    BOMBARDER = 25,
    SIEGESNIPER = 26,
    REAPERRANGE = 27,
    AOESNIPER = 28,
    LONGRANGE = 29,
    FASTSHOT = 30,
    CLOSERANGE = 31,
    HUNTER = 32,
    LOOPSHOOTER = 33,
    CORECASTER = 34,
    SPLASHCASTER = 35,
    MYSTIC = 36,
    BLASTCASTER = 37,
    CHAIN = 38,
    PHALANX = 39,
    FUNNEL = 40,
    PRIMCASTER = 41,
    PHYSICIAN = 42,
    INCANTATIONMEDIC = 43,
    HEALER = 44,
    RINGHEALER = 45,
    WANDERMEDIC = 46,
    CHAINHEALER = 47,
    SLOWER = 48,
    UNDERMINER = 49,
    SUMMONER = 50,
    BARD = 51,
    CRAFTSMAN = 52,
    BLESSING = 53,
    RITUALIST = 54,
    STALKER = 55,
    DOLLKEEPER = 56,
    EXECUTOR = 57,
    GEEK = 58,
    PUSHER = 59,
    MERCHANT = 60,
    HOOKMASTER = 61,
    TRAPER = 62,
    ALCHEMIST = 63,
    NOTCHAR1 = 64,
    NOTCHAR2 = 65,
}

impl From<&str> for SubProfession {
    fn from(value: &str) -> Self {
        if value == "charger" {
            SubProfession::CHARGER
        } else if value == "pioneer" {
            SubProfession::PIONEER
        } else if value == "agent" {
            SubProfession::AGENT
        } else if value == "tactician" {
            SubProfession::TACTICIAN
        } else if value == "bearer" {
            SubProfession::BEARER
        } else if value == "sword" {
            SubProfession::SWORD
        } else if value == "centurion" {
            SubProfession::CENTURION
        } else if value == "reaper" {
            SubProfession::REAPER
        } else if value == "instructor" {
            SubProfession::INSTRUCTOR
        } else if value == "fighter" {
            SubProfession::FIGHTER
        } else if value == "fearless" {
            SubProfession::FEARLESS
        } else if value == "artsfghter" {
            SubProfession::ARTSFGHTER
        } else if value == "musha" {
            SubProfession::MUSHA
        } else if value == "librator" {
            SubProfession::LIBRATOR
        } else if value == "crusher" {
            SubProfession::CRUSHER
        } else if value == "lord" {
            SubProfession::LORD
        } else if value == "hammer" {
            SubProfession::HAMMER
        } else if value == "unyield" {
            SubProfession::UNYIELD
        } else if value == "duelist" {
            SubProfession::DUELIST
        } else if value == "shotprotector" {
            SubProfession::SHOTPROTECTOR
        } else if value == "guardian" {
            SubProfession::GUARDIAN
        } else if value == "fortress" {
            SubProfession::FORTRESS
        } else if value == "protector" {
            SubProfession::PROTECTOR
        } else if value == "artsprotector" {
            SubProfession::ARTSPROTECTOR
        } else if value == "bombarder" {
            SubProfession::BOMBARDER
        } else if value == "siegesniper" {
            SubProfession::SIEGESNIPER
        } else if value == "reaperrange" {
            SubProfession::REAPERRANGE
        } else if value == "aoesniper" {
            SubProfession::AOESNIPER
        } else if value == "longrange" {
            SubProfession::LONGRANGE
        } else if value == "fastshot" {
            SubProfession::FASTSHOT
        } else if value == "closerange" {
            SubProfession::CLOSERANGE
        } else if value == "hunter" {
            SubProfession::HUNTER
        } else if value == "loopshooter" {
            SubProfession::LOOPSHOOTER
        } else if value == "corecaster" {
            SubProfession::CORECASTER
        } else if value == "splashcaster" {
            SubProfession::SPLASHCASTER
        } else if value == "mystic" {
            SubProfession::MYSTIC
        } else if value == "blastcaster" {
            SubProfession::BLASTCASTER
        } else if value == "chain" {
            SubProfession::CHAIN
        } else if value == "phalanx" {
            SubProfession::PHALANX
        } else if value == "funnel" {
            SubProfession::FUNNEL
        } else if value == "primcaster" {
            SubProfession::PRIMCASTER
        } else if value == "physician" {
            SubProfession::PHYSICIAN
        } else if value == "incantationmedic" {
            SubProfession::INCANTATIONMEDIC
        } else if value == "healer" {
            SubProfession::HEALER
        } else if value == "ringhealer" {
            SubProfession::RINGHEALER
        } else if value == "wandermedic" {
            SubProfession::WANDERMEDIC
        } else if value == "chainhealer" {
            SubProfession::CHAINHEALER
        } else if value == "slower" {
            SubProfession::SLOWER
        } else if value == "underminer" {
            SubProfession::UNDERMINER
        } else if value == "summoner" {
            SubProfession::SUMMONER
        } else if value == "bard" {
            SubProfession::BARD
        } else if value == "craftsman" {
            SubProfession::CRAFTSMAN
        } else if value == "blessing" {
            SubProfession::BLESSING
        } else if value == "ritualist" {
            SubProfession::RITUALIST
        } else if value == "stalker" {
            SubProfession::STALKER
        } else if value == "dollkeeper" {
            SubProfession::DOLLKEEPER
        } else if value == "executor" {
            SubProfession::EXECUTOR
        } else if value == "geek" {
            SubProfession::GEEK
        } else if value == "pusher" {
            SubProfession::PUSHER
        } else if value == "merchant" {
            SubProfession::MERCHANT
        } else if value == "hookmaster" {
            SubProfession::HOOKMASTER
        } else if value == "traper" {
            SubProfession::TRAPER
        } else if value == "alchemist" {
            SubProfession::ALCHEMIST
        } else if value == "notchar1" {
            SubProfession::NOTCHAR1
        } else if value == "notchar2" {
            SubProfession::NOTCHAR2
        } else {
            SubProfession::UNKNOW
        }
    }
}

impl From<&SubProfession> for &'static str {
    fn from(value: &SubProfession) -> &'static str {
        match value {
            SubProfession::UNKNOW => "UNKNOW",
            SubProfession::CHARGER => "CHARGER",
            SubProfession::PIONEER => "PIONEER",
            SubProfession::AGENT => "AGENT",
            SubProfession::TACTICIAN => "TACTICIAN",
            SubProfession::BEARER => "BEARER",
            SubProfession::SWORD => "SWORD",
            SubProfession::CENTURION => "CENTURION",
            SubProfession::REAPER => "REAPER",
            SubProfession::INSTRUCTOR => "INSTRUCTOR",
            SubProfession::FIGHTER => "FIGHTER",
            SubProfession::FEARLESS => "FEARLESS",
            SubProfession::ARTSFGHTER => "ARTSFGHTER",
            SubProfession::MUSHA => "MUSHA",
            SubProfession::LIBRATOR => "LIBRATOR",
            SubProfession::CRUSHER => "CRUSHER",
            SubProfession::LORD => "LORD",
            SubProfession::HAMMER => "HAMMER",
            SubProfession::UNYIELD => "UNYIELD",
            SubProfession::DUELIST => "DUELIST",
            SubProfession::SHOTPROTECTOR => "SHOTPROTECTOR",
            SubProfession::GUARDIAN => "GUARDIAN",
            SubProfession::FORTRESS => "FORTRESS",
            SubProfession::PROTECTOR => "PROTECTOR",
            SubProfession::ARTSPROTECTOR => "ARTSPROTECTOR",
            SubProfession::BOMBARDER => "BOMBARDER",
            SubProfession::SIEGESNIPER => "SIEGESNIPER",
            SubProfession::REAPERRANGE => "REAPERRANGE",
            SubProfession::AOESNIPER => "AOESNIPER",
            SubProfession::LONGRANGE => "LONGRANGE",
            SubProfession::FASTSHOT => "FASTSHOT",
            SubProfession::CLOSERANGE => "CLOSERANGE",
            SubProfession::HUNTER => "HUNTER",
            SubProfession::LOOPSHOOTER => "LOOPSHOOTER",
            SubProfession::CORECASTER => "CORECASTER",
            SubProfession::SPLASHCASTER => "SPLASHCASTER",
            SubProfession::MYSTIC => "MYSTIC",
            SubProfession::BLASTCASTER => "BLASTCASTER",
            SubProfession::CHAIN => "CHAIN",
            SubProfession::PHALANX => "PHALANX",
            SubProfession::FUNNEL => "FUNNEL",
            SubProfession::PRIMCASTER => "PRIMCASTER",
            SubProfession::PHYSICIAN => "PHYSICIAN",
            SubProfession::INCANTATIONMEDIC => "INCANTATIONMEDIC",
            SubProfession::HEALER => "HEALER",
            SubProfession::RINGHEALER => "RINGHEALER",
            SubProfession::WANDERMEDIC => "WANDERMEDIC",
            SubProfession::CHAINHEALER => "CHAINHEALER",
            SubProfession::SLOWER => "SLOWER",
            SubProfession::UNDERMINER => "UNDERMINER",
            SubProfession::SUMMONER => "SUMMONER",
            SubProfession::BARD => "BARD",
            SubProfession::CRAFTSMAN => "CRAFTSMAN",
            SubProfession::BLESSING => "BLESSING",
            SubProfession::RITUALIST => "RITUALIST",
            SubProfession::STALKER => "STALKER",
            SubProfession::DOLLKEEPER => "DOLLKEEPER",
            SubProfession::EXECUTOR => "EXECUTOR",
            SubProfession::GEEK => "GEEK",
            SubProfession::PUSHER => "PUSHER",
            SubProfession::MERCHANT => "MERCHANT",
            SubProfession::HOOKMASTER => "HOOKMASTER",
            SubProfession::TRAPER => "TRAPER",
            SubProfession::ALCHEMIST => "ALCHEMIST",
            SubProfession::NOTCHAR1 => "NOTCHAR1",
            SubProfession::NOTCHAR2 => "NOTCHAR2",
        }
    }
}

impl fmt::Display for SubProfession  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&SubProfession as Into<&'static str>>::into(self))
    }
}
