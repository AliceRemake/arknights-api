use crate::migration::utils::*;
use crate::resource::*;

use crate::enemy::*;

use ::entity::*;
use ::error::Error;

use sea_orm::*;
use sea_orm_migration::prelude::*;

pub async fn migrate(db: &DatabaseConnection) -> Result<(), Error> {
    let enemy_handbook_table = std::fs::read_to_string(format!(
        "{}/{}/{}/{}",
        HOME.as_str(),
        LOCAL_RESOURCE.dist,
        LOCAL_RESOURCE.repo,
        "gamedata/excel/enemy_handbook_table.json",
    ))?;
    let enemy_handbook_table_payload: serde_json::Value =
        serde_json::from_str(&enemy_handbook_table)?;
    let enemy_data = as_object(&as_object(&enemy_handbook_table_payload)?["enemyData"])?;

    let enemy_database = std::fs::read_to_string(format!(
        "{}/{}/{}/{}",
        HOME.as_str(),
        LOCAL_RESOURCE.dist,
        LOCAL_RESOURCE.repo,
        "gamedata/levels/enemydata/enemy_database.json",
    ))?;
    let enemy_database_payload: serde_json::Value = serde_json::from_str(&enemy_database)?;

    let mut enemies: Vec<enemy::ActiveModel> = Vec::new();

    for each in as_array(&as_object(&enemy_database_payload)?["enemies"])? {
        let mut name: String = String::from("");
        let mut races: Vec<enemy::Race> = Vec::new();
        let mut attrs: Vec<enemy::Attr> = Vec::new();
        let key = as_str(&as_object(each)?["key"])?;
        for (index, each) in as_array(&as_object(each)?["value"])?.iter().enumerate() {
            let level = as_i64(&as_object(each)?["level"])?;
            let enemy_data = as_object(&as_object(each)?["enemyData"])?;

            if level == 0 {
                name = String::from(extract_str(enemy_data, "name")?.1);
                for each in extract_array(enemy_data, "enemyTags")?.1 {
                    races.push(enemy::Race::from(each.as_str().unwrap()));
                }
            }

            let attributes = as_object(&enemy_data["attributes"])?;
            let hp = extract_i32(attributes, "maxHp")?;
            let atk = extract_i32(attributes, "atk")?;
            let def = extract_i32(attributes, "def")?;
            let res = extract_f32(attributes, "magicResistance")?;
            let edf = extract_f32(attributes, "epDamageResistance")?;
            let adf = extract_f32(attributes, "epResistance")?;
            let mov = extract_f32(attributes, "moveSpeed")?;
            let rng = extract_f32(enemy_data, "rangeRadius")?;
            let atk_spd = extract_f32(attributes, "attackSpeed")?;
            let atk_time = extract_f32(attributes, "baseAttackTime")?;
            let mass = extract_i32(attributes, "massLevel")?;
            let taunt = extract_i32(attributes, "tauntLevel")?;
            let stun_immune = extract_bool(attributes, "stunImmune")?;
            let silence_immune = extract_bool(attributes, "silenceImmune")?;
            let sleep_immune = extract_bool(attributes, "sleepImmune")?;
            let frozen_immune = extract_bool(attributes, "frozenImmune")?;
            let levitate_immune = extract_bool(attributes, "levitateImmune")?;
            let disarmed_immune = extract_bool(attributes, "disarmedCombatImmune")?;
            let feared_immune = extract_bool(attributes, "fearedImmune")?;
            let immune: i16 = enemy::Immune {
                stun: if stun_immune.0 {
                    stun_immune.1
                } else if index > 0 {
                    enemy::Immune::from(attrs[index - 1].immune).stun
                } else {
                    false
                },
                silence: if silence_immune.0 {
                    silence_immune.1
                } else if index > 0 {
                    enemy::Immune::from(attrs[index - 1].immune).silence
                } else {
                    false
                },
                sleep: if sleep_immune.0 {
                    sleep_immune.1
                } else if index > 0 {
                    enemy::Immune::from(attrs[index - 1].immune).sleep
                } else {
                    false
                },
                frozen: if frozen_immune.0 {
                    frozen_immune.1
                } else if index > 0 {
                    enemy::Immune::from(attrs[index - 1].immune).frozen
                } else {
                    false
                },
                levitate: if levitate_immune.0 {
                    levitate_immune.1
                } else if index > 0 {
                    enemy::Immune::from(attrs[index - 1].immune).levitate
                } else {
                    false
                },
                disarmed: if disarmed_immune.0 {
                    disarmed_immune.1
                } else if index > 0 {
                    enemy::Immune::from(attrs[index - 1].immune).disarmed
                } else {
                    false
                },
                feared: if feared_immune.0 {
                    feared_immune.1
                } else if index > 0 {
                    enemy::Immune::from(attrs[index - 1].immune).feared
                } else {
                    false
                },
            }
            .into();
            attrs.push(enemy::Attr {
                hp: if hp.0 {
                    hp.1
                } else if index > 0 {
                    attrs[index - 1].hp
                } else {
                    0
                },
                atk: if atk.0 {
                    atk.1
                } else if index > 0 {
                    attrs[index - 1].atk
                } else {
                    0
                },
                def: if def.0 {
                    def.1
                } else if index > 0 {
                    attrs[index - 1].def
                } else {
                    0
                },
                res: if res.0 {
                    res.1
                } else if index > 0 {
                    attrs[index - 1].res
                } else {
                    0.0
                },
                edf: if edf.0 {
                    edf.1
                } else if index > 0 {
                    attrs[index - 1].edf
                } else {
                    0.0
                },
                adf: if adf.0 {
                    adf.1
                } else if index > 0 {
                    attrs[index - 1].adf
                } else {
                    0.0
                },
                mov: if mov.0 {
                    mov.1
                } else if index > 0 {
                    attrs[index - 1].mov
                } else {
                    0.0
                },
                rng: if rng.0 {
                    rng.1
                } else if index > 0 {
                    attrs[index - 1].rng
                } else {
                    0.0
                },
                atk_spd: if atk_spd.0 {
                    atk_spd.1
                } else if index > 0 {
                    attrs[index - 1].atk_spd
                } else {
                    0.0
                },
                atk_time: if atk_time.0 {
                    atk_time.1
                } else if index > 0 {
                    attrs[index - 1].atk_time
                } else {
                    0.0
                },
                mass: if mass.0 {
                    mass.1
                } else if index > 0 {
                    attrs[index - 1].mass
                } else {
                    0
                },
                taunt: if taunt.0 {
                    taunt.1
                } else if index > 0 {
                    attrs[index - 1].taunt
                } else {
                    0
                },
                immune: immune,
            });
        }

        let icon = String::from(key);
        let (level, dmgs) = match enemy_data.get(key) {
            Some(data) => {
                let level = enemy::Level::from(as_str(&as_object(data)?["enemyLevel"])?);
                let mut dmgs: Vec<enemy::Dmg> = Vec::new();
                for each in as_array(&as_object(data)?["damageType"])? {
                    dmgs.push(enemy::Dmg::from(as_str(each)?));
                }
                (level, dmgs)
            }
            None => {
                let level = enemy::Level::Unknow;
                let dmgs: Vec<enemy::Dmg> = Vec::new();
                (level, dmgs)
            }
        };

        enemies.push(enemy::ActiveModel {
            id: ActiveValue::NotSet,
            icon: ActiveValue::Set(icon),
            name: ActiveValue::Set(name),
            level: ActiveValue::Set(level),
            races: ActiveValue::Set(races),
            dmgs: ActiveValue::Set(dmgs),
            attrs: ActiveValue::Set(enemy::VecAttr(attrs)),
        });
    }

    drop_table(db, Enemy).await?;
    create_table(db, Enemy).await?;
    Mutation::insert_many(db, enemies).await?;

    Ok(())
}
