use raylib::prelude::*;

use super::color::rgba_to_int32;

pub fn apply_styles(d: &mut RaylibDrawHandle) {
  set_general_styles(d);
  set_toggle_styles(d);
  set_label_styles(d);
}

pub fn set_general_styles(d: &mut RaylibDrawHandle) {
  d.gui_set_style(
    GuiControl::DEFAULT,
    GuiDefaultProperty::TEXT_SIZE as i32,
    20
  );
}

pub fn set_toggle_styles(d: &mut RaylibDrawHandle) {
  let enabled_border = rgba_to_int32(68, 99, 63, 255);
  let enabled_background = rgba_to_int32(92, 171, 125, 255);
  let disabled_border = rgba_to_int32(204, 41, 54, 255);
  let disabled_background = rgba_to_int32(255, 152, 154, 255);

  // Enabled should be green
  d.gui_set_style(
    GuiControl::TOGGLE,
    GuiControlProperty::BORDER_COLOR_NORMAL as i32,
    enabled_border
  );

  // and the background
  d.gui_set_style(
    GuiControl::TOGGLE,
    GuiControlProperty::BASE_COLOR_NORMAL as i32,
    enabled_background
  );

  // And disabled
  d.gui_set_style(
    GuiControl::TOGGLE,
    GuiControlProperty::BORDER_COLOR_PRESSED as i32,
    disabled_border
  );

  d.gui_set_style(
    GuiControl::TOGGLE,
    GuiControlProperty::BASE_COLOR_PRESSED as i32,
    disabled_background
  );

  // font color white
  d.gui_set_style(
    GuiControl::TOGGLE,
    GuiControlProperty::TEXT_COLOR_NORMAL as i32,
    rgba_to_int32(255, 255, 255, 255)
  );

  d.gui_set_style(
    GuiControl::TOGGLE,
    GuiControlProperty::TEXT_COLOR_PRESSED as i32,
    rgba_to_int32(255, 255, 255, 255)
  );
}

pub fn set_label_styles(d: &mut RaylibDrawHandle) {

  d.gui_set_style(
    GuiControl::LABEL,
    GuiControlProperty::TEXT_COLOR_NORMAL as i32,
    // This is a 32 bit int representing a color
    rgba_to_int32(255, 255, 255, 255)
  );
}