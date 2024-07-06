use std::path::Path;

use crate::{get, Download};
use model::version_manifest::Version;

impl Download for Version {
    fn download(&self, game_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let game = get(&self.url)?.json::<model::version::Version>()?;

        let versions_dir = &game_dir.join("versions").join(&game.id);

        if !versions_dir.exists() {
            std::fs::create_dir_all(versions_dir)?;
        }

        game.libraries.download(game_dir)?;
        game.asset_index.download(game_dir)?;

        let version_config = &game_dir
            .join("versions")
            .join(&game.id)
            .join(&format!("{}.json", &self.id));

        if version_config.exists() {
            std::fs::remove_file(version_config)?;
        }

        std::fs::File::create(version_config)?;
        std::fs::write(version_config, get(&self.url)?.bytes()?)?;

        let path = &game_dir
            .join("versions")
            .join(&game.id)
            .join(&format!("{}.jar", &game.id));

        if path.exists() {
            if crate::sha1(path)?.eq(&game.downloads.client.sha1) {
                return Ok(());
            } else {
                std::fs::remove_file(path)?;
            }
        }

        std::fs::File::create(path)?;

        let bytes = get(&game.downloads.client.url)?.bytes()?;

        std::fs::write(path, bytes)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download() {
        let version = Version {
            id: "1.21".to_string(),
            type_: "release".to_string(),
            url: "https://piston-meta.mojang.com/v1/packages/177e49d3233cb6eac42f0495c0a48e719870c2ae/1.21.json".to_string(),
            time : "2024-06-13T08:32:38+00:00".to_string(),
            release_time : "2024-06-13T08:24:03+00:00".to_string(),
        };

        let download_path = &std::env::temp_dir().join("rust-minecraft-client-launch");
        std::fs::create_dir_all(download_path).unwrap_or_else(|err| panic!("{:?}", err));

        if let Err(err) = version.download(download_path) {
            panic!("{:?}", err);
        }
    }
}
