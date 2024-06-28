use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct PackingList {
    pub name: String,
    pub items: Vec<PackItem>,
}

impl PackingList {
    pub fn new(name: impl Into<String>, items: Vec<PackItem>) -> Self {
        Self {
            name: name.into(),
            items,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
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