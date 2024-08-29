use crate::migration::utils::*;
use crate::resource::*;

use crate::operator::*;

use ::entity::*;
use ::error::Error;

use sea_orm::*;
use sea_orm_migration::prelude::*;

pub async fn migrate(db: &DatabaseConnection) -> Result<(), Error> {
    let character_table = std::fs::read_to_string(format!(
        "{}/{}/{}/{}",
        HOME.as_str(),
        LOCAL_RESOURCE.dist,
        LOCAL_RESOURCE.repo,
        "gamedata/excel/character_table.json",
    ))?;
    let payload: serde_json::Value = serde_json::from_str(&character_table)?;
    let payload = as_object(&payload)?;

    let mut operators: Vec<operator::ActiveModel> = Vec::new();

    let recruitable_operators = recruitable_operators()?;

    for (key, value) in payload {
        let icon = String::from(key);
        let name = String::from(as_str(&value["name"])?);
        let rarity = as_i64(&value["rarity"])? as i16;
        let profession = operator::Profession::from(as_str(&value["profession"])?);
        let sub_profession = operator::SubProfession::from(as_str(&value["subProfessionId"])?);
        let position = operator::Position::from(as_str(&value["position"])?);
        let mut tags: Vec<String> = Vec::new();
        for tag in as_array(&value["tagList"])? {
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

    drop_table(db, Operator).await?;
    create_table(db, Operator).await?;
    Mutation::insert_many(db, operators).await?;

    Ok(())
}

use regex::Regex;
use serde_json;
use std::collections::HashSet;

fn recruitable_operators() -> Result<HashSet<String>, Error> {
    let gacha_table = std::fs::read_to_string(format!(
        "{}/{}/{}/{}",
        HOME.as_str(),
        LOCAL_RESOURCE.dist,
        LOCAL_RESOURCE.repo,
        "gamedata/excel/gacha_table.json",
    ))?;

    let payload: serde_json::Value = serde_json::from_str(&gacha_table)?;
    let payload = payload
        .as_object()
        .ok_or(Error::RuntimeError(format!("can not parse json")))?;
    let payload = payload["recruitDetail"]
        .as_str()
        .ok_or(Error::RuntimeError(format!("can not parse json")))?;

    let mut operators: HashSet<String> = HashSet::new();

    let re = Regex::new(r"(★\\n| )(<@rc.eml>)?([\u4e00-\u9fa5a-zA-Z0-9-]+)(</>)?")?;
    for iter in re.captures_iter(payload) {
        operators.insert(String::from(iter.get(3).unwrap().as_str()));
    }

    Ok(operators)
}
