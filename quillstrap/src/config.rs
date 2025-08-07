use crate::prelude::*;

use serde::{Deserialize, Serialize};

use crate::prelude::read_file_str;
use std::{io::ErrorKind::*, path::Path};

const CONFIG_PATH: &str = "../qstrap.ron";
const CONFIG_PATH_FRESH: &str = "../qstrap_fresh.ron";

#[derive(Default, Serialize, Deserialize, PartialEq, Clone)]
pub enum GitLinkType {
    Ssh,
    #[default]
    Https,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Clone)]
pub enum GitPlatform {
    // Gitlab is untested!
    Gitlab,
    #[default]
    Github,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Config {
    pub git_link_type: GitLinkType,
    pub git_platform: GitPlatform,
    pub git_username: String,
    // Show underlaying command output too
    pub command_output: bool,
    pub main_private_key_path: String // Relative to root of the quillstrap repo!
}

impl Default for Config {
    fn default() -> Self {
        Self {
            git_link_type: Default::default(),
            git_platform: Default::default(),
            git_username: "PorQ-Pine".to_string(),
            command_output: true,
            main_private_key_path: String::from("other/private/private.pem"),
        }
    }
}

impl Config {
    pub fn save(&self, path: String) {
        // Unwrap, as I doubt it will ever fail, we also have color-eyre for better unwrap
        let str = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap();
        if let Err(err) = std::fs::write(&path, str) {
            error!("Failed to save config, this is bad!: {:?}", err);
        }
    }

    pub fn load() -> Self {
        match read_file_str(CONFIG_PATH.to_string()) {
            Ok(str) => match ron::from_str::<Config>(&str) {
                Ok(conf) => {
                    if Path::new(CONFIG_PATH_FRESH).exists() {
                        warn!("Config file is good but fresh exists, I will remove fresh");
                        let _ = remove_file(CONFIG_PATH_FRESH.to_string());
                    }
                    return conf;},
                Err(err) => {
                    Config::default().save(CONFIG_PATH_FRESH.to_string());
                    // TODO this in the future to insert them automatically, thats stupid as hell
                    error!("For config possible value options, look into quillstrap/src/config.rs");
                    panic!(
                        "Failed to load config file, it's probably outdated, new variables were added. Creating a fresh config file at {}, you need to repair the config at {} manually. This is the error: {:?}",
                        CONFIG_PATH_FRESH, CONFIG_PATH, err
                    );
                }
            },
            Err(err) => match err.kind() {
                NotFound => {
                    warn!("File not found, creating default config and continuing with it");
                    let conf = Config::default();
                    conf.save(CONFIG_PATH.to_string());
                    return conf;
                }
                _ => {
                    panic!(
                        "Something is wrong with accessing the config file, this is not recoverable."
                    );
                }
            },
        }
    }
}
