pub fn rgba_to_int32(r: u8, g: u8, b: u8, a: u8) -> i32 {
  (r as i32) << 24 | (g as i32) << 16 | (b as i32) << 8 | a as i32
}

pub fn brighten(color: i32, amount: f32) -> i32 {
  let r = ((color >> 24) & 0xFF) as f32;
  let g = ((color >> 16) & 0xFF) as f32;
  let b = ((color >> 8) & 0xFF) as f32;
  let a = (color & 0xFF) as f32;

  let r = (r * amount).min(255.0).max(0.0) as u8;
  let g = (g * amount).min(255.0).max(0.0) as u8;
  let b = (b * amount).min(255.0).max(0.0) as u8;
  let a = (a * amount).min(255.0).max(0.0) as u8;

  rgba_to_int32(r, g, b, a)
}