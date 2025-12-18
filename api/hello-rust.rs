use serde_json::json;
use vercel_runtime::{Error, Request, Body, Response, StatusCode, run, service_fn};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service = service_fn(handler);
    run(service).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "Hello Rust World"
            })
            .to_string()
            .into(),
        )?)
}