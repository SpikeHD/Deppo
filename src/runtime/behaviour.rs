use rand::Rng;

use super::state::State;

pub fn maybe_toggle_walk(state: &mut State) {
  // Don't try to move if we can't!
  if state.config.can_move.is_none() || !state.config.can_move.unwrap() {
    return;
  }

  let behaviour_change_rarity = state.config.behaviour_change_rarity.unwrap_or(1.);
  let change = rand::thread_rng().gen_range(0.0..behaviour_change_rarity).round() == 0.0;

  println!("Change: {}", change);

  if !change {
    return;
  }

  println!("Current state: {:?}", state.move_state);

  let new_state = match state.move_state {
    super::state::MovementState::Idle => super::state::MovementState::Walk,
    super::state::MovementState::Walk => super::state::MovementState::Idle,

    // If we are falling or something, we don't want to change mid-air
    _ => state.move_state.clone(),
  };

  println!("New state: {:?}", new_state);

  state.handle_state_change(new_state);
}