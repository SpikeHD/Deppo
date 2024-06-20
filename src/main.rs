use raylib::prelude::*;

mod animation;
mod runtime;

fn main() {
  // TODO determine based on image/gif size
  let mut window_height = 1000;
  let mut window_width = 1000;

  let mut test_anim = animation::load_gif(std::path::PathBuf::from("test.gif"));

  let (mut rl, thread) = raylib::init()
    .size(window_width, window_height)
    .title("Gif Test")
    .transparent()
    .undecorated()
    .build();

  let mut rl_anim = animation::raw_to_texture_2d(&mut rl, &thread, &test_anim);

  rl.set_window_size(window_width, window_height);

  rl.set_target_fps(15);

  while !rl.window_should_close() {
    let frame = &rl_anim.frames[test_anim.current_frame as usize];

    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::BLANK);
    d.draw_texture_ex(frame, Vector2{ x: 0., y: 0. }, 0., 1., Color::WHITE);

    test_anim.current_frame += 1;
    if test_anim.current_frame >= test_anim.frame_count {
      test_anim.current_frame = 0;
    }
  }
}