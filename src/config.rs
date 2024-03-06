use std::fs::{read_to_string, write};
use std::ops::DerefMut;
use std::path::PathBuf;
use std::{env::temp_dir, ops::Deref};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    pub data_dir: PathBuf,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            data_dir: temp_dir(),
        }
    }
}

const CONFIG_FILE: &str = "config.toml";

#[derive(Debug)]
pub struct Configuration {
    inner: ConfigFile,
}

impl Default for Configuration {
    fn default() -> Self {
        let inner = if let Ok(config_string) = read_to_string(CONFIG_FILE) {
            match toml::from_str(&config_string) {
                Ok(x) => x,
                Err(x) => panic!("Config file parsing: {:#?}", x),
            }
        } else {
            ConfigFile::default()
        };
        Self { inner }
    }
}

impl Drop for Configuration {
    fn drop(&mut self) {
        write(CONFIG_FILE, toml::to_string(&self.inner).unwrap()).unwrap();
    }
}

impl Deref for Configuration {
    type Target = ConfigFile;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Configuration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

lazy_static! {
    static ref CONFIG: RwLock<ConfigFile> = RwLock::default();
}

pub async fn get_config() -> ConfigFile {
    CONFIG.read().await.clone()
}
