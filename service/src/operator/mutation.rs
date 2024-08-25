use error::Error;

use entity::{operator, Operator};

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};

pub struct Mutation;

impl Mutation {
    pub async fn insert(db: &DatabaseConnection, data: operator::ActiveModel) -> Result<(), Error> {
        data.insert(db).await?;
        Ok(())
    }

    pub async fn update(db: &DatabaseConnection, data: operator::ActiveModel) -> Result<(), Error> {
        data.update(db).await?;
        Ok(())
    }

    pub async fn save(db: &DatabaseConnection, data: operator::ActiveModel) -> Result<(), Error> {
        data.save(db).await?;
        Ok(())
    }

    pub async fn insert_many(
        db: &DatabaseConnection,
        data: Vec<operator::ActiveModel>,
    ) -> Result<(), Error> {
        Operator::insert_many(data).exec(db).await?;
        Ok(())
    }

    pub async fn delete_all(db: &DatabaseConnection) -> Result<(), Error> {
        Operator::delete_many().exec(db).await?;
        Ok(())
    }
}
