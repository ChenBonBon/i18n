use serde_json;
use serde_yaml;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::thread;
use std::time::Instant;
use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Locale {
    en_us: String,
    zh_cn: String,
}

pub fn main() {
    let file = read_file("i18n/i18n.yaml");

    let now1 = Instant::now();
    old_parse(&file);
    let time1 = now1.elapsed();
    let now2 = Instant::now();
    new_parse(&file);
    let time2 = now2.elapsed();

    println!("time1: {:?}\ntime2: {:?}", time1, time2);
}

fn old_parse(file: &str) {
    let deserialized_map: BTreeMap<String, Locale> = serde_yaml::from_str(file).unwrap();
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

fn new_parse(file: &str) {
    let json = parse(&file);

    for key in json.keys() {
        write_file(
            &format!("i18n/{}.json", key),
            serde_json::to_string(json.get(key).unwrap()).unwrap(),
        );
    }
}

fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    fs::read_to_string(path).unwrap()
}

fn write_file(file_path: &str, contents: String) {
    let path = Path::new(file_path);
    fs::write(path, contents).unwrap();
}

fn parse(file: &str) -> BTreeMap<&str, BTreeMap<std::string::String, std::string::String>> {
    let deserialized_map: BTreeMap<String, Locale> = serde_yaml::from_str(&file).unwrap();
    let keys: Vec<String> = deserialized_map.keys().cloned().collect();
    let map = Arc::new(RwLock::new(deserialized_map));
    let en_us_map = Arc::new(RwLock::new(BTreeMap::new()));
    let zh_cn_map = Arc::new(RwLock::new(BTreeMap::new()));
    let output = Arc::new(RwLock::new(BTreeMap::new()));
    let mut handles = vec![];

    for key in keys {
        let map_clone = Arc::clone(&map);
        let en_us_map_clone = Arc::clone(&en_us_map);
        let zh_cn_map_clone = Arc::clone(&zh_cn_map);
        let output_clone = Arc::clone(&output);
        let handle = thread::spawn(move || {
            let mut en_us_map = en_us_map_clone.write().unwrap();
            let mut zh_cn_map = zh_cn_map_clone.write().unwrap();
            let mut output = output_clone.write().unwrap();
            let map = map_clone.read().unwrap();
            let en_us = parse_en_us(map, key.clone());
            let map = map_clone.read().unwrap();
            let zh_cn = parse_zh_cn(map, key.clone());
            en_us_map.insert(key.clone(), en_us);
            zh_cn_map.insert(key.clone(), zh_cn);

            let en_us_result = en_us_map.clone();
            let zh_cn_result = zh_cn_map.clone();

            output.insert("en_us", en_us_result);
            output.insert("zh_cn", zh_cn_result);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = &*output.read().unwrap();
    result.clone()
}

fn parse_en_us(map: RwLockReadGuard<BTreeMap<String, Locale>>, key: String) -> String {
    map.get(&key).unwrap().en_us.clone()
}

fn parse_zh_cn(map: RwLockReadGuard<BTreeMap<String, Locale>>, key: String) -> String {
    map.get(&key).unwrap().zh_cn.clone()
}
