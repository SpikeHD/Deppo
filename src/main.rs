use raylib::prelude::*;
use util::{
  args::has_arg,
  config,
  deppos::{get_current_deppo_file, list_deppos},
};
use window::platform::force_foreground;

mod animation;
mod menu;
mod runtime;
mod util;
mod window;

fn choose_random_animation(
  animations: &[animation::AnimationTexture2D],
) -> &animation::AnimationTexture2D {
  let index = rand::random::<usize>() % animations.len();
  &animations[index]
}

fn child_is_alive(child: &mut std::process::Child) -> bool {
  match child.try_wait() {
    Ok(Some(_)) => false,
    Ok(None) => true,
    Err(_) => false,
  }
}

fn main() {
  // Opening in menu mode
  if util::args::menu() || list_deppos().is_empty() {
    menu::run();
    return;
  }

  // TODO make configurable
  let mut state = runtime::state::load(get_current_deppo_file());
  let config = config::get_config();

  let (mut rl, thread) = raylib::init()
    .title("Deppo")
    .transparent()
    .undecorated()
    .log_level(TraceLogLevel::LOG_NONE)
    .build();

  // TODO this doesn't work but I would like it to!!
  unsafe { force_foreground(rl.get_window_handle()) };

  let animation_list = runtime::state::load_all_animations(&mut rl, &thread, &state);
  let mut rl_anims = animation_list.idle.as_ref().unwrap_or_else(|| {
    log!("No idle animation found. A character should have at least one idle animation. Exiting.");
    std::process::exit(1);
  });

  let mut rl_anim: &animation::AnimationTexture2D = choose_random_animation(rl_anims);

  let (w, h) = (rl_anim.width as i32, rl_anim.height as i32);
  // used for scaling
  let (mut w_f, mut h_f) = (w as f32, h as f32);

  if let Some(scale) = state.config.scale {
    w_f *= scale;
    h_f *= scale;
  }

  let (w_final, h_final) = (w_f as i32, h_f as i32);

  rl.set_window_size(w_final, h_final);
  rl.set_target_fps(state.config.fps as u32);

  while !rl.window_should_close() {
    // Behaviour
    runtime::behaviour::maybe_toggle_walk(&mut state);

    // Handlers
    runtime::control::handle_mouse(&mut rl, &mut state, w_final, h_final);

    // Handle opening menu
    if runtime::control::maybe_open_menu(&mut rl)
      && (state.menu_process_handle.is_none()
        || !child_is_alive(state.menu_process_handle.as_mut().unwrap()))
    {
      // Spawn ourselves, but with the -menu flag
      let mut cmd = std::process::Command::new(std::env::current_exe().unwrap());
      cmd.arg("-menu");

      if has_arg("debug") {
        cmd.arg("-debug");
      }

      let handle = cmd.spawn().unwrap();
      state.menu_process_handle = Some(handle);
    }

    if runtime::control::maybe_should_quit(&mut rl) {
      break;
    }

    // Do window-based physics
    runtime::physics::do_gravity(&mut state, &mut rl);
    runtime::physics::do_horizontal_checks(&mut state, &mut rl);
    runtime::physics::do_movement(&mut state, &mut rl);
    runtime::physics::handle_friction(&mut state);

    // Set animation if move_state_changed is true
    if state.move_state_changed {
      state.move_state_changed = false;
      let anims = &match state.move_state {
        runtime::state::MovementState::Idle => animation_list.idle.as_ref(),
        runtime::state::MovementState::Walk => animation_list.walk.as_ref(),
        runtime::state::MovementState::Falling => animation_list.fall.as_ref(),
        runtime::state::MovementState::Drag => animation_list.drag.as_ref(),
        runtime::state::MovementState::Click => animation_list.click.as_ref(),
      };

      // If there is no animation for this state, just stay at whatever we were at
      if anims.is_none() {
        log!(
          "Failed to load animation for state {:?}. Going back to idle.",
          state.move_state
        );
        continue;
      }

      log!("Switching to animation for state {:?}", state.move_state);

      rl_anims = anims.unwrap();
      rl_anim = choose_random_animation(rl_anims);
    }

    let frame = &rl_anim.frames[state.current_frame as usize];

    // Create a rectangle to contain the frame
    let source = Rectangle::new(
      0.,
      0.,
      if state.flip_x {
        -frame.width as f32
      } else {
        frame.width as f32
      },
      if state.flip_y {
        -frame.height as f32
      } else {
        frame.height as f32
      },
    );

    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::BLANK);
    d.draw_texture_pro(
      frame,
      source,
      Rectangle::new(0., 0., w_f, h_f),
      Vector2::new(0., 0.),
      0.,
      Color::WHITE,
    );

    if config.show_debug_info {
      let font_size = (8. * state.config.scale.unwrap_or(1.)).floor() as i32;

      // Draw text that says the current move state
      d.draw_text(
        &format!("{:?}", state.move_state),
        10,
        10,
        font_size,
        Color::WHITE,
      );

      // Draw current FPS
      d.draw_text(
        &format!("fps: {}", d.get_fps()),
        10,
        30,
        font_size,
        Color::WHITE,
      );

      // Show the current deppo
      d.draw_text(
        &format!("Deppo: {}", state.config.name),
        10,
        50,
        font_size,
        Color::WHITE,
      );
    }

    state.current_frame += 1;
    if state.current_frame >= rl_anim.frame_count {
      state.current_frame = 0;
    }
  }
}
