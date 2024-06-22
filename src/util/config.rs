use serde::{Deserialize, Serialize};
use std::fs;

use crate::{log, util::paths::config_dir};

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration {
  pub deppo: String, // Current Deppo

  pub open_on_startup: bool,
  pub show_debug_info: bool,
}

pub fn read_config_file() -> String {
  let config_file = config_dir();

  fs::read_to_string(config_file).expect("Config does not exist!")
}

pub fn write_config_file(contents: String) {
  let config_file = config_dir();

  fs::write(config_file, contents).expect("Error writing config!")
}

pub fn default_config() -> Configuration {
  Configuration {
    deppo: "".to_string(),
    open_on_startup: false,
    show_debug_info: false,
  }
}

pub fn get_config() -> Configuration {
  let config_str = read_config_file();
  let config_str = config_str.as_str();

  match serde_json::from_str(config_str) {
    Ok(config) => config,
    Err(e) => {
      log!("Failed to parse config, using default config!");
      log!("Error: {}", e);

      default_config()
    }
  }
}