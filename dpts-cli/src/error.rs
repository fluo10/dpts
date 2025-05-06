#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Missing config value: ({0})")]
    MissingConfig(String),
    #[error("Parse toml error")]
    TomlDe(#[from] toml::de::Error),

}

impl From<dpts_client::error::Error> for Error {
    fn from(e: dpts_client::error::Error) -> Self {
        match e {
            dpts_client::error::Error::TomlDe(x)=> Self::TomlDe(x),
            dpts_client::error::Error::ParseInt(x) => Self::ParseInt(x),
            dpts_client::error::Error::MissingConfig(x) => Self::MissingConfig(x),
        }
    }
}
