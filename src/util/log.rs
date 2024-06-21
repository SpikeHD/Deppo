use chrono::Local;

pub fn log(message: impl AsRef<str>) {
  let time = Local::now();
  println!("[{}] {}", time.format("%Y-%m-%d %H:%M:%S"), message.as_ref());
}

#[macro_export]
macro_rules! log {
  ($($arg:tt)*) => {
    $crate::util::log::log(format!($($arg)*));
  }
}