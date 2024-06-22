use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use std::{fs::File, io::Read, path::PathBuf};

use crate::{animation::AnimationTexture2D, log};

#[derive(Serialize, Deserialize)]
pub struct AnimationList {
  pub idle: Option<Vec<String>>,
  pub walk: Option<Vec<String>>,
  pub drag: Option<Vec<String>>,
  pub fall: Option<Vec<String>>,
  pub click: Option<Vec<String>>,
}

pub struct AnimationListBuffer {
  pub idle: Option<Vec<AnimationTexture2D>>,
  pub walk: Option<Vec<AnimationTexture2D>>,
  pub drag: Option<Vec<AnimationTexture2D>>,
  pub fall: Option<Vec<AnimationTexture2D>>,
  pub click: Option<Vec<AnimationTexture2D>>,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Default)]
pub struct PhysicsConfig {
  #[serde_inline_default(Some(40.))]
  pub max_velocity: Option<f32>, // something like 30 or 40 makes sense
  #[serde_inline_default(Some(0.9))]
  pub friction: Option<f32>, // 0.9 is usually a fair number
}

#[serde_inline_default]
#[derive(Serialize, Deserialize)]
pub struct StateConfig {
  pub name: String,
  #[serde_inline_default(30)]
  pub fps: u32,
  // TODO used for - for example - running a 15fps animation at 30fps, but retaining the speed
  #[serde_inline_default(Some(1.))]
  pub timescale: Option<f32>,
  #[serde_inline_default(Some(1.))]
  pub scale: Option<f32>,
  #[serde_inline_default(Some(40.))]
  pub behaviour_change_rarity: Option<f32>,

  pub can_move: Option<bool>,
  pub can_drag: Option<bool>,
  pub can_click: Option<bool>,
  pub can_fall: Option<bool>,

  // TODO allow variablility in this.
  // eg. change to max_speed and also add a speed_can_be_variable flag
  // and slow down the animation based on speed
  pub move_speed: Option<f32>,

  #[serde_inline_default(PhysicsConfig::default())]
  pub physics: PhysicsConfig,

  pub animations: AnimationList,
}

pub struct State {
  pub name: String,
  pub path: PathBuf,
  pub config: StateConfig,

  pub move_state: MovementState,
  pub move_state_changed: bool,

  pub current_frame: u32,

  pub flip_x: bool,
  pub flip_y: bool,

  pub velocity_frozen: bool,
  pub velocity: (f32, f32),
  pub position: (f32, f32),

  // tuple is (current, last)
  pub mouse_position: ((f32, f32), (f32, f32)),
}

#[derive(Debug, PartialEq, Clone)]
pub enum MovementState {
  Idle,
  Walk,
  Drag,
  Falling,

  // TODO this will probably be used
  #[allow(dead_code)]
  Click,
}

impl State {
  pub fn change_animation(&mut self) {
    // TODO will probably be used for handling transitions and such
    self.move_state_changed = true;
    self.current_frame = 0;
  }

  pub fn set_velocity(&mut self, velocity: (f32, f32)) {
    self.velocity = velocity;
  }

  pub fn is_ground_state(&self) -> bool {
    self.move_state == MovementState::Idle || self.move_state == MovementState::Walk
  }

  // pub fn set_position(&mut self, position: (f32, f32)) {
  //   self.position = position;
  // }

  // TODO this should probably be used over updating directly in the physics functions
  // pub fn update(&mut self) {
  //   if !self.velocity_frozen {
  //     self.position.0 += self.velocity.0;
  //     self.position.1 += self.velocity.1;
  //   }
  // }

  pub fn handle_state_change(&mut self, new_state: MovementState) {
    // TODO this wont work properly for now, we need to account for clicks properly
    match new_state {
      MovementState::Idle => {
        if self.move_state != MovementState::Idle {
          self.change_animation();
          self.move_state = MovementState::Idle;
        }
      }
      MovementState::Walk => {
        if self.move_state != MovementState::Walk {
          self.change_animation();
          self.move_state = MovementState::Walk;
        }
      }
      MovementState::Falling => {
        if self.move_state != MovementState::Falling {
          self.change_animation();
          self.move_state = MovementState::Falling;
        }
      }
      MovementState::Drag => {
        if self.move_state != MovementState::Drag {
          self.change_animation();
          self.move_state = MovementState::Drag;
        }
      }
      MovementState::Click => {
        if self.move_state != MovementState::Click {
          self.change_animation();
          self.move_state = MovementState::Click;
        }
      }
    }
  }
}

