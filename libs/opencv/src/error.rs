use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    PathConvertToUnicode,
    UnableOpenFile,
    UnableSaveFile,
    Convert(Box<dyn StdError + Send + Sync + 'static>),
    UnsupportedImageReader,
    UnsupportedImageWriter,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Error::Convert(ref error) => {
                write!(f, "{}", error)
            },
            _ => {
                write!(f, "{}", self.description())
            },
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            &Error::PathConvertToUnicode => {
                "Unable convert path to string due invalid unicode characters"
            },
            &Error::UnableOpenFile => {
                "Unable to open image"
            },
            &Error::UnsupportedImageReader => {
                "Unsupported image reader"
            },
            &Error::UnsupportedImageWriter => {
                "Unsupported image writer"
            },
            &Error::UnableSaveFile => {
                "Unable to save image"
            },
            &Error::Convert(ref error) => {
                error.description()
            }
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            &Error::Convert(ref error) => {
                Some(error.as_ref())
            },
            _ => {
                None
            }
        }
    }
}