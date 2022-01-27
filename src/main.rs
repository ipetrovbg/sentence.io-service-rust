mod models;

use lambda_runtime::Context;
use lambda_runtime::Error;
use crate::models::{Event, Output};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}

async fn handler(event: Event, context: Context) -> Result<Output, Error> {
    println!("{}", event.message);
    Ok(Output {
        message: format!("v3 {}", event.message),
        request_id: context.request_id,
    })
}