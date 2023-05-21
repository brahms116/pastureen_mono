use prpc_core::*;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;

pub enum PRPCMiddlewareResponse {
    Next(Value),
    Return(PRPCResult<Value>),
}

type PRPCMiddleware = Box<dyn FnMut(Value) -> Box<dyn Future<Output = PRPCMiddlewareResponse>>>;

type PRPCAuthCommand =
    Box<dyn FnMut(String, Value) -> Box<dyn Future<Output = PRPCResponse<Value>>>>;

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
        F: FnMut(String, T) -> Box<dyn Future<Output = K>> + 'static,
        K: Into<PRPCResult<E>> + 'static,
        T: DeserializeOwned,
        E: Serialize,
    {
        let my_handler: PRPCAuthCommand = Box::new(move |id: String, params: Value| {
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
    pub async fn handle(&mut self, command: String, params: Value) -> PRPCResponse<Value> {
        let mut middleware_params = params;
        for middleware in self.middlewares.iter_mut() {
            let res = middleware(middleware_params.clone());
            let res = Box::into_pin(res);
            let res = res.await;
            match res {
                PRPCMiddlewareResponse::Return(res) => {
                    return res.into();
                }
                PRPCMiddlewareResponse::Next(params) => {
                    middleware_params = params;
                }
            }
        }

        let handler = self.commands.get_mut(&command);
        if let Some(handler_func) = handler {
            let res = handler_func(middleware_params);
            let res = Box::into_pin(res);
            let res = res.await;
            return res;
        }

        let handler = self.authenticated_commands.get_mut(&command);
        if let Some(handler_func) = handler {
            //TODO: Add authentication
            let res = handler_func("".to_string(), middleware_params);
            let res = Box::into_pin(res);
            let res = res.await;
            return res;
        }

        return Err(PRPCError {
            kind: PRPCErrorType::NotFound,
            message: "Command not found".to_string(),
        }).into();
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

