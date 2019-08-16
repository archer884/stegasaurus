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
