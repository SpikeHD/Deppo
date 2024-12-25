use std::os::raw::c_void;

use windows::Win32::{Foundation::{HWND, RECT}, UI::{self, WindowsAndMessaging::{SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE}}};

use crate::log;

pub fn force_foreground(hwnd: *mut c_void) {
  let x = desktop_size().0 as i32 / 2;
  let y = desktop_size().1 as i32 / 2;

  unsafe {
    SetWindowPos(HWND(hwnd), HWND_TOPMOST, x, y, 0, 0, SWP_NOSIZE | SWP_NOMOVE).unwrap_or_else(|e| {
      log!("Failed to force window to foreground: {}", e);
    });
  }
}

pub fn desktop_size() -> (u32, u32) {
  let current_monitor = raylib::core::window::get_current_monitor();

  (
    raylib::core::window::get_monitor_width(current_monitor) as u32,
    raylib::core::window::get_monitor_height(current_monitor) as u32 - taskbar_height(),
  )
}

pub fn taskbar_height() -> u32 {
  // Get height of the taskbar using SPI_GETWORKAREA and GetSystemMetrics (for screen size)
  let mut rect = RECT::default();

  unsafe {
    UI::WindowsAndMessaging::SystemParametersInfoW(
      UI::WindowsAndMessaging::SPI_GETWORKAREA,
      0,
      Some(&mut rect as *mut _ as *mut _),
      UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS::default(),
    )
    .expect("Failed to get work area");
  };

  let screen_height =
    unsafe { UI::WindowsAndMessaging::GetSystemMetrics(UI::WindowsAndMessaging::SM_CYSCREEN) };

  (screen_height - rect.bottom) as u32
}
