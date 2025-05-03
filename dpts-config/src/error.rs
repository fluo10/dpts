#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parse toml error")]
    TomlDe(#[from] toml::de::Error),
    #[error("Missing config value: ({0})")]
    MissingConfig(String)

}
