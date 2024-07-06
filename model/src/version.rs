use serde::Deserialize;

use crate::{asset::AssetIndex, library::Library};

pub type Libraries = Vec<Library>;

#[derive(Deserialize)]
pub struct Version {
    #[serde(alias = "assetIndex")]
    pub asset_index: AssetIndex,
    pub downloads: Download,
    pub id: String,
    pub libraries: Libraries,
    #[serde(alias = "mainClass")]
    pub main_class: String,
    #[serde(alias = "releaseTime")]
    pub release_time: String,
    pub time: String,
    #[serde(alias = "type")]
    pub type_: String,
}

#[derive(Deserialize)]
pub struct Download {
    pub client: Client,
}

#[derive(Deserialize)]
pub struct Client {
    pub sha1: String,
    pub size: u32,
    pub url: String,
}
