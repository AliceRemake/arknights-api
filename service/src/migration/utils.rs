use ::error::Error;

use sea_orm::*;
use sea_orm_migration::prelude::*;
use sea_orm_migration::SchemaManager;

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

pub fn as_array(value: &serde_json::Value) -> Result<&Vec<serde_json::Value>, Error> {
    Ok(value
        .as_array()
        .ok_or(Error::RuntimeError(format!("can not parse {:?}", value)))?)
}

pub fn _as_bool(value: &serde_json::Value) -> Result<bool, Error> {
    Ok(value
        .as_bool()
        .ok_or(Error::RuntimeError(format!("can not parse {:?}", value)))?)
}

pub fn _as_f64(value: &serde_json::Value) -> Result<f64, Error> {
    Ok(value
        .as_f64()
        .ok_or(Error::RuntimeError(format!("can not parse {:?}", value)))?)
}

pub fn as_i64(value: &serde_json::Value) -> Result<i64, Error> {
    Ok(value
        .as_i64()
        .ok_or(Error::RuntimeError(format!("can not parse {:?}", value)))?)
}

pub fn _as_null(value: &serde_json::Value) -> Result<(), Error> {
    Ok(value
        .as_null()
        .ok_or(Error::RuntimeError(format!("can not parse {:?}", value)))?)
}

pub fn _as_number(value: &serde_json::Value) -> Result<&serde_json::Number, Error> {
    Ok(value
        .as_number()
        .ok_or(Error::RuntimeError(format!("can not parse {:?}", value)))?)
}

pub fn as_object(
    value: &serde_json::Value,
) -> Result<&serde_json::Map<String, serde_json::Value>, Error> {
    Ok(value
        .as_object()
        .ok_or(Error::RuntimeError(format!("can not parse {:?}", value)))?)
}

pub fn as_str(value: &serde_json::Value) -> Result<&str, Error> {
    Ok(value
        .as_str()
        .ok_or(Error::RuntimeError(format!("can not parse {:?}", value)))?)
}

pub fn _as_u64(value: &serde_json::Value) -> Result<u64, Error> {
    Ok(value
        .as_u64()
        .ok_or(Error::RuntimeError(format!("can not parse {:?}", value)))?)
}

pub fn extract_bool(
    table: &serde_json::Map<String, serde_json::Value>,
    key: &'static str,
) -> Result<(bool, bool), Error> {
    Ok((
        table[key].as_object().unwrap()["m_defined"]
            .as_bool()
            .ok_or(Error::RuntimeError(format!(
                "can not parse `{}` in {:?}",
                key, table
            )))?,
        table[key].as_object().unwrap()["m_value"]
            .as_bool()
            .ok_or(Error::RuntimeError(format!(
                "can not parse `{}` in {:?}",
                key, table
            )))?,
    ))
}

pub fn extract_i32(
    table: &serde_json::Map<String, serde_json::Value>,
    key: &'static str,
) -> Result<(bool, i32), Error> {
    Ok((
        table[key].as_object().unwrap()["m_defined"]
            .as_bool()
            .ok_or(Error::RuntimeError(format!(
                "can not parse `{}` in {:?}",
                key, table
            )))?,
        table[key].as_object().unwrap()["m_value"]
            .as_i64()
            .ok_or(Error::RuntimeError(format!(
                "can not parse `{}` in {:?}",
                key, table
            )))? as i32,
    ))
}

pub fn extract_f32(
    table: &serde_json::Map<String, serde_json::Value>,
    key: &'static str,
) -> Result<(bool, f32), Error> {
    Ok((
        table[key].as_object().unwrap()["m_defined"]
            .as_bool()
            .ok_or(Error::RuntimeError(format!(
                "can not parse `{}` in {:?}",
                key, table
            )))?,
        table[key].as_object().unwrap()["m_value"]
            .as_f64()
            .ok_or(Error::RuntimeError(format!(
                "can not parse `{}` in {:?}",
                key, table
            )))? as f32,
    ))
}

pub fn extract_str<'a>(
    table: &'a serde_json::Map<String, serde_json::Value>,
    key: &'static str,
) -> Result<(bool, &'a str), Error> {
    Ok((
        table[key].as_object().unwrap()["m_defined"]
            .as_bool()
            .ok_or(Error::RuntimeError(format!(
                "can not parse `{}` in {:?}",
                key, table
            )))?,
        table[key].as_object().unwrap()["m_value"]
            .as_str()
            .ok_or(Error::RuntimeError(format!(
                "can not parse `{}` in {:?}",
                key, table
            )))?,
    ))
}

pub fn extract_array<'a>(
    table: &'a serde_json::Map<String, serde_json::Value>,
    key: &'static str,
) -> Result<(bool, &'a Vec<JsonValue>), Error> {
    Ok((
        table[key].as_object().unwrap()["m_defined"]
            .as_bool()
            .ok_or(Error::RuntimeError(format!(
                "can not parse `{}` in {:?}",
                key, table
            )))?,
        table[key].as_object().unwrap()["m_value"]
            .as_array()
            .ok_or(Error::RuntimeError(format!(
                "can not parse `{}` in {:?}",
                key, table
            )))?,
    ))
}
