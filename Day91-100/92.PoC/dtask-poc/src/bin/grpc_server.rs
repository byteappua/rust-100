use tonic::{transport::Server, Request, Response, Status};
use echo::echo_service_server::{EchoService, EchoServiceServer};
use echo::{EchoRequest, EchoResponse};

pub mod echo {
    tonic::include_proto!("echo");
}

#[derive(Debug, Default)]
pub struct MyEcho;

#[tonic::async_trait]
impl EchoService for MyEcho {
    async fn echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = EchoResponse {
            message: format!("Echo: {}", request.into_inner().message),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let echo = MyEcho::default();

    println!("EchoServer listening on {}", addr);

    Server::builder()
        .add_service(EchoServiceServer::new(echo))
        .serve(addr)
        .await?;

    Ok(())
}
