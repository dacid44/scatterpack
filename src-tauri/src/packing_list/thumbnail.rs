use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use base64::{prelude::BASE64_STANDARD, Engine};

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
    pub fn copy_from_path(&self, from: impl AsRef<Path>, to: &str) -> Result<String, io::Error> {
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

    /// Decodes base64 data and saves the resulting file into the thumbnail store. Expects the
    /// extension argument to either be a filename with an extension, or just an extension.
    pub fn save_from_base64(
        &self,
        from: impl AsRef<str>,
        to: &str,
        extension: impl AsRef<Path>,
    ) -> Result<String, anyhow::Error> {
        let name = format!(
            "{}.{}",
            to,
            extension
                .as_ref()
                .extension()
                .unwrap_or(extension.as_ref().as_os_str())
                .to_string_lossy()
                .trim_start_matches('.'),
        );
        let path = self.base_path.join(&name);
        let data = BASE64_STANDARD
            .decode(from.as_ref())
            .context("Failed to decode base64 thumbnail image data")?;
        let mut file = File::create(path).context("Failed to create thumbnail image file")?;
        file.write_all(&data)
            .context("Failed to write thumbnail image data to file")?;
        Ok(name)
    }

    pub fn delete(&self, name: &str) -> Result<(), io::Error> {
        fs::remove_file(self.base_path.join(name))
    }

    pub fn rename(&self, from: &str, to: &str) -> Result<(), io::Error> {
        fs::rename(self.base_path.join(from), self.base_path.join(to))
    }
}
