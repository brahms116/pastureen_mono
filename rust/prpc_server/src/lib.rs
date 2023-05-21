use prpc_core::*;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;

pub enum PRPCMiddlewareResponse {
    Next(PRPCRequest<Value>),
    Return(PRPCResult<Value>),
}

type PRPCMiddleware = Box<dyn FnMut(PRPCRequest<Value>) -> Box<dyn Future<Output = PRPCMiddlewareResponse>>>;

type PRPCAuthCommand =
    Box<dyn FnMut(&str, Value) -> Box<dyn Future<Output = PRPCResponse<Value>>>>;

type PRPCCommand = Box<dyn FnMut(Value) -> Box<dyn Future<Output = PRPCResponse<Value>>>>;

pub struct PRPCServerBuilder {
    middlewares: Vec<PRPCMiddleware>,
    authenticated_commands: HashMap<String, PRPCAuthCommand>,
    commands: HashMap<String, PRPCCommand>,
}

impl PRPCServerBuilder {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
            authenticated_commands: HashMap::new(),
            commands: HashMap::new(),
        }
    }

    pub fn use_middleware(&mut self, middleware: PRPCMiddleware) {
        self.middlewares.push(middleware);
    }

    pub fn use_command<F, T, K, E>(&mut self, command: &str, mut handler: F)
    where
        F: FnMut(T) -> Box<dyn Future<Output = K>> + 'static,
        K: Into<PRPCResult<E>> + 'static,
        T: DeserializeOwned,
        E: Serialize,
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
                let result = result.into();
                let result = serde_json::to_value(result);
                if result.is_err() {
                    let err = PRPCError {
                        kind: PRPCErrorType::Internal,
                        message: "Failed to serialize result into JSON".to_string(),
                    };

                    let res: PRPCResponse<Value> = Err(err).into();
                    return res;
                }
                let res: PRPCResponse<Value> = Ok(result.unwrap()).into();
                res
            };
            Box::new(fut)
        });

        self.commands.insert(command.to_string(), my_handler);
    }

    pub fn use_authenticated_command<F, T, K, E>(&mut self, command: &str, mut handler: F)
    where
        F: FnMut(&str, T) -> Box<dyn Future<Output = K>> + 'static,
        K: Into<PRPCResult<E>> + 'static,
        T: DeserializeOwned,
        E: Serialize,
    {
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
                let result = serde_json::to_value(result);
                if result.is_err() {
                    let err = PRPCError {
                        kind: PRPCErrorType::Internal,
                        message: "Failed to serialize result into JSON".to_string(),
                    };

                    let res: PRPCResponse<Value> = Err(err).into();
                    return res;
                }
                let res: PRPCResponse<Value> = Ok(result.unwrap()).into();
                res
            };
            Box::new(fut)
        });

        self.authenticated_commands
            .insert(command.to_string(), my_handler);
    }

    pub fn build(self) -> PRPCServer {
        PRPCServer {
            middlewares: self.middlewares,
            authenticated_commands: self.authenticated_commands,
            commands: self.commands,
        }
    }
}

pub struct PRPCServer {
    middlewares: Vec<PRPCMiddleware>,
    authenticated_commands: HashMap<String, PRPCAuthCommand>,
    commands: HashMap<String, PRPCCommand>,
}

impl PRPCServer {
    pub async fn handle(&mut self, input: Value) -> PRPCResponse<Value> {
        let req = serde_json::from_value::<PRPCRequest<Value>>(input);
        if req.is_err() {
            let err = PRPCError {
                kind: PRPCErrorType::InvalidArgument,
                message: "Not a valid prpc request".to_string(),
            };
            return Err(err).into();
        }
        let mut req = req.unwrap();

        for middleware in self.middlewares.iter_mut() {
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

        let handler = self.commands.get_mut(&req.command);
        if let Some(func) = handler {
            let fut = func(req.params);
            let fut = Box::into_pin(fut);
            let fut = fut.await;
            return fut;
        }

        let handler = self.authenticated_commands.get_mut(&req.command);
        if let Some(func) = handler {
            // TODO: Authentication
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
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TestParams {
        a: i32,
        b: i32,
    }

    enum TestError {
        TestError,
    }
}
