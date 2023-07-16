use lambda_runtime::LambdaEvent;
use prpc_server::*;
use serde_json::{json, Value};

pub struct PRPCLambdaWrapper {
    server: PRPCServer,
}

#[derive(Debug)]
pub enum PRPCLambdaWrapperErr {
    UnableToSerializeResult,
    UnableToDeserializeBody,
    MissingBodyIntHttpRequest,
}

impl std::fmt::Display for PRPCLambdaWrapperErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnableToSerializeResult => write!(f, "Unable to serialize result"),
            Self::MissingBodyIntHttpRequest => write!(f, "Missing body in http request"),
            Self::UnableToDeserializeBody => write!(f, "Unable to deserialize body"),
        }
    }
}

impl std::error::Error for PRPCLambdaWrapperErr {}

impl PRPCLambdaWrapper {
    pub fn new(server: PRPCServer) -> Self {
        PRPCLambdaWrapper { server }
    }

    pub async fn handle_event(
        &self,
        event: LambdaEvent<Value>,
    ) -> Result<Value, PRPCLambdaWrapperErr> {
        let (event, _context) = event.into_parts();
        println!("Event: {:?}", event);
        let body = event["body"].as_str();
        if let Some(body) = body {
            println!("Body: {:?}", body);
            let body = serde_json::from_str(body)
                .map_err(|_| PRPCLambdaWrapperErr::UnableToDeserializeBody)?;
            let result = self.server.handle(body).await;
            let value = serde_json::to_value(result)
                .map_err(|_| PRPCLambdaWrapperErr::UnableToSerializeResult)?;
            return Ok(lambda_http_response(value));
        }
        let result = self.server.handle(event).await;
        let value = serde_json::to_value(result)
            .map_err(|_| PRPCLambdaWrapperErr::UnableToSerializeResult)?;
        Ok(value)
    }
}

fn lambda_http_response(value: Value) -> Value {
    let response = json!({
        "statusCode": 200,
        "headers": {
            "Content-Type": "application/json"
        },
        "body": value
    });
    response
}

#[cfg(test)]
mod test {
    use super::*;
    use lambda_runtime::Context;
    use prpc_core::*;
    use serde::Deserialize;
    use std::future::Future;

    #[derive(Debug, PartialEq, Deserialize)]
    struct Numbers {
        a: i32,
        b: i32,
    }

    async fn add(params: Numbers) -> PRPCResult<i32> {
        Ok(params.a + params.b)
    }

    fn add_wrapper(params: Numbers) -> Box<dyn Future<Output = PRPCResult<i32>>> {
        Box::new(add(params))
    }

    fn get_wrapper() -> PRPCLambdaWrapper {
        let mut server = PRPCServerBuilder::new();
        server.use_command("add", add_wrapper);
        let server = server.build();
        let server_wrapper = PRPCLambdaWrapper::new(server);
        server_wrapper
    }

    // Test handling lambda event
    #[tokio::test]
    async fn test_handling_lambda_event() {
        let event = json!({
            "command": "add",
            "params": {
                "a": 2,
                "b": 3
            }
        });
        let event = LambdaEvent::new(event, Context::default());
        let server_wrapper = get_wrapper();
        let result = server_wrapper.handle_event(event).await.unwrap();
        let result = result["result"].as_i64().unwrap();
        assert_eq!(result, 5);
    }

    // Test handling http lambda event
    #[tokio::test]
    async fn test_handling_http_lambda_event() {
        let event = json!({
            "body":"{\"command\":\"add\",\"params\":{\"a\":2,\"b\":3}}"
        });
        let event = LambdaEvent::new(event, Context::default());
        let server_wrapper = get_wrapper();
        let result = server_wrapper.handle_event(event).await.unwrap();
        let result = result["body"].as_object().unwrap()["result"]
            .as_i64()
            .unwrap();
        assert_eq!(result, 5);
    }

    // Test handling incorrect lambda http event
}
