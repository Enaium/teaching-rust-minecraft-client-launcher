use serde::Deserialize;

#[derive(Deserialize)]
pub struct Library {
    pub downloads: Download,
    pub name: String,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Deserialize)]
pub struct Rule {
    pub action: String,
    pub os: Os,
}

#[derive(Deserialize)]
pub struct Os {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Download {
    pub artifact: Artifact,
}

#[derive(Deserialize)]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: i32,
    pub url: String,
}
