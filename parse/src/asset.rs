use model::asset::*;

use crate::Parse;

impl Parse<&str> for AssetIndex {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<AssetIndex>(value)
    }
}

impl Parse<&str> for Index {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Index>(value)
    }
}

impl Parse<&str> for Object {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Object>(value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_asset_index() {
        let asset_index = AssetIndex::parse(
        r#"{"id": "17", "sha1": "fab15439bdef669e389e25e815eee8f1b2aa915e", "size": 447033, "totalSize": 799252591, "url": "https://piston-meta.mojang.com/v1/packages/fab15439bdef669e389e25e815eee8f1b2aa915e/17.json"}"#).unwrap_or_else(|err| panic!("{:?}",err));
        assert_eq!("17", asset_index.id);
        assert_eq!("fab15439bdef669e389e25e815eee8f1b2aa915e", asset_index.sha1);
        assert_eq!(447033, asset_index.size);
        assert_eq!(799252591, asset_index.total_size);
        assert_eq!("https://piston-meta.mojang.com/v1/packages/fab15439bdef669e389e25e815eee8f1b2aa915e/17.json", asset_index.url);
    }

    #[test]
    fn test_index() {
        let index = Index::parse(r#"{"objects": {"icons/icon_128x128.png": {"hash": "b62ca8ec10d07e6bf5ac8dae0c8c1d2e6a1e3356", "size": 9101}}}"#)
            .unwrap_or_else(|err| panic!("{:?}",err));

        assert_eq!(1, index.objects.len());
        assert_eq!(
            "b62ca8ec10d07e6bf5ac8dae0c8c1d2e6a1e3356",
            index.objects.get("icons/icon_128x128.png").unwrap().hash
        );
        assert_eq!(
            9101,
            index.objects.get("icons/icon_128x128.png").unwrap().size
        );
    }
}
