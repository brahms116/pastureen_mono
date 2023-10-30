use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HttpErrResponseBody {
    pub error_type: String,
    pub message: String,
}

pub trait TypedErr {
    fn error_type(&self) -> String;
}

impl<T> From<T> for HttpErrResponseBody
where
    T: TypedErr + std::fmt::Debug,
{
    fn from(err: T) -> Self {
        HttpErrResponseBody {
            error_type: err.error_type(),
            message: format!("{:?}", err),
        }
    }
}

impl std::fmt::Display for HttpErrResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ error_type: {}, message: {} }}", self.error_type, self.message)
    }
}

impl std::error::Error for HttpErrResponseBody {}

#[derive(Debug, Clone, PartialEq)]
pub enum ClientHttpResponseError {
    RawErr(String),
    TypedServiceErr(HttpErrResponseBody),
}

impl std::fmt::Display for ClientHttpResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientHttpResponseError::RawErr(msg) => write!(f, "RawErr({})", msg),
            ClientHttpResponseError::TypedServiceErr(err) => write!(f, "TypedServiceErr({})", err),
        }
    }
}

impl std::error::Error for ClientHttpResponseError {}
