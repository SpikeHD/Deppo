use raylib::ffi::TraceLogLevel;

use crate::util::config::{get_config, save_config, write_config_file};

mod color;
mod display;
mod runtime;
mod styles;

pub fn run() {
  let config = get_config();
  let mut state = runtime::state::State::new(config);

  let (mut rl, thread) = raylib::init()
    .title("Deppo Settings")
    .vsync()
    .log_level(TraceLogLevel::LOG_NONE)
    .resizable()
    .build();

  // No need to run fast
  rl.set_target_fps(30);

  while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);

    display::draw_gui(&mut state, &mut d);
  }

  // Save the config after menu is closed
  save_config(state.config);
}