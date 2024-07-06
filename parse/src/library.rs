use model::library::*;

use crate::Parse;

impl Parse<&str> for Library {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Library>(value)
    }
}

impl Parse<&str> for Rule {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Rule>(value)
    }
}

impl Parse<&str> for Os {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Os>(value)
    }
}

impl Parse<&str> for Download {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Download>(value)
    }
}

impl Parse<&str> for Artifact {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Artifact>(value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_library() {
        let library = Library::parse(
                r#"{"downloads": {"artifact": {"path": "ca/weblite/java-objc-bridge/1.1/java-objc-bridge-1.1.jar", "sha1": "1227f9e0666314f9de41477e3ec277e542ed7f7b", "size": 1330045, "url": "https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.1/java-objc-bridge-1.1.jar"}}, "name": "ca.weblite:java-objc-bridge:1.1", "rules": [{"action": "allow", "os": {"name": "osx"}}]}"#,
            ).unwrap_or_else(|err| panic!("{:?}",err));

        assert_eq!("ca.weblite:java-objc-bridge:1.1", library.name);
        assert_eq!(
            "ca/weblite/java-objc-bridge/1.1/java-objc-bridge-1.1.jar",
            library.downloads.artifact.path
        );
        assert_eq!(
            "1227f9e0666314f9de41477e3ec277e542ed7f7b",
            library.downloads.artifact.sha1
        );
        assert_eq!(1330045, library.downloads.artifact.size);
        assert_eq!(
            "https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.1/java-objc-bridge-1.1.jar",
            library.downloads.artifact.url
        );
        let rules = &library.rules.unwrap();
        assert_eq!("allow", rules[0].action);
        assert_eq!("osx", rules[0].os.name);
    }
}
