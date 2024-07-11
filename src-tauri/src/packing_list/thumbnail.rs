use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use crate::platform;

const THUMBNAIL_STORE_PATH: &str = "item_thumbnails";

pub struct Thumbnails {
    base_path: PathBuf,
}

impl Thumbnails {
    pub fn new(app: tauri::AppHandle) -> Result<Self, io::Error> {
        let base_path = platform::data_path(app).join(THUMBNAIL_STORE_PATH);
        // Ensure thumbnail store directory exists
        if !base_path.is_dir() {
            fs::create_dir_all(&base_path)?;
        }
        Ok(Self { base_path })
    }

    pub fn list_thumbnails(&self) -> Result<Vec<String>, io::Error> {
        fs::read_dir(&self.base_path)?
            .map(|path| path.map(|path| path.file_name().to_string_lossy().into_owned()))
            .collect()
    }

    /// Get a full store path from a given thumbnail name. Does not validate that the given name is
    /// in the store.
    pub fn get_full_path(&self, name: &str) -> PathBuf {
        self.base_path.join(name)
    }

    /// Copies a file from the given path to the thumbnail store. Appends the original file
    /// extension to the new name given and returns the name.
    pub fn copy_from(&self, from: impl AsRef<Path>, to: &str) -> Result<String, io::Error> {
        // TODO: Convert weird formats like HEIC
        let name = format!(
            "{}.{}",
            to,
            from.as_ref()
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
        );
        let new_path = self.base_path.join(&name);
        let mut old_file = File::open(from)?;
        let mut new_file = File::create(&new_path)?;
        io::copy(&mut old_file, &mut new_file)?;
        Ok(name)
    }

    pub fn delete(&self, name: &str) -> Result<(), io::Error> {
        fs::remove_file(self.base_path.join(name))
    }

    pub fn rename(&self, from: &str, to: &str) -> Result<(), io::Error> {
        fs::rename(self.base_path.join(from), self.base_path.join(to))
    }
}
