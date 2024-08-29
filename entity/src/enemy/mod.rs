pub mod attr;
pub mod dmg;
pub mod immune;
pub mod level;
pub mod race;

pub use attr::Attr;
pub use dmg::Dmg;
pub use immune::Immune;
pub use level::Level;
pub use race::Race;

use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct VecEnemyAttr(pub Vec<Attr>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct VecAttr(pub Vec<Attr>);

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "enmey")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub icon: String,
    pub name: String,
    pub level: Level,
    pub races: Vec<Race>,
    pub dmgs: Vec<Dmg>,
    pub attrs: VecAttr,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
