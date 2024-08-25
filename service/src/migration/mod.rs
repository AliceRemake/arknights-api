pub mod operator;

use ::error::Error;

use sea_orm::*;
use sea_orm_migration::prelude::*;
use sea_orm_migration::SchemaManager;

pub async fn migrate(db: &DatabaseConnection) -> Result<(), Error> {
    operator::migrate(db).await?;
    Ok(())
}

pub async fn drop_table<T>(db: &DatabaseConnection, table: T) -> Result<(), Error>
where
    T: IntoTableRef,
{
    let manager = SchemaManager::new(db);

    manager
        .drop_table(Table::drop().table(table).if_exists().to_owned())
        .await?;

    Ok(())
}

pub async fn create_table<E>(db: &DatabaseConnection, entity: E) -> Result<(), Error>
where
    E: EntityTrait,
{
    let manager = SchemaManager::new(db);
    let backend = db.get_database_backend();
    let schema = sea_orm::Schema::new(backend);

    manager
        .create_table(schema.create_table_from_entity(entity))
        .await?;

    Ok(())
}
