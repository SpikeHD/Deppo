use rand::Rng;

use super::state::State;

pub fn maybe_toggle_walk(state: &mut State) {
  // Don't try to move if we can't!
  if state.config.can_move.is_none() || !state.config.can_move.unwrap() {
    return;
  }

  // If the current state is neither walk nor idle, don't change it
  if !state.is_ground_state() {
    return;
  }

  let behaviour_change_rarity = state.config.behaviour_change_rarity.unwrap_or(1.);
  let change = rand::thread_rng()
    .gen_range(0.0..behaviour_change_rarity)
    .round()
    == 0.0;

  if !change {
    return;
  }

  let new_state = match state.move_state {
    super::state::MovementState::Idle => super::state::MovementState::Walk,
    super::state::MovementState::Walk => super::state::MovementState::Idle,

    // If we are falling or something, we don't want to change mid-air
    _ => state.move_state.clone(),
  };

  // If new state is Walk, set the horizontal velocity to the move speed, multiplied by a random direction
  if new_state == super::state::MovementState::Walk {
    let walk_direction = walk_direction_sign();
    state.set_velocity((state.config.move_speed.unwrap_or(0.) * walk_direction, 0.));

    // Change flip_x based on direction
    state.flip_x = walk_direction == -1.0;
  }

  // Otherwise if we are idle, make sure we aren't moving
  if new_state != super::state::MovementState::Walk {
    state.set_velocity((0., 0.));
  }

  state.handle_state_change(new_state);
}

pub fn walk_direction_sign() -> f32 {
  // small helper to return -1 or 1 randomly
  (rand::thread_rng().gen_range(0..2) * 2 - 1) as f32
}
