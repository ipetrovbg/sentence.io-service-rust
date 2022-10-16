mod models;

use lambda_runtime::{Error, LambdaEvent, service_fn, run};
use crate::models::{DynamoDBNewImageTrigger, Output};

use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();

    let handler = service_fn(handler);
    run(handler).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<DynamoDBNewImageTrigger>) -> Result<Output, Error> {
    let (event, context) = event.into_parts();
    info!("DynamoDB new record: {}", event.dynamodb.new_image);

    Ok(Output {
        message: event.dynamodb.new_image,
        request_id: context.request_id,
    })
}