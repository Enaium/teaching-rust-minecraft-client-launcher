pub mod asset;
pub mod library;
pub mod version;
pub mod version_manifest;

pub trait Parse<T>: Sized {
    type Error;
    fn parse(value: T) -> Result<Self, Self::Error>;
}
