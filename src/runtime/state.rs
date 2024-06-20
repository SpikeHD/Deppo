use std::path::PathBuf;
use serde::{Serialize, Deserialize};

use crate::animation::AnimationTexture2D;

#[derive(Serialize, Deserialize)]
pub struct AnimationList {
  pub idle: Option<Vec<String>>,
  pub walk: Option<Vec<String>>,
  pub drag: Option<Vec<String>>,
  pub click: Option<Vec<String>>,
}

pub struct AnimationListBuffer {
  pub idle: Option<Vec<AnimationTexture2D>>,
  pub walk: Option<Vec<AnimationTexture2D>>,
  pub drag: Option<Vec<AnimationTexture2D>>,
  pub click: Option<Vec<AnimationTexture2D>>,
}

#[derive(Serialize, Deserialize)]
pub struct StateConfig {
  pub name: String,
  pub fps: u32,
  pub scale: f32,
  
  pub can_move: Option<bool>,
  pub can_drag: Option<bool>,
  pub can_click: Option<bool>,
  pub can_face: Option<bool>,

  pub move_speed: Option<f32>,

  pub animations: AnimationList,
}

pub struct State {
  pub name: String,
  pub path: PathBuf,
  pub config: StateConfig,

  pub current_frame: u32,
  pub current_animation: String,

  pub velocity_frozen: bool,
  pub velocity: (f32, f32),
  pub position: (f32, f32),
}

pub fn load(path: PathBuf) -> State {
  let file = std::fs::read_to_string(&path).unwrap();
  let config: StateConfig = serde_json::from_str(&file).unwrap();

  State {
    name: path.file_name().unwrap().to_str().unwrap().to_string(),
    path: path.parent().unwrap().to_path_buf(),
    config,
    current_frame: 0,
    current_animation: "idle".to_string(),
    velocity_frozen: false,
    velocity: (0., 0.),
    position: (0., 0.),
  }
}

pub fn load_all_animations(rl: &mut raylib::prelude::RaylibHandle, thread: &raylib::prelude::RaylibThread, state: &State) -> AnimationListBuffer {
  AnimationListBuffer {
    idle: match &state.config.animations.idle {
      Some(paths) => Some(paths.iter().map(|path| crate::animation::raw_to_texture_2d(rl, thread, &crate::animation::load_gif(state.path.join(path)))).collect()),
      None => None,
    },
    walk: match &state.config.animations.walk {
      Some(paths) => Some(paths.iter().map(|path| crate::animation::raw_to_texture_2d(rl, thread, &crate::animation::load_gif(state.path.join(path)))).collect()),
      None => None,
    },
    drag: match &state.config.animations.drag {
      Some(paths) => Some(paths.iter().map(|path| crate::animation::raw_to_texture_2d(rl, thread, &crate::animation::load_gif(state.path.join(path)))).collect()),
      None => None,
    },
    click: match &state.config.animations.click {
      Some(paths) => Some(paths.iter().map(|path| crate::animation::raw_to_texture_2d(rl, thread, &crate::animation::load_gif(state.path.join(path)))).collect()),
      None => None,
    },
  }
}