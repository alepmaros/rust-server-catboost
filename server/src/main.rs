use std::net::SocketAddr;

use hyper::{Method, StatusCode};
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;
use serde::{Serialize, Deserialize};
use catboost;

#[derive(Serialize, Deserialize, Debug)]
struct ModelInput {
    f1: f32,
    f2: f32,
    f3: f32,
    f4: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelOutput {
    proba: Vec<f64>
}

fn sigmoid(x: f64) -> f64 {
    1. / (1. + (-x).exp())
}

fn predictor(model_input: ModelInput) -> ModelOutput {
    // TODO: This should not be loaded every time a thread spawn?
    let model = catboost::Model::load("model.cbm").unwrap();

    let float_feat = vec![model_input.f1, model_input.f2, model_input.f3, model_input.f4];
    let cat_feat: Vec<String> = vec![];

    let mut prediction = model.calc_model_prediction(vec![float_feat], vec![cat_feat]).unwrap();
    
    // Apply sigmoid function to convert to probabilities
    prediction = prediction.iter().map(|x| sigmoid(*x)).collect();

    ModelOutput{proba: prediction}
}

async fn handler(
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

            // TODO: Better Error Handling so it does not panic on wrong request?
            let model_input: ModelInput = serde_json::from_slice(&body).unwrap();

            let output: ModelOutput = predictor(model_input);
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
                .serve_connection(stream, service_fn(handler))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
