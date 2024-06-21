use std::path::PathBuf;
use serde::{Serialize, Deserialize};

use crate::animation::AnimationTexture2D;

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

#[derive(Serialize, Deserialize)]
pub struct StateConfig {
  pub name: String,
  pub fps: u32,
  // used for - for example - running a 15fps animation at 30fps, but retaining the speed
  pub timescale: Option<f32>,
  pub scale: Option<f32>,
  pub behaviour_change_rarity: Option<f32>,
  
  pub can_move: Option<bool>,
  pub can_drag: Option<bool>,
  pub can_click: Option<bool>,
  pub can_fall: Option<bool>,

  pub move_speed: Option<f32>,

  pub animations: AnimationList,
}

pub struct State {
  pub name: String,
  pub path: PathBuf,
  pub config: StateConfig,

  pub move_state: MovementState,
  pub move_state_changed: bool,

  pub current_frame: u32,

  pub velocity_frozen: bool,
  pub velocity: (f32, f32),
  pub position: (f32, f32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum MovementState {
  Idle,
  Walk,
  Drag,
  Falling,
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

  pub fn set_position(&mut self, position: (f32, f32)) {
    self.position = position;
  }

  pub fn update(&mut self) {
    if !self.velocity_frozen {
      self.position.0 += self.velocity.0;
      self.position.1 += self.velocity.1;
    }
  }

  pub fn handle_state_change(&mut self, new_state: MovementState) {
    // TODO this wont work properly for now, we need to account for clicks properly
    match new_state {
      MovementState::Idle => {
        if self.move_state != MovementState::Idle {
          self.change_animation();
          self.move_state = MovementState::Idle;
        }
      },
      MovementState::Walk => {
        if self.move_state != MovementState::Walk {
          self.change_animation();
          self.move_state = MovementState::Walk;
        }
      },
      MovementState::Falling => {
        if self.move_state != MovementState::Falling {
          self.change_animation();
          self.move_state = MovementState::Falling;
        }
      },
      MovementState::Drag => {
        if self.move_state != MovementState::Drag {
          self.change_animation();
          self.move_state = MovementState::Drag;
        }
      },
      MovementState::Click => {
        if self.move_state != MovementState::Click {
          self.change_animation();
          self.move_state = MovementState::Click;
        }
      },
    }
  }
}

pub fn load(path: PathBuf) -> State {
  let file = std::fs::read_to_string(&path).unwrap();
  let config: StateConfig = serde_json::from_str(&file).unwrap();

  State {
    name: path.file_name().unwrap().to_str().unwrap().to_string(),
    path: path.parent().unwrap().to_path_buf(),
    move_state: MovementState::Idle,
    // This will ensure an animation is loaded right when we start
    move_state_changed: true,
    config,
    current_frame: 0,
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
    fall: match &state.config.animations.fall {
      Some(paths) => Some(paths.iter().map(|path| crate::animation::raw_to_texture_2d(rl, thread, &crate::animation::load_gif(state.path.join(path)))).collect()),
      None => None,
    },
    click: match &state.config.animations.click {
      Some(paths) => Some(paths.iter().map(|path| crate::animation::raw_to_texture_2d(rl, thread, &crate::animation::load_gif(state.path.join(path)))).collect()),
      None => None,
    },
  }
}