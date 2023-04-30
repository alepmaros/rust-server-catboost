use std::net::SocketAddr;

use hyper::{Method, StatusCode};
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ModelInput {
    feature1: f32,
    feature2: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelOutput {
    class: u8,
    proba: f32
}

async fn echo(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(full(
            "Try POSTing data to /echo",
        ))),
        (&Method::POST, "/echo") => {
            Ok(Response::new(req.into_body().boxed()))
        },
        (&Method::POST, "/invocations") => {
            let body = req.collect().await?.to_bytes();
            let input: ModelInput = serde_json::from_slice(&body).unwrap();
            println!("{:?}", input);

            let output: ModelOutput = ModelOutput{class: 0, proba: 0.0};
            let output_str = serde_json::to_string(&output).unwrap();

            Ok(Response::new(full(output_str)))
        }

        // Return 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(echo))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
