#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Uninitialized OnceCell: {0}")]
    UninitializedOnceCell(String),
    #[error("Parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Argon2 Password hash error: {0}")]
    PasswordHash(String),
    #[error("Parse toml error")]
    TomlDe(#[from] toml::de::Error),
    #[error("Missing config value: ({0})")]
    MissingConfig(String)

}
