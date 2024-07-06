use std::path::Path;

use sha1::{Digest, Sha1};

pub mod asset;
pub mod library;
pub mod version;

pub trait Download {
    fn download(&self, game_dir: &Path) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn get<T: reqwest::IntoUrl>(url: T) -> reqwest::Result<reqwest::blocking::Response> {
    reqwest::blocking::Client::builder()
        .timeout(None)
        .build()?
        .get(url)
        .send()
}

pub fn sha1<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let mut hasher = Sha1::new();
    file_hashing::get_hash_file(path, &mut hasher)
}

pub trait LibaryAllowed {
    fn allowed(&self) -> bool;
}
