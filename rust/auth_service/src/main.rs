use std::net::SocketAddr;

use hyper:: {
    Response as HyperResponse,
    Request as HyperRequest,
    Body
};

use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {

        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

async fn health_check(_req: HyperRequest<Body>) -> Result<HyperResponse<Body>, StdError> {
    Ok(HyperResponse::new(Body::from("OK")))
}

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let greeter = MyGreeter::default();

    let health_addr: SocketAddr = "127.0.0.1:8081".parse()?;
    let service_fn = hyper::service::make_service_fn(|_| async {
        Ok::<_, StdError>(hyper::service::service_fn(health_check))
    });

    let server = hyper::Server::bind(&health_addr).serve(service_fn);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }


    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
