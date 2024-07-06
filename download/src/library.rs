use std::path::Path;

use model::{library, version::Libraries};

use crate::{Download, LibaryAllowed};

impl LibaryAllowed for library::Library {
    fn allowed(&self) -> bool {
        let mut allowed = true;

        if self.rules.is_some() {
            for rule in self.rules.as_ref().unwrap() {
                if rule.os.name == "osx" && !cfg!(target_os = "macos") {
                    allowed = false;
                    break;
                } else if rule.os.name == "linux" && !cfg!(target_os = "linux") {
                    allowed = false;
                    break;
                } else if rule.os.name == "windows" && !cfg!(target_os = "windows") {
                    allowed = false;
                    break;
                }
            }
        }

        if self.name.contains("natives") {
            if self.name.contains("x86") && !cfg!(target_arch = "x86") {
                allowed = false;
            } else if self.name.contains("arm64") && !cfg!(target_arch = "aarch64") {
                allowed = false;
            } else if !cfg!(target_arch = "x86_64") {
                allowed = false;
            }
        }

        allowed
    }
}

impl Download for Libraries {
    fn download(&self, game_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Downloading libraries");

        let libraries_dir = &game_dir.join("libraries");

        if !libraries_dir.exists() {
            std::fs::create_dir_all(libraries_dir)?;
        }

        for library in self {
            if !library.allowed() {
                continue;
            }

            let library_file = &library.downloads.artifact.path;

            let library_path = &libraries_dir.join(library_file);

            if !library_path.parent().unwrap().exists() {
                std::fs::create_dir_all(library_path.parent().unwrap())?;
            }

            if library_path.exists() {
                if crate::sha1(library_path)?.eq(&library.downloads.artifact.sha1) {
                    continue;
                } else {
                    std::fs::remove_file(library_path)?;
                }
            }

            let url = &library.downloads.artifact.url;

            println!("Downloading library: {}", url);

            let bytes = crate::get(url)?.bytes()?;

            std::fs::write(library_path, bytes)?;
        }

        Ok(())
    }
}

mod tests {
    use super::*;
    use model::version::Version;

    #[test]
    fn test_download() {
        let game = reqwest::blocking::get("https://piston-meta.mojang.com/v1/packages/177e49d3233cb6eac42f0495c0a48e719870c2ae/1.21.json")
            .unwrap()
            .json::<Version>()
            .unwrap();

        let download_path = &std::env::temp_dir().join("rust-minecraft-client-launch");
        std::fs::create_dir_all(download_path).unwrap_or_else(|err| panic!("{:?}", err));

        if let Err(err) = game.libraries.download(download_path) {
            panic!("{:?}", err);
        }
    }
}
