# arknights-api

base url: https://arknights-api.aliceremake.top

| Method | Url                              | ParamType | Param         | ParamExample             |
| ------ | -------------------------------- | --------- | ------------- | ------------------------ |
| GET    | /                                |           |               |                          |
| POST   | /update                          | JSON      | {"TOKEN": ""} | {"TOKEN": "your_tokon"}  |
| POST   | /migrate                         | JSON      | {"TOKEN": ""} | {"TOKEN": "your_tokon"}  |
| GET    | /enemy/{:name}                   | Path      | name          | 源石虫                   |
| GET    | /operator/{:name}                | Path      | name          | 阿米娅                   |
| GET    | /resource/avatar/{:icon}         | Path      | icon          | char_002_amiya           |
| GET    | /resource/building_skill/{:icon} | Path      | icon          | bskill_ctrl_aegir        |
| GET    | /resource/enemy/{:icon}          | Path      | icon          | enemy_1000_gopro         |
| GET    | /resource/item/{:icon}           | Path      | icon          | COIN_FURN                |
| GET    | /resource/item_rarity/{:icon}    | Path      | icon          | sprite_item_r1           |
| GET    | /resource/map/{:icon}            | Path      | icon          | a001_01                  |
| GET    | /resource/portrait/{:icon}       | Path      | icon          | char_002_amiya_1+        |
| GET    | /resource/skill/{:icon}          | Path      | icon          | skill_icon_skchr_absin_1 |
| GET    | /resource/skin/{:icon}           | Path      | icon          | char_002_amiya_1         |
