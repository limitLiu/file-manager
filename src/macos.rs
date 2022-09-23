use std::env;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
pub fn home_dir() -> Option<PathBuf> {
  env::var_os("HOME")
    .and_then(|h| if h.is_empty() { None } else { Some(h) })
    .map(PathBuf::from)
}

pub fn home_dir_string() -> Option<String> {
  if let Some(h) = home_dir() {
    return h.to_str().map(String::from);
  }
  None
}
