use std::{fs::File, io::Read};

use crate::{log, runtime::state::State};

pub struct Gif {
  pub width: u16,
  pub height: u16,
  pub transparent_idx: Option<u8>,
  pub delay: u16,

  // Full image buffer
  pub buf: Vec<u8>,
}

pub struct AnimationRaw {
  pub width: u16,
  pub height: u16,

  // PNG buffer
  pub frames: Vec<Vec<u8>>,
  pub frame_delay: u32,
  pub current_frame: u32,
  pub frame_count: u32,
}

pub struct AnimationTexture2D {
  pub width: u16,
  pub height: u16,

  pub frames: Vec<raylib::prelude::Texture2D>,
  pub frame_delay: u32,
  pub current_frame: u32,
  pub frame_count: u32,
}

pub fn load_gif_file<R>(r: R) -> AnimationRaw
where
  R: Read,
{
  let mut decoder = gif::DecodeOptions::new();
  decoder.set_color_output(gif::ColorOutput::RGBA);

  let mut decoder = decoder.read_info(r).unwrap();
  let mut frame_count = 0;
  let mut frames: Vec<Gif> = Vec::new();

  while let Some(frame) = decoder.read_next_frame().unwrap() {
    frame_count += 1;

    frames.push(Gif {
      width: frame.width,
      height: frame.height,
      transparent_idx: frame.transparent,
      delay: frame.delay,
      buf: frame.buffer.to_vec(),
    });
  }

  // Create an Animation struct
  AnimationRaw {
    width: frames[0].width,
    height: frames[0].height,
    frames: frames.iter().map(frame_to_png).collect(),
    frame_delay: 0,
    current_frame: 0,
    frame_count,
  }
}

pub fn load_gif_from_zip(state: &State, path: String) -> AnimationRaw {
  log!("Loading gif from zip: {:?}", path);
  let file = File::open(&state.path).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap();

  // Find the file in the zip
  let gif_file = match archive.by_name(&path) {
    Ok(file) => file,
    Err(_) => {
      log!("Gif not found in zip: {:?}", path);
      std::process::exit(1);
    }
  };

  load_gif_file(gif_file)
}

pub fn load_gif_from_file(state: &State, path: String) -> AnimationRaw {
  log!("Loading gif from raw file: {:?}", path);
  let full_path = state.path.join(&path);
  let file = File::open(full_path).unwrap();

  load_gif_file(file)
}

pub fn load_gif(state: &State, path: String) -> AnimationRaw {
  if state.path.extension().unwrap_or_default() == "zip" {
    load_gif_from_zip(state, path)
  } else {
    load_gif_from_file(state, path)
  }
}

pub fn raw_to_texture_2d(
  rl: &mut raylib::prelude::RaylibHandle,
  thread: &raylib::prelude::RaylibThread,
  anim: &AnimationRaw,
) -> AnimationTexture2D {
  AnimationTexture2D {
    width: anim.width,
    height: anim.height,
    frames: anim
      .frames
      .iter()
      .map(|f| {
        let img = raylib::prelude::Image::load_image_from_mem(".png", f).unwrap();
        rl.load_texture_from_image(thread, &img).unwrap()
      })
      .collect(),
    frame_delay: anim.frame_delay,
    current_frame: anim.current_frame,
    frame_count: anim.frame_count,
  }
}

// Convert the frame buffer to a PNG
pub fn frame_to_png(frame: &Gif) -> Vec<u8> {
  let mut png = Vec::new();
  let mut encoder = png::Encoder::new(&mut png, frame.width as u32, frame.height as u32);
  encoder.set_color(png::ColorType::Rgba);
  encoder.set_depth(png::BitDepth::Eight);

  let mut writer = encoder.write_header().unwrap();
  writer.write_image_data(&frame.buf).unwrap();

  // Must be done before moving the png
  drop(writer);

  png
}
