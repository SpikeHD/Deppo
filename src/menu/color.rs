pub fn rgba_to_int32(r: u8, g: u8, b: u8, a: u8) -> i32 {
  (r as i32) << 24 | (g as i32) << 16 | (b as i32) << 8 | a as i32
}
