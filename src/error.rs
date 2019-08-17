use std::error;
use std::fmt::{self, Display};
use std::io;

#[derive(Debug)]
pub enum Error {
    Image(image::ImageError),
    IO(io::Error),
}

impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Self {
        Error::Image(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Image(e) => write!(f, "{}", e),
            Error::IO(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Image(e) => Some(e),
            Error::IO(e) => Some(e),
        }
    }
}
