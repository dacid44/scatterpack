use std::{fs, io, path::PathBuf};

use crate::platform;

const THUMBNAIL_STORE_PATH: &str = "item_thumbnails";

pub struct Thumbnails {
    base_path: PathBuf,
}

impl Thumbnails {
    pub fn new(app: tauri::AppHandle) -> Self {
        Self {
            base_path: platform::data_path(app).join(THUMBNAIL_STORE_PATH),
        }
    }

    pub fn list_thumbnails(&self) -> Result<Vec<String>, io::Error> {
        fs::read_dir(&self.base_path)?
            .map(|path| path.map(|path| path.file_name().to_string_lossy().into_owned()))
            .collect()
    }
}
