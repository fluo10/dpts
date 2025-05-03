#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parser error")]
    Clap(#[from] clap::Error),
    #[error("Parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("IO Error")]
    Io(#[from] std::io::Error),
    #[error("Parse toml error")]
    TomlDe(#[from] toml::de::Error),
    #[error("Missing config value: ({0})")]
    MissingConfig(String)

}

impl Error {
    pub fn print(&self) -> Result<(), Error> {
        match self {
            Error::Clap(x) => Ok(x.print()?),
            _ => unimplemented!(),
        }
    }
}