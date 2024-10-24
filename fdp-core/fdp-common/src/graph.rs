//! Used to validate the extracted information and construct a graph representation of the FDP system

use crate::info::{AppDefinitionInfo, MessageDeclarationInfo, SystemDefinitionInfo};
use petgraph::{
    dot::{Config, Dot},
    graph::{DiGraph, NodeIndex},
    visit::NodeRef,
};
use schemars::schema::RootSchema;
use std::{collections::HashMap, fmt::Debug};

/// The FdpSystem holds a graph representation of the FDP system and it's messages producer/consumer relationship
pub struct FdpSystem {
    pub graph: DiGraph<FdpApp, FdpMessage>,
    pub index_map: HashMap<String, NodeIndex>,
}

/// A FdpApp represents an application within the FDP system
#[derive(Debug)]
pub struct FdpApp {
    pub name: String,
}

/// A FdpMessage represents a message passing through the MQTT broker in the FDP system
#[derive(Debug)]
pub struct FdpMessage {
    pub name: String,
    pub message_type: MessageType,
    pub topic: String,
    pub schema: RootSchema,
}

/// A message going through the MQTT broker can be only one of these types
#[derive(Debug)]
pub enum MessageType {
    Event,
    Request,
    Response,
}

impl FdpSystem {
    pub fn from(definition_info: SystemDefinitionInfo) -> Result<Self, String> {
        let mut graph = DiGraph::new();
        let mut index_map = HashMap::new();

        // Create nodes for each app
        for (app_name, _) in &definition_info.apps {
            let index = graph.add_node(FdpApp {
                name: app_name.clone(),
            });
            index_map.insert(app_name.clone(), index);
        }

        // Create edges based on message definitions and references
        // We want to go through the message declaration modules, and match them against message references modules
        // to create associated edges.

        // For a message reference, we also want to confirm that the name, submodule match.
        // We also want to be able to list any messages that are not used
        // TODO: dissalow self references within the same app

        // For each app definition
        for (app_name, info) in &definition_info.apps {
            let app_index = index_map[app_name];

            // We first handle the broadcast messages declaration submodule
            for broadcast_message_declaration in &info.broadcasted_events {
                // We now check all other message references in other apps to see if there is a match
                let mut listened = false;
                for (other_app_name, other_info) in &definition_info.apps {
                    // We skip the current app for self references
                    if other_app_name != app_name
                        && other_info.references_message_as(app_name, broadcast_message_declaration)
                    {
                        // Another app references this broadcast message declaration, it's an edge
                        let target_index = index_map[other_app_name];
                        graph.add_edge(
                            app_index,
                            target_index,
                            FdpMessage {
                                name: broadcast_message_declaration.identifier.clone(),
                                message_type: MessageType::Event,
                                topic: broadcast_message_declaration.topic.clone(),
                                schema: broadcast_message_declaration.schema.clone(),
                            },
                        );
                        listened = true;
                    }
                }
                if !listened {
                    return Err(format!(
                        "Broadcast message '{}' on topic '{}' is never listened to by any other app.",
                        broadcast_message_declaration.identifier, broadcast_message_declaration.topic
                    ));
                }
            }

            // We then handle incoming_requests / responses bi-directional edges
            for incoming_requests_declaration in &info.incoming_requests {
                let mut handled = false;
                for (other_app_name, other_info) in &definition_info.apps {
                    if other_app_name != app_name
                        && other_info.references_request_as(app_name, incoming_requests_declaration)
                    {
                        // An incoming_request is references in another apps emitted_requests, we add the according edges
                        handled = true;
                        // We add the request
                        let target_index = index_map[other_app_name];
                        graph.add_edge(
                            target_index,
                            app_index,
                            FdpMessage {
                                name: incoming_requests_declaration.identifier.clone(),
                                message_type: MessageType::Request,
                                topic: incoming_requests_declaration.topic.clone(),
                                schema: incoming_requests_declaration.schema.clone(),
                            },
                        );

                        // We find the associated response

                        // And the associated response
                        // graph.add_edge(
                        //     app_index,
                        //     target_index,
                        //     FdpMessage {
                        //         name: incoming_requests_declaration.response.identifier.clone(),
                        //         message_type: MessageType::Response,
                        //         topic: "TODO".to_string(),
                        //         schema: incoming_requests_declaration.schema.clone(),
                        //     },
                        // );
                    }
                }
                if !handled {
                    return Err(format!(
                        "Request '{}' on topic '{}' is never handled by any other app.",
                        incoming_requests_declaration.identifier,
                        incoming_requests_declaration.topic
                    ));
                }
            }
        }

        Ok(FdpSystem { graph, index_map })
    }

    pub fn to_graphviz(&self) -> String {
        let dot = Dot::with_attr_getters(
            &self.graph,
            &[Config::NodeNoLabel, Config::EdgeNoLabel],
            &|_, er| {
                let fdp_message = er.weight();
                let text = match fdp_message.message_type {
                    MessageType::Event => format!("Broadcasts: {}", fdp_message.name),
                    MessageType::Request => format!("Handles: {}", fdp_message.name),
                    MessageType::Response => format!("Replies with {}", fdp_message.name),
                };

                format!("label = \"{}\"", text)
            },
            &|_, nr| format!("label = \"{}\"", nr.weight().name),
        );

        format!("{:?}", dot)
    }
}

impl AppDefinitionInfo {
    /// Returns true if the current AppDefinitionInfo with a given app_name references
    /// a given MessageDeclarationInfo as a message type (Event or Request)
    pub fn references_message_as(
        &self,
        declaring_app_name: &str,
        message_declaration: &MessageDeclarationInfo,
    ) -> bool {
        // println!(
        //     "Checking if app {} references message {} declared in app {}",
        //     declaring_app_name, message_declaration.identifier, declaring_app_name
        // );
        for message_reference in &self.listened_events {
            if message_reference.app_name == declaring_app_name
                && message_reference.identifier == message_declaration.identifier
            {
                return true;
            }
        }
        false
    }

    pub fn references_request_as(
        &self,
        self_app_name: &str,
        request_declaration: &MessageDeclarationInfo,
    ) -> bool {
        for request_reference in &self.emitted_requests {
            if request_reference.app_name == self_app_name
                && request_reference.identifier == request_declaration.identifier
            {
                return true;
            }
        }
        return false;
    }
}
