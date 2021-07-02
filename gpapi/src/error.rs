use std::error::Error as StdError;
use std::io::Error as IOError;
use std::fmt;

use protobuf::error::ProtobufError;

#[derive(Debug)]
pub enum ErrorKind {
    FileExists,
    DirectoryMissing,
    InvalidApp,
    IO(IOError),
    Str(String),
    Protobuf(ProtobufError),
    Other(Box<dyn StdError>),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(k: ErrorKind) -> Error {
        Error {
            kind: k,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error {
            kind: ErrorKind::IO(err),
	}
    }
}

impl From<Box<dyn StdError>> for Error {
    fn from(err: Box<dyn StdError>) -> Error {
        Error {
            kind: ErrorKind::Other(err),
	}
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Error {
        Error {
            kind: ErrorKind::Str(err.to_string()),
	}
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error {
            kind: ErrorKind::Str(err),
	}
    }
}



impl From<ProtobufError> for Error {
    fn from(err: ProtobufError) -> Error {
        Error {
            kind: ErrorKind::Protobuf(err),
	}
    }
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            ErrorKind::FileExists => write!(f, "File already exists"),
            ErrorKind::InvalidApp => write!(f, "Invalid app response"),
            ErrorKind::DirectoryMissing => write!(f, "Destination path provided is not a valid directory"),
            ErrorKind::IO(err) => err.fmt(f),
            ErrorKind::Str(err) => err.fmt(f),
            ErrorKind::Protobuf(err) => err.fmt(f),
            ErrorKind::Other(err) => err.fmt(f),
        }
    }
}