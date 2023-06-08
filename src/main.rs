use serde_json;
use serde_yaml;
use std::collections::BTreeMap;
use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Locale {
    en_us: String,
    zh_cn: String,
}

pub fn main() {
    let file = read_file("i18n/i18n.yaml");
    let deserialized_map: BTreeMap<String, Locale> = serde_yaml::from_str(&file).unwrap();
    let mut en_us_json = BTreeMap::new();
    let mut zh_cn_json = BTreeMap::new();

    for key in deserialized_map.keys() {
        let value = &deserialized_map[key];
        let en_us = &value.en_us;
        let zh_cn = &value.zh_cn;
        en_us_json.insert(key, en_us);
        zh_cn_json.insert(key, zh_cn);
    }

    println!("{:?}, {:?}", en_us_json, zh_cn_json);

    write_file(
        "i18n/en_us.json",
        serde_json::to_string(&en_us_json).unwrap(),
    );
    write_file(
        "i18n/zh_cn.json",
        serde_json::to_string(&zh_cn_json).unwrap(),
    );
}

fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    fs::read_to_string(path).unwrap()
}

fn write_file(file_path: &str, contents: String) {
    let path = Path::new(file_path);
    fs::write(path, contents).unwrap();
}
