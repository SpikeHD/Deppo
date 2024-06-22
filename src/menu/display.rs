use std::ffi::{CStr, CString};

use raylib::{
  drawing::RaylibDrawHandle, ffi::GuiIconName, math::Rectangle, rgui::RaylibDrawGui, rstr, RaylibHandle
};

use crate::{log, util::paths::deppo_path};

pub fn draw_gui(d: &mut RaylibDrawHandle) {
  let width = d.get_screen_width() as f32;
  let height = d.get_screen_height() as f32;

  // Draw a button that takes you to the Deppo path
  let i_text = d.gui_icon_text(
    GuiIconName::ICON_FOLDER_OPEN,
    Some(rstr!("Open Deppos Folder"))
  );
  let i_text = CString::new(i_text).unwrap();
  
  if d.gui_button(
    Rectangle::new(
      0.,
      0.,
      width,
      20.),
    Some(i_text.as_c_str()),
  ) {
    open::that(
      deppo_path().to_str().unwrap()
    ).unwrap_or_else(|_| {
      log!("Failed to open Deppo path");
    });
  }
}