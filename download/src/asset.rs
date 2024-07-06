use std::{fs, path::Path};

use model::asset::{AssetIndex, Index};
use parse::Parse;

use crate::{get, Download};

impl Download for AssetIndex {
    fn download(&self, game_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Downloading asset index: {}", self.id);

        let indexes_dir = &game_dir.join("assets").join("indexes");

        if !indexes_dir.exists() {
            std::fs::create_dir_all(indexes_dir)?;
        }

        let path = &indexes_dir.join(&format!("{}.json", self.id));

        std::fs::File::create(path)?;

        let url = &self.url;
        let text = &get(url)?.text()?;

        std::fs::write(path, text)?;

        let index = Index::parse(text)?;

        let objects_dir = &game_dir.join("assets").join("objects");

        if !objects_dir.exists() {
            std::fs::create_dir_all(objects_dir)?;
        }

        for (_, value) in index.objects {
            let hash = &value.hash;
            let hash_first_two = &hash[0..2];

            let first_two_dir = &objects_dir.join(hash_first_two);

            if !first_two_dir.exists() {
                std::fs::create_dir_all(first_two_dir)?;
            }

            let path = &first_two_dir.join(hash);

            if path.exists() {
                if crate::sha1(path)?.eq(hash) {
                    continue;
                } else {
                    std::fs::remove_file(path)?;
                }
            }

            std::fs::File::create(path)?;

            let url = format!(
                "https://resources.download.minecraft.net/{}/{}",
                hash_first_two, hash
            );

            println!("Downloading asset: {}", url);

            let bytes = get(&url)?.bytes()?;
            fs::write(path, bytes)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Download;

    #[test]
    fn test_asset_index() {
        let asset_index = model::asset::AssetIndex {
            id: "17".to_string(),
            sha1: "fab15439bdef669e389e25e815eee8f1b2aa915e".to_string(),
            size: 447033,
            total_size: 799252591,
            url: "https://piston-meta.mojang.com/v1/packages/fab15439bdef669e389e25e815eee8f1b2aa915e/17.json".to_string(),
        };

        let download_path = &std::env::temp_dir().join("rust-minecraft-client-launch");
        std::fs::create_dir_all(download_path).unwrap_or_else(|err| panic!("{:?}", err));

        if let Err(err) = asset_index.download(download_path) {
            panic!("{:?}", err);
        }
    }
}
