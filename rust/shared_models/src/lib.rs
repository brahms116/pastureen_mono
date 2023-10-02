use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    T: TypedErr + std::fmt::Debug
{
    fn from(err: T) -> Self {
        HttpErrResponseBody {
            error_type: err.error_type(),
            message: format!("{:?}", err)
        }
    }
}

