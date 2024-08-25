use error::Error;

use entity::{operator, Operator, Position, Profession, SubProfession};

use crate::resource::{get_local_resource_instance, HOME};

use serde_json;

use regex::Regex;
use sea_orm::{ActiveValue, ConnectionTrait, DatabaseConnection, EntityTrait};
use sea_orm_migration::prelude::Table;
use sea_orm_migration::SchemaManager;
use std::collections;

pub async fn migrate(db: &DatabaseConnection) -> Result<(), Error> {
    let local_resource = get_local_resource_instance();
    let character_table = std::fs::read_to_string(format!(
        "{}/{}/{}/{}",
        HOME.as_str(),
        local_resource.dist,
        local_resource.repo,
        "gamedata/excel/character_table.json",
    ))?;
    let payload: serde_json::Value = serde_json::from_str(&character_table)?;
    let payload = payload
        .as_object()
        .ok_or(Error::RuntimeError(format!("can not parse json")))?;

    let mut operators: Vec<operator::ActiveModel> = Vec::new();

    let recruitable_operators = recruitable_operators()?;

    for (key, value) in payload {
        let icon = String::from(key);
        let name = String::from(
            value["name"]
                .as_str()
                .ok_or(Error::RuntimeError(format!("can not parse json")))?,
        );
        let rarity = value["rarity"]
            .as_i64()
            .ok_or(Error::RuntimeError(format!("can not parse json")))? as i32;
        let profession = Profession::from(
            value["profession"]
                .as_str()
                .ok_or(Error::RuntimeError(format!("can not parse json")))?,
        );
        let sub_profession = SubProfession::from(
            value["subProfessionId"]
                .as_str()
                .ok_or(Error::RuntimeError(format!("can not parse json")))?,
        );
        let position = Position::from(
            value["position"]
                .as_str()
                .ok_or(Error::RuntimeError(format!("can not parse json")))?,
        );
        let mut tags: Vec<String> = Vec::new();
        for tag in value["tagList"]
            .as_array()
            .ok_or(Error::RuntimeError(format!("can not parse json")))?
        {
            tags.push(String::from(tag.as_str().unwrap()));
        }

        operators.push(operator::ActiveModel {
            recruitable: ActiveValue::Set(recruitable_operators.contains(&name)),
            id: ActiveValue::NotSet,
            icon: ActiveValue::Set(icon),
            name: ActiveValue::Set(name),
            rarity: ActiveValue::Set(rarity),
            profession: ActiveValue::Set(profession),
            sub_profession: ActiveValue::Set(sub_profession),
            position: ActiveValue::Set(position),
            tags: ActiveValue::Set(tags),
        });
    }

    let manager = SchemaManager::new(db);

    manager
        .drop_table(Table::drop().table(Operator).if_exists().to_owned())
        .await?;

    let backend = db.get_database_backend();
    let schema = sea_orm::Schema::new(backend);
    manager
        .create_table(schema.create_table_from_entity(Operator))
        .await?;

    Operator::insert_many(operators).exec(db).await?;

    Ok(())
}

fn recruitable_operators() -> Result<collections::HashSet<String>, error::Error> {
    let local_resource = get_local_resource_instance();
    let gacha_table = std::fs::read_to_string(format!(
        "{}/{}/{}/{}",
        HOME.as_str(),
        local_resource.dist,
        local_resource.repo,
        "gamedata/excel/gacha_table.json",
    ))?;

    let payload: serde_json::Value = serde_json::from_str(&gacha_table)?;
    let payload = payload
        .as_object()
        .ok_or(Error::RuntimeError(format!("can not parse json")))?;
    let payload = payload["recruitDetail"]
        .as_str()
        .ok_or(Error::RuntimeError(format!("can not parse json")))?;

    let mut operators: collections::HashSet<String> = collections::HashSet::new();

    let re = Regex::new(r"(★\\n| )(<@rc.eml>)?([\u4e00-\u9fa5a-zA-Z0-9-]+)(</>)?")?;
    for iter in re.captures_iter(payload) {
        operators.insert(String::from(iter.get(3).unwrap().as_str()));
    }

    Ok(operators)
}
