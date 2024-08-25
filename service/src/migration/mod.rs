pub mod operator;

use error::Error;

use sea_orm::DatabaseConnection;

pub async fn migrate(db: &DatabaseConnection) -> Result<(), Error> {
    operator::migrate(db).await?;
    Ok(())
}
