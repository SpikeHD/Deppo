use crate::{
  runtime::state::StateConfig,
  util::{
    config::Configuration,
    deppos::{list_deppos, load_deppo},
    paths::deppo_path,
  },
};

pub struct Controls {
  pub deppo_dropdown: bool,
}

pub struct State {
  pub config: Configuration,
  pub controls: Controls,
  pub deppo_list: Vec<StateConfig>,
}

impl State {
  pub fn new(config: Configuration) -> Self {
    Self {
      config,
      controls: Controls {
        deppo_dropdown: false,
      },
      deppo_list: list_deppos()
        .iter()
        .map(|n| {
          let deppo_path = deppo_path();
          load_deppo(&deppo_path.join(n))
        })
        .collect(),
    }
  }
}
