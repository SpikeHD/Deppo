use raylib::prelude::*;

mod animation;
mod runtime;
mod window;

fn main() {
  let mut state = runtime::state::load(std::path::PathBuf::from("example_animation/config.json"));

  let (mut rl, thread) = raylib::init()
    .title("Gif Test")
    .transparent()
    .undecorated()
    .build();

  let animation_list = runtime::state::load_all_animations(&mut rl, &thread, &state);
  let rl_anims = &animation_list.idle.unwrap();

  // TODO randomly pick from the list each time the state changes
  let rl_anim = &rl_anims[0];
  
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
    // Hanlders
    runtime::control::handle_mouse(&mut rl, &mut state, w_final, h_final);
    
    // Do window-based physics
    runtime::physics::do_gravity(&mut state, &mut rl);
    runtime::physics::do_horizontal_checks(&mut state, &mut rl);
    runtime::physics::do_movement(&mut state, &mut rl);

    let frame = &rl_anim.frames[state.current_frame as usize];
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::BLANK);
    d.draw_texture_ex(
      frame,
      Vector2{ x: 0., y: 0. },
      0.,
      match state.config.scale {
        Some(scale) => scale,
        None => 1.,
      },
      Color::WHITE
    );

    state.current_frame += 1;
    if state.current_frame >= rl_anim.frame_count {
      state.current_frame = 0;
    }
  }
}