pub mod position;
pub mod profession;
pub mod sub_profession;

pub use position::Position;
pub use profession::Profession;
pub use sub_profession::SubProfession;

use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "operator")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub icon: String,
    pub name: String,
    pub rarity: i16,
    pub profession: Profession,
    pub sub_profession: SubProfession,
    pub recruitable: bool,
    pub position: Position,
    pub tags: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
