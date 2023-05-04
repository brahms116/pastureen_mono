mod error;
use serde::{Deserialize, Serialize};
pub use error::*;

#[derive(Serialize, Deserialize)]
pub struct PRPCReqeust<T> {
    pub auth: Option<String>,
    pub command: String,
    pub params: T,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PRPCResponse<T> {
    Result(T),
    Error { code: u8, message: String },
}

impl<T> From<PRPCError> for PRPCResponse<T> {
    fn from(error: PRPCError) -> Self {
        Self::Error {
            code: error.kind.to_code(),
            message: error.message,
        }
    }
}
