#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Password hash error")]
    PasswordHash(String),
    #[error("Deserialize toml error")]
    TomlDe(#[from] toml::de::Error),
    #[error("Serialize toml error")]
    TomlSer(#[from] toml::ser::Error),
    #[error("Missing config value: ({0})")]
    MissingConfig(String),
    #[error("Cannot get default config directory")]
    DefaultConfigDir,
    #[error("Uninitialized global var: {0}")]
    UninitializedOnceCell(String),
    #[error("Once cell is initializing: {0}")]
    InitializingOnceCell(String),
    #[error("Once cell is already Initialized: {0}")]
    AlreadyInitializedOnceCell(String),
    #[cfg(feature="desktop")]
    #[error("DB Error: {0}")]
    Db(#[from]sea_orm::DbErr),
}

impl<T> From<tokio::sync::SetError<T>> for Error{
    fn from(e: tokio::sync::SetError<T>) -> Self {
        match e {
            tokio::sync::SetError::AlreadyInitializedError(_) => Self::AlreadyInitializedOnceCell(std::any::type_name::<T>().to_string()),
            tokio::sync::SetError::InitializingError(_) => Self::InitializingOnceCell(std::any::type_name::<T>().to_string())
        }
    }
}
