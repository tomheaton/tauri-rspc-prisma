use std::path::PathBuf;

/// Get the trp config directory.
/// This config directory has the following structure:
/// - dev.db
/// - assets/ # All the images/videos/audios are stored here
/// - config.toml
pub fn get_trp_dir() -> PathBuf {
  let path = platform_dirs::AppDirs::new(Some("trp"), true).unwrap();
  let mut data_dir = path.data_dir;

  // check if in dev mode
  if cfg!(debug_assertions) {
    data_dir.push("dev");
  }

  return data_dir;
}
