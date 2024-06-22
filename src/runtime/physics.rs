use raylib::RaylibHandle;

use super::super::window::platform::desktop_size;
use super::state::State;

static MAX_VELOCITY: f32 = 30.0;

pub fn do_gravity(state: &mut State, rl: &mut RaylibHandle) {
  if state.velocity_frozen || state.config.can_fall.is_none() || !state.config.can_fall.unwrap() {
    return;
  }

  // Limit Y velocity
  if state.velocity.1.abs() > MAX_VELOCITY {
    state.velocity.1 = if state.velocity.1 > 0.0 {
      MAX_VELOCITY
    } else {
      -MAX_VELOCITY
    };
  } else {
    state.velocity.1 -= 1.0;
  }

  // If the windows is sitting past or on the bottom of the screen, don't move it
  if desktop_size().1 - rl.get_screen_height() as u32 <= rl.get_window_position().y as u32
    && state.velocity.1 <= 0.0
  {
    // Ensure we don't fall farther than the bottom of the screen
    state.velocity.1 = 0.0;

    // Also make sure our state is proper
    state.handle_state_change(super::state::MovementState::Idle);
  }

  if state.velocity.1 != 0.0 {
    state.handle_state_change(super::state::MovementState::Falling);
  } else if state.move_state == super::state::MovementState::Falling {
    state.handle_state_change(super::state::MovementState::Idle);
  }
}

pub fn do_horizontal_checks(state: &mut State, rl: &mut RaylibHandle) {
  if state.velocity_frozen {
    return;
  }

  // At the right edge of the screen
  if desktop_size().0 - rl.get_screen_width() as u32 <= rl.get_window_position().x as u32 {
    // Specifically set the position to be right at the bottom of the screen
    // Make sure this isn't 0 so we don't get stuck
    rl.set_window_position(
      (desktop_size().0 - rl.get_screen_width() as u32) as i32 - 1,
      rl.get_window_position().y as i32,
    );
    state.velocity.0 = 0.0;
    return;
  }

  // At the left edge of the screen
  if rl.get_window_position().x <= 0. {
    // Make not 0 so we don't get stuck
    rl.set_window_position(1, rl.get_window_position().y as i32);
    state.velocity.0 = 0.0;
  }
}

pub fn do_movement(state: &mut State, rl: &mut RaylibHandle) {
  if state.velocity_frozen {
    return;
  }

  let (x, y) = state.velocity;
  let new_x = rl.get_window_position().x - x;
  let mut new_y = rl.get_window_position().y - y;

  // If the windows is sitting past or on the bottom of the screen, move it back up
  if desktop_size().1 - rl.get_screen_height() as u32 <= rl.get_window_position().y as u32
    && state.velocity.1 <= 0.0
  {
    new_y = (desktop_size().1 - rl.get_screen_height() as u32) as f32;
  }

  rl.set_window_position(new_x as i32, new_y as i32);
}
