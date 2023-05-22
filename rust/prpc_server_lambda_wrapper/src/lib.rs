use lambda_runtime::LambdaEvent;
use prpc_server::*;
use serde_json::{Value, json};

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

    pub async fn handle_lambda_event(
        &mut self,
        event: LambdaEvent<Value>,
    ) -> Result<Value, PRPCLambdaWrapperErr> {
        let (event, _context) = event.into_parts();
        let result = self.server.handle(event).await;
        let value = serde_json::to_value(result)
            .map_err(|_| PRPCLambdaWrapperErr::UnableToSerializeResult)?;
        Ok(value)
    }

    pub async fn handle_http_lambda_event(
        &mut self,
        event: LambdaEvent<Value>,
    ) -> Result<Value, PRPCLambdaWrapperErr> {
        let (event, _context) = event.into_parts();
        let body = event["body"]
            .as_str()
            .ok_or(PRPCLambdaWrapperErr::MissingBodyIntHttpRequest)?;
        let body = serde_json::from_str(body)
            .map_err(|_| PRPCLambdaWrapperErr::UnableToDeserializeBody)?;
        let result = self.server.handle(body).await;
        let value = serde_json::to_value(result)
            .map_err(|_| PRPCLambdaWrapperErr::UnableToSerializeResult)?;
        Ok(lambda_http_response(value))
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
    // Test handling lambda event

    // Test handling http lambda event
    
    // Test handling incorrect lambda http event
}
