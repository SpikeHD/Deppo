use std::path::PathBuf;

use crate::log;

use super::{config::get_config, paths::deppo_path};

pub fn list_deppos() -> Vec<String> {
  let deppo_dir = deppo_path();
  let mut deppos = Vec::new();

  // List each dir OR <file>.deppo/<file>.zip
  for entry in deppo_dir.read_dir().expect("Failed to read deppo directory") {
    let entry = match entry {
      Ok(entry) => entry,
      Err(_) => {
        log!("Failed to read entry in deppo directory.");
        std::process::exit(1);
      },
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