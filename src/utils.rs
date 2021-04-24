use std::io;

#[derive(Debug)]
pub enum Error {
    Serenity(serenity::Error),
    Io(io::Error),
    None(&'static str),
}

macro_rules! from_error {
    ($enum:ident, $err:ty) => {
        impl From<$err> for Error {
            fn from(e: $err) -> Error {
                Error::$enum(e)
            }
        }
    };
}

from_error!(Serenity, serenity::Error);
from_error!(Io, io::Error);
