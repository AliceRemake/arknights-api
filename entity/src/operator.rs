use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

use crate::position::Position;
use crate::profession::Profession;
use crate::sub_profession::SubProfession;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "operator")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub icon: String,
    pub name: String,
    pub rarity: u8,
    pub profession: Profession,
    pub sub_profession: SubProfession,
    pub recruitable: bool,
    pub position: Position,
    pub tags: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}