use std::path::PathBuf;
use tauri::Manager;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub fn data_path(app: tauri::AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("platform::data_path() should only be available on supported platforms")
}

#[cfg(target_os = "android")]
pub fn data_path(app: tauri::AppHandle) -> PathBuf {
    PathBuf::from("/data/data").join(&app.config().identifier)
}
