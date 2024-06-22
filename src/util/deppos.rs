use std::{io::Read, path::PathBuf};

use crate::{log, runtime::state::StateConfig};

use super::{config::get_config, paths::deppo_path};

pub fn list_deppos() -> Vec<String> {
  let deppo_dir = deppo_path();
  let mut deppos = Vec::new();

  // List each dir OR <file>.deppo/<file>.zip
  for entry in deppo_dir
    .read_dir()
    .expect("Failed to read deppo directory")
  {
    let entry = match entry {
      Ok(entry) => entry,
      Err(_) => {
        log!("Failed to read entry in deppo directory.");
        std::process::exit(1);
      }
    };
    let path = entry.path();

    deppos.push(path.file_name().unwrap().to_str().unwrap().to_string());
  }

  deppos
}

pub fn get_current_deppo_file() -> PathBuf {
  let config = get_config();
  let deppo_dir = deppo_path();

  let deppo_file = deppo_dir.join(&config.deppo);

  // If it doesn't exist, just use the first deppo
  if !deppo_file.exists() || config.deppo.is_empty() {
    let deppos = list_deppos();
    let first_deppo = deppo_dir.join(&deppos[0]);

    log!("Deppo file not found. Using first deppo: {:?}", first_deppo);
    first_deppo
  } else {
    deppo_file
  }
}

pub fn load_from_file(path: &PathBuf) -> Result<StateConfig, std::io::Error> {
  let file = std::fs::read_to_string(path)?;
  let config: StateConfig = serde_json::from_str(&file)?;

  Ok(config)
}

pub fn load_from_zip(path: &PathBuf) -> Result<StateConfig, std::io::Error> {
  let file = std::fs::File::open(path)?;
  let mut archive = zip::ZipArchive::new(file)?;

  let mut config_file = archive.by_name("deppo.json")?;
  let mut config_str = String::new();
  config_file.read_to_string(&mut config_str)?;

  let config: StateConfig = serde_json::from_str(&config_str)?;

  Ok(config)
}

pub fn load_deppo(path: &PathBuf) -> StateConfig {
  if path.is_dir() {
    // look for deppo.json
    let mut json_path = path.clone();
    json_path.push("deppo.json");

    if json_path.exists() {
      load_from_file(&json_path).unwrap()
    } else {
      log!("No deppo.json found in directory.");
      std::process::exit(1);
    }
  } else {
    // If the file is a zip, load from zip,otherwise load from file
    if path.extension().unwrap_or_default() == "zip"
      || path.extension().unwrap_or_default() == "deppo"
    {
      load_from_zip(path).unwrap_or_else(|e| {
        log!("Failed to load archive: {}", e);
        std::process::exit(1);
      })
    } else {
      load_from_file(path).unwrap_or_else(|e| {
        log!("Failed to load file: {}", e);
        std::process::exit(1);
      })
    }
  }
}
