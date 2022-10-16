use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Output {
    pub request_id: String,
}

#[derive(Deserialize)]
pub struct DynamoDB {
    #[serde(rename = "NewImage")]
    pub new_image: serde_json::Value
}

#[derive(Deserialize)]
pub struct DynamoDBRecords {
    #[serde(rename = "eventID")]
    pub event_id: String,
    pub dynamodb: DynamoDB
}

#[derive(Deserialize)]
pub struct DynamoDBNewImageTrigger {
    #[serde(rename = "Records")]
    pub records: Vec<DynamoDBRecords>
}
