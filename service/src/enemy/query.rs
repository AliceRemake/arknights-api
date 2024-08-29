use ::entity::*;
use ::error::Error;

use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<enemy::Model>, Error> {
        let response = Enemy::find_by_id(id.clone()).one(db).await?;
        Ok(response)
    }

    pub async fn find_by_name(
        db: &DatabaseConnection,
        name: String,
    ) -> Result<Vec<enemy::Model>, Error> {
        let response = Enemy::find()
            .filter(enemy::Column::Name.eq(name))
            .all(db)
            .await?;
        Ok(response)
    }

    pub async fn find_by_level(
        db: &DatabaseConnection,
        level: enemy::Level,
    ) -> Result<Vec<enemy::Model>, Error> {
        let response = Enemy::find()
            .filter(enemy::Column::Level.eq(level.clone()))
            .all(db)
            .await?;
        Ok(response)
    }

    pub async fn find_by_race(
        db: &DatabaseConnection,
        race: enemy::Race,
    ) -> Result<Vec<enemy::Model>, Error> {
        let response = Enemy::find()
            .filter(enemy::Column::Races.contains(race))
            .all(db)
            .await?;
        Ok(response)
    }

    pub async fn find_by_dmg(
        db: &DatabaseConnection,
        dmg: enemy::Dmg,
    ) -> Result<Vec<enemy::Model>, Error> {
        let response = Enemy::find()
            .filter(enemy::Column::Dmgs.contains(dmg))
            .all(db)
            .await?;
        Ok(response)
    }
}
