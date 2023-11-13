use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf, sync::OnceLock};

use crate::tools::NoSyncWrap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub graphql: String,
    pub username: String,
    pub password: String,
    pub show_img_num: u16,
    pub filter_entities_my: Vec<FilterEntities>,
    #[serde(default)]
    pub filter_entities_more: Vec<FilterEntities>,
    #[serde(default)]
    pub filter_entities_stage: Vec<FilterEntities>,
    #[serde(default)]
    pub token: String,
}

impl Config {
    pub fn set_token(&mut self, token: String) {
        self.token = token
    }
    pub fn get_token(&self) -> &str {
        &self.token
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterEntities {
    pub filter_key: String,
    pub operation: u32,
    pub filter_value: String,
}

pub static CONFIG: OnceLock<NoSyncWrap<Config>> = OnceLock::new();
pub fn parse_config() -> Result<(), String> {
    let f = get_config_file()?;

    let config: Config = serde_yaml::from_reader(f)
        .map_err(|e| format!("serde_yaml::from_reader err: {}", e))?;

    CONFIG
        .set(NoSyncWrap::new(config))
        .map_err(|_| format!("OnceLock set err"))
}

pub fn get_config() -> &'static Config {
    CONFIG.get().unwrap().get().unwrap()
}

pub fn set_config_token(token: String) {
    let config = CONFIG.get().unwrap().get_mut().unwrap();
    config.set_token(token);
}

fn get_config_file() -> Result<File, String> {
    let work_dir = env::current_exe().unwrap();
    let dir = work_dir
        .parent()
        .ok_or(format!("Executable must be in some directory"))?;

    let config_path = dir.join("config.yaml");
    let default_config_path = "config.yaml"
        .parse::<PathBuf>()
        .map_err(|e| e.to_string())?;

    let path_arr = [config_path, default_config_path];

    let mut count = 0;
    loop {
        let item = File::open(&path_arr[count]).map_err(|e| e.to_string());
        if item.is_err() && count != path_arr.len() - 1 {
            count += 1;
            continue;
        }
        break item;
    }
}
