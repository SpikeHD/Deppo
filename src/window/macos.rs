use cocoa::base::id;
use objc::{class, msg_send, sel, sel_impl};
use objc::runtime::{Class, Object, Sel};

pub fn desktop_size() -> (u32, u32) {
  let current_monitor = raylib::core::window::get_current_monitor();

  println!("height: {}", raylib::core::window::get_monitor_height(current_monitor));

  (
    raylib::core::window::get_monitor_width(current_monitor) as u32,
    raylib::core::window::get_monitor_height(current_monitor) as u32 - taskbar_height(),
  )
}

// TODO WIP
pub fn taskbar_height() -> u32 {
  // Get height of the dock
  unsafe {
    // Subtract the visible screen height from the total screen height
    let screen_cls = class!(NSScreen);
    // call "visibleFrame" on the screen
    let screens: id = msg_send![screen_cls, screens];
    println!("screens: {:?}", screens);

    // just use the first screen in the list
    let screen: id = msg_send![screens, objectAtIndex:0];
    println!("screen: {:?}", screen);
    let visible_frame: id = msg_send![screen, frame];
    println!("visible_frame: {:?}", visible_frame);

    0
  }
}
