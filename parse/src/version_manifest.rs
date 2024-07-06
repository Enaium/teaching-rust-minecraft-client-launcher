use model::version_manifest::*;

use crate::Parse;

impl Parse<&str> for VersionManifest {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<VersionManifest>(value)
    }
}

impl Parse<&str> for Latest {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Latest>(value)
    }
}

impl Parse<&str> for Version {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Version>(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = Version::parse(
            r#"{"id": "1.21", "type": "release", "url": "https://piston-meta.mojang.com/v1/packages/177e49d3233cb6eac42f0495c0a48e719870c2ae/1.21.json", "time": "2024-06-13T08:32:38+00:00", "releaseTime": "2024-06-13T08:24:03+00:00"}"#,
        ).unwrap_or_else(|err| panic!("{:?}",err));

        assert_eq!("1.21", version.id);
        assert_eq!("release", version.type_);
        assert_eq!("https://piston-meta.mojang.com/v1/packages/177e49d3233cb6eac42f0495c0a48e719870c2ae/1.21.json", version.url);
        assert_eq!("2024-06-13T08:32:38+00:00", version.time);
        assert_eq!("2024-06-13T08:24:03+00:00", version.release_time);
    }

    #[test]
    fn test_latest() {
        let latest = Latest::parse(r#"{"release": "1.21", "snapshot": "1.21"}"#)
            .unwrap_or_else(|err| panic!("{:?}", err));
        assert_eq!("1.21", latest.release);
        assert_eq!("1.21", latest.snapshot);
    }

    #[test]
    fn test_version_manifest() {
        let version_manifest =
            VersionManifest::parse(r#"{"latest": {"release": "1.21", "snapshot": "1.21"}, "versions": [{"id": "1.21", "type": "release", "url": "https://piston-meta.mojang.com/v1/packages/177e49d3233cb6eac42f0495c0a48e719870c2ae/1.21.json", "time": "2024-06-13T08:32:38+00:00", "releaseTime": "2024-06-13T08:24:03+00:00"}]}"#).unwrap_or_else(|err| panic!("{:?}", err));
        assert_eq!("1.21", version_manifest.latest.release);
        assert_eq!("1.21", version_manifest.latest.snapshot);
        assert_eq!("1.21", version_manifest.versions[0].id);
        assert_eq!("release", version_manifest.versions[0].type_);
        assert_eq!("https://piston-meta.mojang.com/v1/packages/177e49d3233cb6eac42f0495c0a48e719870c2ae/1.21.json", version_manifest.versions[0].url);
        assert_eq!(
            "2024-06-13T08:32:38+00:00",
            version_manifest.versions[0].time
        );
        assert_eq!(
            "2024-06-13T08:24:03+00:00",
            version_manifest.versions[0].release_time
        );
    }
}
