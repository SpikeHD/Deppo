use windows::Win32::{
  UI,
  Foundation::RECT
};

pub fn taskbar_height() -> u32 {
  // Get height of the taskbar using SPI_GETWORKAREA and GetSystemMetrics (for screen size)
  let mut rect = RECT::default();

  unsafe {
    UI::WindowsAndMessaging::SystemParametersInfoW(
      UI::WindowsAndMessaging::SPI_GETWORKAREA,
      0,
      Some(&mut rect as *mut _ as *mut _),
      UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS::default()
    ).expect("Failed to get work area");
  };

  let screen_height = unsafe {
    UI::WindowsAndMessaging::GetSystemMetrics(UI::WindowsAndMessaging::SM_CYSCREEN)
  };

  (screen_height - rect.bottom) as u32
}

pub fn desktop_size() -> (u32, u32) {
  let mut rect = RECT::default();

  unsafe {
    UI::WindowsAndMessaging::SystemParametersInfoW(
      UI::WindowsAndMessaging::SPI_GETWORKAREA,
      0,
      Some(&mut rect as *mut _ as *mut _),
      UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS::default()
    ).expect("Failed to get work area");
  };

  ((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32)
}