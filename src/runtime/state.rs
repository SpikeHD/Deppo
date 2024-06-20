pub struct StateConfig {
  fps: u32,
  scale: f32,
}

pub struct State {
  pub name: String,
  pub config: StateConfig,
  pub velocity_frozen: bool,
  pub velocity: (f32, f32),
}
