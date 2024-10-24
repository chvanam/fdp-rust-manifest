//! The raw information extracted from the declarative Rust app definitions, that are created by the fdp-definition-macros

use schemars::schema::RootSchema;
use std::collections::HashMap;

/// Represents the information extracted about the whole FDP system
#[derive(Debug)]
pub struct SystemDefinitionInfo {
    pub apps: HashMap<String, AppDefinitionInfo>,
}

impl SystemDefinitionInfo {
    pub fn from(app_definitions: Vec<(String, AppDefinitionInfo)>) -> Self {
        let apps = app_definitions.into_iter().collect::<HashMap<_, _>>();
        SystemDefinitionInfo { apps }
    }
}

/// Represents the information extracted from a FDP app definition
#[derive(Debug)]
pub struct AppDefinitionInfo {
    // Message declarations
    pub broadcasted_events: Vec<MessageDeclarationInfo>,
    pub incoming_requests: Vec<MessageDeclarationInfo>,
    pub outgoing_responses: Vec<MessageDeclarationInfo>,
    // Message references
    pub listened_events: Vec<MessageReferenceInfo>,
    pub emitted_requests: Vec<MessageReferenceInfo>,
}

/// Represents the information available from the Rust code for a message declaration
/// extracted from a 'pub struct Identifier { ... }' that implements the Message trait
#[derive(Debug)]
pub struct MessageDeclarationInfo {
    /// The identifier of the message
    pub identifier: String,
    /// The topic of the message
    pub topic: String,
    /// The JSON schema of the message
    pub schema: RootSchema,
}

/// Representes the information available from the Rust code for a message reference
/// extracted from a `pub use crate::apps::<app_name>::<submodule>::<Identifier>;` statement.
#[derive(Debug)]
pub struct MessageReferenceInfo {
    /// The non-renamed identifier of the message
    pub identifier: String,
    /// The name of the app that the message is defined in
    pub app_name: String,
    /// The submodule in which the message is defined
    pub module: String,
}
