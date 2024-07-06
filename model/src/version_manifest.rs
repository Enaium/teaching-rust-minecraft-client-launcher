use serde::Deserialize;

#[derive(Deserialize)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize)]
pub struct Version {
    pub id: String,
    #[serde(alias = "type")]
    pub type_: String,
    pub url: String,
    pub time: String,
    #[serde(alias = "releaseTime")]
    pub release_time: String,
}
