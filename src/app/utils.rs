use thiserror::Error;

#[derive(Error, Debug)]
pub enum TinoError {
    #[error("Configuration directory not found.")]
    ConfigDirNotFound,
    #[error("Error while reading config file: {0}")]
    ReadConfigFileFailed(std::io::Error),
    #[error("Error while Deserializing config file content: {0}")]
    DeserializeConfigFileContentFailed(toml::de::Error),
}
