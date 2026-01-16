use dtask::dtask::scheduler_client::SchedulerClient;
use dtask::dtask::SubmitTaskRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SchedulerClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SubmitTaskRequest {
        payload: "Hello from client".into(),
    });

    let response = client.submit_task(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
