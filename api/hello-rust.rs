use serde_json::{Value, json};
use vercel_runtime::{Error, Request, run, service_fn};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service = service_fn(handler);
    run(service).await
}


async fn handler(_req: Request) -> Result<Value, Error> {
    Ok(json!({
        "message": "Hello Rust World",
    }))
}