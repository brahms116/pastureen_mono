use jsonwebtoken::{decode, DecodingKey, Validation};
use prpc_core::*;
use serde::de::DeserializeOwned;
use serde::ser::Serialize as SerializeTrait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug)]

enum PRPCServerInternalError {
    InvalidToken,
    #[allow(dead_code)]
    Unknown,
}

impl std::fmt::Display for PRPCServerInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidToken => write!(f, "Invalid token"),
            _ => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for PRPCServerInternalError {}

#[derive(Debug)]
pub enum PRPCServerError {
    TokenSecretNotSet,
    Unknown,
}

impl std::fmt::Display for PRPCServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenSecretNotSet => write!(f, "Token secret not set"),
            _ => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for PRPCServerError {}

fn decode_token(token: &str, secret: &str) -> Result<String, PRPCServerInternalError> {
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );

    if token.is_err() {
        return Err(PRPCServerInternalError::InvalidToken);
    }

    let token = token.unwrap();
    Ok(token.claims.sub)
}

fn turn_response_result_to_value<T>(
    result: PRPCResult<T>,
) -> Result<PRPCResponse<Value>, serde_json::Error>
where
    T: SerializeTrait,
{
    match result {
        Ok(response) => Ok(PRPCResponse {
            result: Some(serde_json::to_value(response)?),
            error: None,
        }),
        Err(error) => Ok(PRPCResponse {
            result: None,
            error: Some(error),
        }),
    }
}

pub enum PRPCMiddlewareResponse {
    Next(PRPCRequest<Value>),
    Return(PRPCResult<Value>),
}

type PRPCMiddleware =
    Box<dyn Fn(PRPCRequest<Value>) -> Box<dyn Future<Output = PRPCMiddlewareResponse>>>;

type PRPCAuthCommand = Box<dyn Fn(&str, Value) -> Box<dyn Future<Output = PRPCResponse<Value>>>>;

type PRPCCommand = Box<dyn Fn(Value) -> Box<dyn Future<Output = PRPCResponse<Value>>>>;

pub struct PRPCServerBuilder {
    middlewares: Vec<PRPCMiddleware>,
    authenticated_commands: HashMap<String, PRPCAuthCommand>,
    commands: HashMap<String, PRPCCommand>,
    token_secret: Option<String>,
}

impl PRPCServerBuilder {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
            authenticated_commands: HashMap::new(),
            commands: HashMap::new(),
            token_secret: None,
        }
    }

    pub fn use_middleware(&mut self, middleware: PRPCMiddleware) {
        self.middlewares.push(middleware);
    }

    pub fn use_command<F, T, K, E>(&mut self, command: &str, handler: F)
    where
        F: Fn(T) -> Box<dyn Future<Output = K>> + 'static,
        K: Into<PRPCResult<E>> + 'static,
        T: DeserializeOwned,
        E: SerializeTrait,
    {
        let my_handler: PRPCCommand = Box::new(move |params: Value| {
            let args = serde_json::from_value::<T>(params);
            if args.is_err() {
                let err = PRPCError {
                    kind: PRPCErrorType::InvalidArgument,
                    message: "Invalid arguments".to_string(),
                };

                let res: PRPCResponse<Value> = Err(err).into();
                return Box::new(async { res });
            }
            let args = args.unwrap();

            let fut = handler(args);
            let fut = Box::into_pin(fut);
            let fut = async move {
                let result = fut.await;
                let result: PRPCResult<E> = result.into();
                let response = turn_response_result_to_value(result);
                if response.is_err() {
                    let err = PRPCError {
                        kind: PRPCErrorType::Internal,
                        message: "Failed to serialize json".to_string(),
                    };
                    let res: PRPCResponse<Value> = Err(err).into();
                    return res;
                }
                response.unwrap()
            };
            Box::new(fut)
        });

        self.commands.insert(command.to_string(), my_handler);
    }

    pub fn set_token_secret(&mut self, secret: &str) {
        self.token_secret = Some(secret.to_string());
    }

    pub fn use_authenticated_command<F, T, K, E>(
        &mut self,
        command: &str,
        handler: F,
    ) -> Result<(), PRPCServerError>
    where
        F: Fn(&str, T) -> Box<dyn Future<Output = K>> + 'static,
        K: Into<PRPCResult<E>> + 'static,
        T: DeserializeOwned,
        E: Serialize,
    {
        if let None = self.token_secret {
            return Err(PRPCServerError::TokenSecretNotSet);
        }

        let my_handler: PRPCAuthCommand = Box::new(move |id: &str, params: Value| {
            let args = serde_json::from_value::<T>(params);
            if args.is_err() {
                let err = PRPCError {
                    kind: PRPCErrorType::InvalidArgument,
                    message: "Invalid arguments".to_string(),
                };

                let res: PRPCResponse<Value> = Err(err).into();
                return Box::new(async { res });
            }
            let args = args.unwrap();

            let fut = handler(id, args);
            let fut = Box::into_pin(fut);
            let fut = async move {
                let result = fut.await;
                let result = result.into();
                let response = turn_response_result_to_value(result);
                if response.is_err() {
                    let err = PRPCError {
                        kind: PRPCErrorType::Internal,
                        message: "Failed to serialize json".to_string(),
                    };
                    let res: PRPCResponse<Value> = Err(err).into();
                    return res;
                }
                response.unwrap()
            };
            Box::new(fut)
        });

        self.authenticated_commands
            .insert(command.to_string(), my_handler);
        Ok(())
    }

    pub fn build(self) -> PRPCServer {
        PRPCServer {
            middlewares: self.middlewares,
            authenticated_commands: self.authenticated_commands,
            commands: self.commands,
            token_secret: self.token_secret,
        }
    }
}

