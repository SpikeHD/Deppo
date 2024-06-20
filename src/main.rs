use raylib::prelude::*;

mod animation;
mod runtime;

fn main() {
  let anim = animation::load_gif(std::path::PathBuf::from("test.gif"));
  let (w, h) = (anim.width as i32, anim.height as i32);

  let (mut rl, thread) = raylib::init()
    .size(w, h)
    .title("Gif Test")
    .transparent()
    .undecorated()
    .build();

  let mut rl_anim = animation::raw_to_texture_2d(&mut rl, &thread, &anim);

  rl.set_window_size(w, h);

  rl.set_target_fps(15);

  while !rl.window_should_close() {
    let frame = &rl_anim.frames[rl_anim.current_frame as usize];

    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::BLANK);
    d.draw_texture_ex(frame, Vector2{ x: 0., y: 0. }, 0., 1., Color::WHITE);

    rl_anim.current_frame += 1;
    if rl_anim.current_frame >= rl_anim.frame_count {
      rl_anim.current_frame = 0;
    }
  }
}