use ::entity::*;
use ::error::Error;

use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn insert(db: &DatabaseConnection, data: enemy::ActiveModel) -> Result<(), Error> {
        data.insert(db).await?;
        Ok(())
    }

    pub async fn update(db: &DatabaseConnection, data: enemy::ActiveModel) -> Result<(), Error> {
        data.update(db).await?;
        Ok(())
    }

    pub async fn save(db: &DatabaseConnection, data: enemy::ActiveModel) -> Result<(), Error> {
        data.save(db).await?;
        Ok(())
    }

    pub async fn insert_many(
        db: &DatabaseConnection,
        data: Vec<enemy::ActiveModel>,
    ) -> Result<(), Error> {
        Enemy::insert_many(data).exec(db).await?;
        Ok(())
    }

    pub async fn delete_all(db: &DatabaseConnection) -> Result<(), Error> {
        Enemy::delete_many().exec(db).await?;
        Ok(())
    }
}
