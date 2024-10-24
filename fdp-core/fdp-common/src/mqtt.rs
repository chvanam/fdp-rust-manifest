//! Traits and types to interact with an MQTT client

use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};

/// A Message within the FDP system can be serialized/deserialized to JSON and has a topic.
/// It can represent an event, a request or a response
pub trait Message: Serialize + DeserializeOwned + JsonSchema + Send + Sync + 'static {
    /// The static topic the message is sent on
    fn topic() -> &'static str;

    /// The JSON schema of the message
    fn schema() -> schemars::schema::SchemaObject {
        schemars::schema_for!(Self).schema
    }
}

/// Events are broadcasted from FDP apps, and can be listened to by other apps.
pub trait Event: Message {}

/// Requests to a FDP app require a corresponding response.
pub trait Request: Message {
    /// The associated response type required for the request
    type Response: Message;
}
