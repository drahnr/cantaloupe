use thiserror::Error;
use toml;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io again")]
    Io(#[from] std::io::Error),

    #[error("toml went boom")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("toml no serial no do")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("zip zap gz")]
    CompressionFailed,

    #[error("parse parse zhsuh")]
    Parsing,
}

pub type Result<T> = std::result::Result<T, Error>;
