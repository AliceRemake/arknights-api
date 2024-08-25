use ::error::Error;
use ::entity::{operator, Operator, Position, Profession, SubProfession};

use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<operator::Model>, Error> {
        let response = Operator::find_by_id(id.clone()).one(db).await?;
        Ok(response)
    }

    pub async fn find_by_name(
        db: &DatabaseConnection,
        name: String,
    ) -> Result<Vec<operator::Model>, Error> {
        let response = Operator::find()
            .filter(operator::Column::Name.eq(name))
            .all(db)
            .await?;
        Ok(response)
    }

    pub async fn find_by_rarity(
        db: &DatabaseConnection,
        rarity: u8,
    ) -> Result<Vec<operator::Model>, Error> {
        let response = Operator::find()
            .filter(operator::Column::Rarity.eq(rarity.clone()))
            .all(db)
            .await?;
        Ok(response)
    }

    pub async fn find_by_profession(
        db: &DatabaseConnection,
        profession: Profession,
    ) -> Result<Vec<operator::Model>, Error> {
        let response = Operator::find()
            .filter(operator::Column::Profession.eq(profession))
            .all(db)
            .await?;
        Ok(response)
    }

    pub async fn find_by_sub_profession(
        db: &DatabaseConnection,
        sub_profession: SubProfession,
    ) -> Result<Vec<operator::Model>, Error> {
        let response = Operator::find()
            .filter(operator::Column::SubProfession.eq(sub_profession))
            .all(db)
            .await?;
        Ok(response)
    }

    pub async fn find_by_recruitable(
        db: &DatabaseConnection,
        recruitable: bool,
    ) -> Result<Vec<operator::Model>, Error> {
        let response = Operator::find()
            .filter(operator::Column::Recruitable.eq(recruitable))
            .all(db)
            .await?;
        Ok(response)
    }

    pub async fn find_by_position(
        db: &DatabaseConnection,
        position: Position,
    ) -> Result<Vec<operator::Model>, Error> {
        let response = Operator::find()
            .filter(operator::Column::Position.eq(position))
            .all(db)
            .await?;
        Ok(response)
    }

    pub async fn find_by_tag(
        db: &DatabaseConnection,
        tag: String,
    ) -> Result<Vec<operator::Model>, Error> {
        let response = Operator::find()
            .filter(operator::Column::Tags.contains(tag))
            .all(db)
            .await?;
        Ok(response)
    }
}
