#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parser error")]
    Clap(#[from] clap::Error),
    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
}

impl Error {
    pub fn print(&self) -> Result<(), Error> {
        match self {
            Error::Clap(x) => Ok(x.print()?),
            _ => unimplemented!(),
        }
    }
}