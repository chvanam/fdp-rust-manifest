// This file has access to ui.h members and can access them in unsafe blocks
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/netb_bindings.rs"));

use mqtt_client::MqttClient;
use std::{env, time::Duration};
use tokio::time::sleep;

/// Example typical usage of the MQTT clibrary in a standalone Rust application
/// How to tie this to the C code?
/// For example, when a C function "send_message" is called, it should add the message to the outgoing queue
/// then it should be picked up by the tokio runtime and sent to the MQTT broker somehow
/// Also, I want to be able to call a C function from the tokio runtime when a message is received in a particluar
/// callback, should the second queue be used for this? Or can i block the tokio runtime and call the C function directly?
pub async fn rustui() {
    // env::set_var("RUST_LOG", "debug");
    // env_logger::init();
    let (mut client, event_loop) = MqttClient::new("rustui", "localhost", 1883);
    client.publish("info", "(rustui) Connected").await;
    client.publish("start", "(rustui) Let's play! 🏓").await;

    let responder = client.clone();
    client.register_callback("ping", move |payload: String| {
        let responder = responder.clone();
        Box::pin(async move {
            sleep(Duration::from_secs(1)).await;
            println!("(rustui) Got {:?}, responding with C function", payload);
            unsafe {
                let answer = function_to_be_called_from_rustui();
                responder.publish("pong", format!("Pong with {}", answer)).await;
            }
        })
    }).await;

    client.start(event_loop).await;
}
