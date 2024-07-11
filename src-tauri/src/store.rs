use std::{fs::{self, File}, io, path::PathBuf};

use anyhow::Context;

use crate::{
    packing_list::{PackingList, UniqueItem},
    platform,
};

const STORE_PATH: &str = "store";

pub fn init_store(app: tauri::AppHandle) -> io::Result<()> {
    let mut path = platform::data_path(app);
    path.push(STORE_PATH);
    fs::create_dir_all(STORE_PATH)
}

pub trait Store: Sized {
    const FILENAME: &'static str;

    fn store_path(app: tauri::AppHandle) -> PathBuf {
        let mut path = platform::data_path(app);
        path.push(STORE_PATH);
        path.push(Self::FILENAME);
        path
    }

    fn load(app: tauri::AppHandle) -> anyhow::Result<Self>;

    fn save(&self, app: tauri::AppHandle) -> anyhow::Result<()>;
}

impl Store for Vec<PackingList> {
    const FILENAME: &'static str = "packing_lists.json";

    fn load(app: tauri::AppHandle) -> anyhow::Result<Self> {
        serde_json::from_reader(
            File::open(Self::store_path(app))
                .context("Failed to open the packing list store file")?,
        )
        .context("Failed to deserialize the packing list store file")
    }

    fn save(&self, app: tauri::AppHandle) -> anyhow::Result<()> {
        serde_json::to_writer(
            File::create(Self::store_path(app)).context("Failed to write the packing list store file")?,
            self
        )
        .context("Failed to serialize the packing list store")
    }
}

impl Store for Vec<UniqueItem> {
    const FILENAME: &'static str = "unique_items.json";

    fn load(app: tauri::AppHandle) -> anyhow::Result<Self> {
        serde_json::from_reader(
            File::open(Self::store_path(app))
                .context("Failed to open the unique item store file")?,
        )
        .context("Failed to deserialize the unique item store file")
    }

    fn save(&self, app: tauri::AppHandle) -> anyhow::Result<()> {
        serde_json::to_writer(
            File::create(Self::store_path(app)).context("Failed to write the unique item store file")?,
            self
        )
        .context("Failed to serialize the unique item store")
    }
}
