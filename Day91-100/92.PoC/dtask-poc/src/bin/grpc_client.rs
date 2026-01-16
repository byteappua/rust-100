use echo::echo_service_client::EchoServiceClient;
use echo::EchoRequest;

pub mod echo {
    tonic::include_proto!("echo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EchoServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(EchoRequest {
        message: "Hello Tonic".into(),
    });

    let response = client.echo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
