pub mod thumbnail;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PackingList {
    pub name: String,
    pub items: Vec<ListItem>,
}

impl PackingList {
    pub fn new(name: impl Into<String>, items: Vec<ListItem>) -> Self {
        Self {
            name: name.into(),
            items,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PackItem {
    pub name: String,
    pub location: String,
    pub quantity: u32,
}

impl PackItem {
    pub fn new(name: impl Into<String>, location: impl Into<String>, quantity: u32) -> Self {
        Self {
            name: name.into(),
            location: location.into(),
            quantity,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PackCollection {
    pub name: String,
    pub items: Vec<ListItem>,
}

impl PackCollection {
    pub fn new(name: impl Into<String>, items: Vec<ListItem>) -> Self {
        Self {
            name: name.into(),
            items,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "camelCase")]
pub enum ListItem {
    Item(PackItem),
    Collection(PackCollection),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniqueItem {
    name: String,
    description: String,
    location: String,
    thumbnail: Option<String>,
}

impl UniqueItem {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        location: impl Into<String>,
        thumbnail: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            location: location.into(),
            thumbnail,
        }
    }
}
