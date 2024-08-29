mod utils;

mod enemy;
mod operator;

use ::error::Error;

use sea_orm::*;

pub async fn migrate(db: &DatabaseConnection) -> Result<(), Error> {
    operator::migrate(db).await?;
    enemy::migrate(db).await?;
    Ok(())
}
