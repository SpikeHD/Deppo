use mouse_position::mouse_position::Mouse;
use raylib::prelude::*;

use super::{physics::DEFAULT_MAX_VELOCITY, state::State};

pub fn mouse_as_vec2() -> Vector2 {
  let position = Mouse::get_mouse_position();
  let mut current_mouse_pos = Vector2::new(0.0, 0.0);

  if let Mouse::Position { x, y } = position {
    current_mouse_pos.x = x as f32;
    current_mouse_pos.y = y as f32;
  }

  current_mouse_pos
}

pub fn handle_mouse(rl: &mut RaylibHandle, state: &mut State, width: i32, _height: i32) {
  if state.config.can_drag.is_none() || !state.config.can_drag.unwrap() {
    return;
  }

  let current_mouse_pos = mouse_as_vec2();

  if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
    if !state.velocity_frozen {
      state.velocity_frozen = true;
      state.velocity = (0.0, 0.0);
    }

    // Set state mouse positions
    state.mouse_position = (
      (current_mouse_pos.x, current_mouse_pos.y),
      state.mouse_position.0,
    );

    // Ignore if mouse pos is negative
    if current_mouse_pos.x > 0.0 || current_mouse_pos.y > 0.0 {
      // Move the window
      rl.set_window_position(
        current_mouse_pos.x as i32 - width / 2,
        current_mouse_pos.y as i32,
      );
      state.handle_state_change(super::state::MovementState::Drag);
    }
  } else if state.velocity_frozen {
    let max_vel = state
      .config
      .physics
      .max_velocity
      .unwrap_or(DEFAULT_MAX_VELOCITY);

    state.velocity_frozen = false;

    // Set the velocity to whatever the velocity is based on the current and last mouse position
    let velocity = (
      (state.mouse_position.0 .0 - current_mouse_pos.x).clamp(-max_vel, max_vel),
      (state.mouse_position.0 .1 - current_mouse_pos.y).clamp(-max_vel, max_vel),
    );

    state.set_velocity(velocity);

    // Set mouse positions to 0. We don't need to track this all the time!
    state.mouse_position = ((0.0, 0.0), (0.0, 0.0));
  }
}

pub fn maybe_open_menu(rl: &mut RaylibHandle) -> bool {
  rl.is_key_down(KeyboardKey::KEY_M)
}

pub fn maybe_should_quit(rl: &mut RaylibHandle) -> bool {
  rl.is_key_down(KeyboardKey::KEY_Q)
}