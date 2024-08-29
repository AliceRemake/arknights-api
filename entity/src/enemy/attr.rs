use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Attr {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub res: f32,
    pub edf: f32,
    pub adf: f32,
    pub mov: f32,
    pub rng: f32,
    pub atk_spd: f32,
    pub atk_time: f32,
    pub mass: i32,
    pub taunt: i32,
    pub immune: i16,
}
