use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u32,
    #[serde(alias = "totalSize")]
    pub total_size: u32,
    pub url: String,
}

#[derive(Deserialize)]
pub struct Index {
    pub objects: HashMap<String, Object>,
}

#[derive(Deserialize)]
pub struct Object {
    pub hash: String,
    pub size: u32,
}
