use crate::Parse;
use model::version::*;

impl Parse<&str> for Version {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Version>(value)
    }
}

impl Parse<&str> for Download {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Download>(value)
    }
}

impl Parse<&str> for Client {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Client>(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let game = Version::parse(
            r#"{"assetIndex": {"id": "17", "sha1": "fab15439bdef669e389e25e815eee8f1b2aa915e", "size": 447033, "totalSize": 799252591, "url": "https://piston-meta.mojang.com/v1/packages/fab15439bdef669e389e25e815eee8f1b2aa915e/17.json"}, "downloads": {"client": {"sha1": "0e9a07b9bb3390602f977073aa12884a4ce12431", "size": 26836080, "url": "https://piston-data.mojang.com/v1/objects/0e9a07b9bb3390602f977073aa12884a4ce12431/client.jar"}}, "id": "1.21", "libraries": [], "mainClass": "net.minecraft.client.main.Main", "releaseTime": "2024-06-13T08:24:03+00:00", "time": "2024-06-13T08:32:38+00:00", "type": "release"}"#,
        ).unwrap_or_else(|err| panic!("{:?}",err));

        let client = &game.downloads.client;
        assert_eq!("0e9a07b9bb3390602f977073aa12884a4ce12431", client.sha1);
        assert_eq!(26836080, client.size);
        assert_eq!("https://piston-data.mojang.com/v1/objects/0e9a07b9bb3390602f977073aa12884a4ce12431/client.jar", client.url);

        assert_eq!("1.21", game.id);
        assert_eq!("net.minecraft.client.main.Main", game.main_class);
        assert_eq!("2024-06-13T08:24:03+00:00", game.release_time);
        assert_eq!("2024-06-13T08:32:38+00:00", game.time);
        assert_eq!("release", game.type_);
    }
}
