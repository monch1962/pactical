use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Consumer {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Provider {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PactSpecification {
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    pact_specification: PactSpecification,
}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HeaderString {
    header: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Message {
    description: String,
    provider_state: String,
    contents: String,
    meta_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Messages {
    message: Vec<Option<Message>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Request {
    method: String,
    path: String,
    headers: Option<serde_json::Value>,
    query: Option<serde_json::Value>,
    body: Option<serde_json::Value>,
    matching_rules: Option<serde_json::Value>,
    generators: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    status: Option<u16>,
    headers: Option<serde_json::Value>,
    body: Option<serde_json::Value>,
    generators: Option<serde_json::Value>,
    matching_rules: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Interaction {
    description: String,
    provider_state: Option<serde_json::Value>,
    request: Option<Request>,
    response: Option<Response>,
    messages: Option<Messages>,
    tags: Option<Vec<String>>, // Added this non-standard extension to Pact to let me select subsets of contracts to execute
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pact {
    consumer: Consumer,
    pub provider: Provider,
    interactions: Vec<Interaction>,
    metadata: Metadata,
}
