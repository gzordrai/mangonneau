use thiserror::Error;

pub type Result<T> = core::result::Result<T, MangonneauError>;

#[derive(Debug, Error)]
pub enum MangonneauError {
    #[error("missing or invalid environment variable {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("failed to parse TOML config: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("failed to serialize TOML: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("G5K client error: {0}")]
    G5KClient(#[from] g5k::error::G5KError),
}
