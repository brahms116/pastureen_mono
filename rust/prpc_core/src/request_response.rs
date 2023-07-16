// Global dependencies
use serde::{Deserialize, Serialize};

// External dependencies
use super::error::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PRPCRequest<T> {
    pub auth: Option<String>,
    pub command: String,
    pub params: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PRPCResponse<T> {
    pub result: Option<T>,
    pub error: Option<PRPCError>,
}

pub type PRPCResult<T> = Result<T, PRPCError>;

impl<T> From<PRPCResult<T>> for PRPCResponse<T> {
    fn from(result: PRPCResult<T>) -> Self {
        match result {
            Ok(response) => PRPCResponse {
                result: Some(response),
                error: None,
            },
            Err(error) => PRPCResponse {
                result: None,
                error: Some(error),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_result_to_response() {
        let result: PRPCResult<i32> = Ok(1);
        let response: PRPCResponse<i32> = result.into();
        assert_eq!(response.result, Some(1));
    }
}

// BELOW CODE NEEDS TO GO INTO CLIENT
// impl<T> PRPCResponse<T> {
//     pub fn is_valid(&self) -> bool {
//         (self.result.is_some() || self.error.is_some())
//             && !(self.error.is_some() && self.result.is_some())
//     }

//     pub fn to_result(self) -> PRPCClientError<PRPCResult<T>> {
//         if !self.is_valid() {
//             return Err(PRPCClientError {
//                 kind: PRPCClientErrorType::InvalidResponse,
//                 message: "Invalid response".to_string(),
//             });
//         }

//         if self.result.is_some() {
//             return Ok(Ok(self.result.unwrap()));
//         } else {
//             return Ok(Err(self.error.unwrap()));
//         }
//     }
// }
