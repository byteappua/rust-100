use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::{Full, BodyExt};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Method, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

// We define a generic "Body" type for our responses to avoid complex type signatures.
type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResultResponse = Result<Response<Full<Bytes>>, GenericError>;

async fn hello(req: Request<hyper::body::Incoming>) -> ResultResponse {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::new(Full::new(Bytes::from("Hello, World! Welcome to Day 40!"))))
        },
        (&Method::POST, "/echo") => {
            // Read the whole body into bytes
            let upper = req.collect().await?.to_bytes();
            Ok(Response::new(Full::new(upper)))
        },
        (&Method::GET, "/echo") => {
             Ok(Response::new(Full::new(Bytes::from("Try POSTing to /echo"))))
        },
        _ => {
            let mut not_found = Response::new(Full::new(Bytes::from("404 Not Found")));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    println!("--- Day 40: Async Web Server with Hyper ---");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function into a `Service`
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
