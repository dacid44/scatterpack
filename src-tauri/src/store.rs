use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use anyhow::Context;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    packing_list::{PackingList, UniqueItem},
    platform,
};

const STORE_PATH: &str = "store";

pub fn init_store(app: tauri::AppHandle) -> io::Result<()> {
    let mut path = platform::data_path(app);
    path.push(STORE_PATH);
    fs::create_dir_all(path)
}

pub trait Store: Serialize + DeserializeOwned {
    const FILENAME: &'static str;
    const ERROR_NAME: &'static str;

    fn store_path(app: tauri::AppHandle) -> PathBuf {
        let mut path = platform::data_path(app);
        path.push(STORE_PATH);
        path.push(Self::FILENAME);
        path
    }

    fn load(app: tauri::AppHandle) -> anyhow::Result<Self> {
        serde_json::from_reader(
            File::open(Self::store_path(app))
                .with_context(|| format!("Failed to open the {} store file", Self::ERROR_NAME))?,
        )
        .with_context(|| format!("Failed to deserialize the {} store file", Self::ERROR_NAME))
    }

    fn save(&self, app: tauri::AppHandle) -> anyhow::Result<()> {
        serde_json::to_writer(
            File::create(Self::store_path(app))
                .with_context(|| format!("Failed to write the {} store file", Self::ERROR_NAME))?,
            self,
        )
        .with_context(|| format!("Failed to serialize the {} store", Self::ERROR_NAME))
    }
}

impl Store for Vec<PackingList> {
    const FILENAME: &'static str = "packing_lists.json";
    const ERROR_NAME: &'static str = "packing list";
}

impl Store for Vec<UniqueItem> {
    const FILENAME: &'static str = "unique_items.json";
    const ERROR_NAME: &'static str = "unique item";
}
