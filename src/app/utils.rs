use thiserror::Error;

#[derive(Error, Debug)]
pub enum TinoError {
    #[error("Configuration directory not found.")]
    ConfigDirNotFound,
    #[error("Home directory not found.")]
    HomeDirNotFound,
    #[error("Error while reading config file: {0}")]
    ReadConfigFileFailed(std::io::Error),
    #[error("Error while deserializing config file content: {0}")]
    DeserializeConfigFileContentFailed(toml::de::Error),
    #[error("A valid category haven't be selected.")]
    NotSelectedCategory,
    #[error("Error while trying to read {0} directory: {1}")]
    DirCouldNotBeReaded(String, std::io::Error),
    #[error("Error while reading tino file content: {0}")]
    ReadTinoFileFailed(std::io::Error),
    #[error("A file haven't be selected.")]
    NotSelectedTinoFile,
}
