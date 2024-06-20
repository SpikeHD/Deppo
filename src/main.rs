use raylib::prelude::*;

mod animation;
mod runtime;

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
  // TODO use for scaling
  let (w_f, h_f) = (w as f32, h as f32);

  rl.set_window_size(w, h);
  rl.set_target_fps(state.config.fps as u32);

  while !rl.window_should_close() {
    let frame = &rl_anim.frames[state.current_frame as usize];

    let mut d = rl.begin_drawing(&thread);

    println!("Frame: {}", state.current_frame);

    d.clear_background(Color::BLANK);
    d.draw_texture_ex(frame, Vector2{ x: 0., y: 0. }, 0., 1., Color::WHITE);

    state.current_frame += 1;
    if state.current_frame >= rl_anim.frame_count {
      state.current_frame = 0;
    }
  }
}