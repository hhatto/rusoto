use std::error::Error;
use std::fmt;
use std::io;

use crate::credential::CredentialsError;

use super::proto::xml::util::XmlParseError;
use super::request::{BufferedHttpResponse, HttpDispatchError};

/// Generic error type returned by all rusoto requests.
#[derive(Debug, PartialEq)]
pub enum RusotoError<E> {
    /// A service-specific error occurred.
    Service(E),
    /// A common-service error occurred.
    ServiceCommonError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

/// Result carrying a generic `RusotoError`.
pub type RusotoResult<T, E> = Result<T, RusotoError<E>>;

impl<E> From<XmlParseError> for RusotoError<E> {
    fn from(err: XmlParseError) -> Self {
        let XmlParseError(message) = err;
        RusotoError::ParseError(message.to_string())
    }
}

impl<E> From<serde_json::error::Error> for RusotoError<E> {
    fn from(err: serde_json::error::Error) -> Self {
        RusotoError::ParseError(err.to_string())
    }
}

impl<E> From<CredentialsError> for RusotoError<E> {
    fn from(err: CredentialsError) -> Self {
        RusotoError::Credentials(err)
    }
}

impl<E> From<HttpDispatchError> for RusotoError<E> {
    fn from(err: HttpDispatchError) -> Self {
        RusotoError::HttpDispatch(err)
    }
}

impl<E> From<io::Error> for RusotoError<E> {
    fn from(err: io::Error) -> Self {
        RusotoError::HttpDispatch(HttpDispatchError::from(err))
    }
}

impl<E: Error + 'static> fmt::Display for RusotoError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error + 'static> Error for RusotoError<E> {
    fn description(&self) -> &str {
        match *self {
            RusotoError::Service(ref err) => err.description(),
            RusotoError::ServiceCommonError(ref err) => err,
            RusotoError::Validation(ref cause) => cause,
            RusotoError::Credentials(ref err) => err.description(),
            RusotoError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RusotoError::ParseError(ref cause) => cause,
            RusotoError::Unknown(ref cause) => cause.body_as_str(),
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RusotoError::Service(ref err) => Some(err),
            RusotoError::Credentials(ref err) => Some(err),
            RusotoError::HttpDispatch(ref err) => Some(err),
            _ => None,
        }
    }
}