pub fn load(path: PathBuf) -> State {
  if path.is_dir() {
    // look for deppo.json
    let mut json_path = path.clone();
    json_path.push("deppo.json");

    if json_path.exists() {
      load_from_file(json_path).unwrap()
    } else {
      log!("No deppo.json found in directory.");
      std::process::exit(1);
    }
  } else {
    // If the file is a zip, load from zip,otherwise load from file
    if path.extension().unwrap_or_default() == "zip" {
      load_from_zip(path).unwrap_or_else(|e| {
        log!("Failed to load zip: {}", e);
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

pub fn load_from_file(path: PathBuf) -> Result<State, std::io::Error> {
  let file = std::fs::read_to_string(&path)?;
  let config: StateConfig = serde_json::from_str(&file)?;

  Ok(State {
    name: config.name.clone(),
    path: path.parent().unwrap_or(&path).to_path_buf(),

    move_state: MovementState::Idle,
    move_state_changed: true,
    config,
    current_frame: 0,

    flip_x: false,
    flip_y: false,

    velocity_frozen: false,
    velocity: (0., 0.),
    position: (0., 0.),

    mouse_position: ((0., 0.), (0., 0.)),
  })
}

pub fn load_from_zip(zip: PathBuf) -> Result<State, std::io::Error> {
  let file = File::open(&zip)?;
  let mut archive = zip::ZipArchive::new(file)?;

  // Find deppo.json
  let mut config = String::new();

  match archive.by_name("deppo.json") {
    Ok(mut file) => {
      file.read_to_string(&mut config)?;
    }
    Err(_) => {
      return Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "deppo.json not found in zip",
      ));
    }
  };

  let config: StateConfig = serde_json::from_str(&config)?;

  Ok(State {
    name: config.name.clone(),
    path: zip,

    move_state: MovementState::Idle,
    move_state_changed: true,
    config,
    current_frame: 0,

    flip_x: false,
    flip_y: false,

    velocity_frozen: false,
    velocity: (0., 0.),
    position: (0., 0.),

    mouse_position: ((0., 0.), (0., 0.)),
  })
}

pub fn load_all_animations(
  rl: &mut raylib::prelude::RaylibHandle,
  thread: &raylib::prelude::RaylibThread,
  state: &State,
) -> AnimationListBuffer {
  AnimationListBuffer {
    idle: state.config.animations.idle.as_ref().map(|paths| {
      paths
        .iter()
        .map(|path| {
          crate::animation::raw_to_texture_2d(
            rl,
            thread,
            &crate::animation::load_gif(&state, path.clone()),
          )
        })
        .collect()
    }),
    walk: state.config.animations.walk.as_ref().map(|paths| {
      paths
        .iter()
        .map(|path| {
          crate::animation::raw_to_texture_2d(
            rl,
            thread,
            &crate::animation::load_gif(&state, path.clone()),
          )
        })
        .collect()
    }),
    drag: state.config.animations.drag.as_ref().map(|paths| {
      paths
        .iter()
        .map(|path| {
          crate::animation::raw_to_texture_2d(
            rl,
            thread,
            &crate::animation::load_gif(&state, path.clone()),
          )
        })
        .collect()
    }),
    fall: state.config.animations.fall.as_ref().map(|paths| {
      paths
        .iter()
        .map(|path| {
          crate::animation::raw_to_texture_2d(
            rl,
            thread,
            &crate::animation::load_gif(&state, path.clone()),
          )
        })
        .collect()
    }),
    click: state.config.animations.click.as_ref().map(|paths| {
      paths
        .iter()
        .map(|path| {
          crate::animation::raw_to_texture_2d(
            rl,
            thread,
            &crate::animation::load_gif(&state, path.clone()),
          )
        })
        .collect()
    }),
  }
}
