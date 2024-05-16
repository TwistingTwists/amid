use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

pub mod pb {
    tonic::include_proto!("hello");
}

use pb::{HelloReply, HelloRequest};

#[derive(Default)]
pub struct GreeterService {}

#[tonic::async_trait]
impl pb::greeter_server::Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let request = request.into_inner();
        let reply = HelloReply {
            message: format!("Hello, {}!", request.name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 50051));

    let greeter_service = GreeterService::default();

    let svc = pb::greeter_server::GreeterServer::new(greeter_service);

    let server = Server::builder().add_service(svc).serve(addr);

    println!("gRPC server listening on {}", addr);

    server.await?;

    Ok(())
}

// fn main() {
//     println!("Hello, world!");
// }
