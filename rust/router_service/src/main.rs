use lambda_runtime::{service_fn, LambdaEvent};
use prpc_core::PRPCResult;
use prpc_server::PRPCServerBuilder;
use prpc_server_lambda_wrapper::PRPCLambdaWrapper;
use serde_json::Value;
use std::future::Future;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}

async fn hello(_params: Value) -> PRPCResult<()> {
    println!("Hello World");
    Ok(())
}

fn hello_wrapper(params: Value) -> Box<dyn Future<Output = PRPCResult<()>>> {
    Box::new(hello(params))
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let mut server = PRPCServerBuilder::new();
    server.use_command("hello", hello_wrapper);
    let server = server.build();
    let server_wrapper = PRPCLambdaWrapper::new(server);
    let result = server_wrapper.handle_event(event).await?;
    Ok(result)
}
