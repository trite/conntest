#[derive(Debug)]
pub enum Error {
    MissingRequiredArgument(String)
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::MissingRequiredArgument(arg) => write!(fmt, "Missing required argument: {arg}"),
        }
    }
}