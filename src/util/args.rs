pub fn has_arg(a: impl AsRef<str>) -> bool {
  std::env::args().any(|arg| arg.to_lowercase() == a.as_ref().to_lowercase())
}

pub fn menu() -> bool {
  has_arg("-menu")
}