pub struct PRPCServer {
    middlewares: Vec<PRPCMiddleware>,
    authenticated_commands: HashMap<String, PRPCAuthCommand>,
    commands: HashMap<String, PRPCCommand>,
    token_secret: Option<String>,
}

impl PRPCServer {
    pub async fn handle(&self, input: Value) -> PRPCResponse<Value> {
        let req = serde_json::from_value::<PRPCRequest<Value>>(input);
        if req.is_err() {
            let err = PRPCError {
                kind: PRPCErrorType::InvalidArgument,
                message: "Not a valid prpc request".to_string(),
            };
            return Err(err).into();
        }
        let mut req = req.unwrap();

        for middleware in self.middlewares.iter() {
            let fut = middleware(req);
            let fut = Box::into_pin(fut);
            let fut = fut.await;
            match fut {
                PRPCMiddlewareResponse::Next(value) => {
                    req = value;
                }
                PRPCMiddlewareResponse::Return(value) => {
                    return value.into();
                }
            }
        }

        let handler = self.commands.get(&req.command);
        if let Some(func) = handler {
            let fut = func(req.params);
            let fut = Box::into_pin(fut);
            let fut = fut.await;
            return fut;
        }

        let handler = self.authenticated_commands.get(&req.command);
        if let Some(func) = handler {
            if let None = req.auth {
                let err = PRPCError {
                    kind: PRPCErrorType::Unauthenticated,
                    message: "Unauthenticated".to_string(),
                };
                return Err(err).into();
            }

            if let None = self.token_secret {
                let err = PRPCError {
                    kind: PRPCErrorType::Internal,
                    message: "Credentials not set on server".to_string(),
                };
                return Err(err).into();
            }

            let token = req.auth.unwrap();
            let secret = self.token_secret.as_ref().unwrap();

            let id = decode_token(&token, secret);

            if let Err(e) = id {
                let err = match e {
                    PRPCServerInternalError::InvalidToken => PRPCError {
                        kind: PRPCErrorType::Unauthenticated,
                        message: "Invalid token".to_string(),
                    },
                    PRPCServerInternalError::Unknown => PRPCError {
                        kind: PRPCErrorType::Internal,
                        message: "Could not decode token".to_string(),
                    },
                };

                return Err(err).into();
            }

            let fut = func("USER_ID_HERE", req.params);
            let fut = Box::into_pin(fut);
            let fut = fut.await;
            return fut;
        }

        let err = PRPCError {
            kind: PRPCErrorType::NotFound,
            message: "Command not found".to_string(),
        };
        Err(err).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize)]
    struct TestParams {
        a: i32,
        b: i32,
    }

    // #[derive(Serialize)]
    // enum TestError {
    //     TestError,
    // }

    // impl Into<PRPCError> for TestError {
    //     fn into(self) -> PRPCError {
    //         PRPCError {
    //             kind: PRPCErrorType::Internal,
    //             message: "Test error".to_string(),
    //         }
    //     }
    // }

    async fn test_function(params: TestParams) -> PRPCResult<i32> {
        Ok(params.a + params.b).into()
    }

    fn test_function_wrapper(params: TestParams) -> Box<dyn Future<Output = PRPCResult<i32>>> {
        let result = Box::new(test_function(params));
        result
    }

    // Invalid shape

    // Invalid command

    // Invalid argument

    // Correct command
    #[tokio::test]
    async fn test_correct_command() {
        let mut builder = PRPCServerBuilder::new();
        builder.use_command("test", test_function_wrapper);
        let server = builder.build();
        let response = server
            .handle(json!(
                {
                    "command": "test",
                    "params": {
                        "a": 1,
                        "b": 2
                    }
                }
            ))
            .await;
        if let Some(num) = response.result {
            assert_eq!(num, json!(3));
        } else {
            panic!("No result");
        }
    }

    // Correct authenticated command

    // Correct middleware

    // Unauthenticated error

    // Wrong signature error

    // Wrong expiry error
}
