pub fn force_foreground(hwnd: u32) {
  // TODO WIP
}

pub fn desktop_size() -> (u32, u32) {
  let current_monitor = raylib::core::window::get_current_monitor();

  (
    raylib::core::window::get_monitor_width(current_monitor) as u32,
    raylib::core::window::get_monitor_height(current_monitor) as u32 - taskbar_height(),
  )
}

pub fn taskbar_height() -> u32 {
  0
}
