use std::{convert, fmt, io};

#[derive(Debug)]
pub enum ErrorKind {
    UserInput,
    MQTT,
    IO,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    source: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new<S>(kind: ErrorKind, message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            kind,
            message: message.into(),
            source: None,
        }
    }

    pub fn new_with_source<S>(
        kind: ErrorKind,
        message: S,
        source: Box<dyn std::error::Error>,
    ) -> Self
    where
        S: Into<String>,
    {
        Self {
            kind,
            message: message.into(),
            source: Some(source),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;
        let prefix = match self.kind {
            UserInput => "Illegal user input",
            MQTT => "MQTT error",
            IO => "IO error",
        };

        write!(f, "{}: {}", prefix, self.message)
    }
}

impl convert::From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::new_with_source(ErrorKind::IO, error.to_string(), error.into())
    }
}

impl convert::From<rumqtt::ClientError> for Error {
    fn from(error: rumqtt::ClientError) -> Self {
        // rumqtt::ClientError doesn't implmenet std::error::Error
        Error::new(ErrorKind::MQTT, error.to_string())
    }
}
