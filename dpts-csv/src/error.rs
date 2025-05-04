#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Missing config value: ({0})")]
    MissingConfig(String)
}
