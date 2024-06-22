use crate::util::config::Configuration;

pub struct Controls {
  pub deppo_dropdown: bool
}

pub struct State {
  pub config: Configuration,
  pub controls: Controls,
}

impl State {
  pub fn new(config: Configuration) -> Self {
    Self {
      config,
      controls: Controls {
        deppo_dropdown: false
      }
    }
  }
}