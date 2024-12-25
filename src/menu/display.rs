use raylib::{
  drawing::RaylibDrawHandle, ffi::GuiIconName, math::Rectangle, rgui::RaylibDrawGui, rstr,
};
use std::ffi::CString;

use crate::{log, util::paths::deppo_path};

use super::{runtime::state::State, styles};

pub fn draw_gui(state: &mut State, d: &mut RaylibDrawHandle) {
  let width = d.get_screen_width() as f32;
  //let height = d.get_screen_height() as f32;

  styles::apply_styles(d);

  // Draw a button that takes you to the Deppo path
  let i_text = d.gui_icon_text(
    GuiIconName::ICON_FOLDER_OPEN,
    Some(rstr!("Open Deppos Folder")),
  );
  let i_text = CString::new(i_text).unwrap();

  if d.gui_button(Rectangle::new(0., 0., width, 40.), Some(i_text.as_c_str())) {
    open::that(deppo_path().to_str().unwrap()).unwrap_or_else(|_| {
      log!("Failed to open Deppo path");
    });
  }

  d.gui_label(
    Rectangle::new(
      0.,
      // 10px padding
      50.,
      width / 2.,
      20.,
    ),
    Some(rstr!("Settings")),
  );

  // Three toggles in a row
  let t_width = width / 3.;
  let t_height = 40.;
  let t_y = 70.;

  d.gui_toggle(
    Rectangle::new(0., t_y, t_width, t_height),
    Some(rstr!("Open on Startup")),
    &mut state.config.open_on_startup,
  );

  d.gui_toggle(
    Rectangle::new(t_width, t_y, t_width, t_height),
    Some(rstr!("Show Debug Info")),
    &mut state.config.show_debug_info,
  );

  d.gui_toggle(
    Rectangle::new(t_width * 2., t_y, t_width, t_height),
    Some(rstr!("Third Option")),
    &mut state.config.open_on_startup,
  );

  // "Switch Deppo" label
  d.gui_label(
    Rectangle::new(0., t_y + t_height + 10., width / 2., 20.),
    Some(rstr!("Switch Deppo")),
  );

  // Loop through all Deppos, draw a button for each
  let mut y = t_y + t_height + 30.;

  for deppo in &state.deppo_list {
    let mut text = deppo.name.clone();

    if let Some(author) = &deppo.author {
      text.push_str(" by ");
      text.push_str(author);
    }

    let text = CString::new(text).unwrap();

    if d.gui_button(Rectangle::new(0., y, width, 40.), Some(text.as_c_str())) {
      log!("Switching to Deppo: {:?}", deppo.name);
      state.config.deppo.clone_from(&deppo.filename.clone().unwrap_or(deppo.name.clone()));
    }

    y += 50.;
  }
}
