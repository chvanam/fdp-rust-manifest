use crate::implementation::handle_message;
use rumqttc::{Client, Connection, MqttOptions, QoS};
use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Mutex, OnceLock,
    },
    thread,
};

pub type Payloads = fdp_lib_rust::NetbIncomingPayloads;

#[derive(Debug)]
pub struct Message {
    topic: String,
    payload: Payloads,
}

/// Exposes the rust bindings we want to use from NETB
//pub mod bindings;

//pub mod implementation;

/// A queue holding the outgoing messages to be sent to the MQTT broker from NETB
static OUTGOING: OnceLock<Mutex<Sender<Message>>> = OnceLock::new();

/// A queue holding the incoming messages from the MQTT broker, to be dispatched to NETB
static INCOMING: OnceLock<Mutex<Sender<Message>>> = OnceLock::new();

/// Initializes rustui by creating a outbound and incoming channel for messages between MQTT and NETB
pub fn init() {
    let (outgoing_sender, outgoing_receiver) = std::sync::mpsc::channel::<Message>();
    let (incoming_sender, incoming_receiver) = std::sync::mpsc::channel::<Message>();
    OUTGOING.set(Mutex::new(outgoing_sender)).unwrap();
    INCOMING.set(Mutex::new(incoming_sender)).unwrap();
    thread::spawn(|| rustui_main_thread(outgoing_receiver, incoming_receiver));
    println!("🦀 Initialized rustui outgoing/incoming channels and spawned rustui thread!");
}

/// The rustui thread connects to the MQTT broker and runs an event-loop to publish/subscribe to it's messages.
fn rustui_main_thread(outgoing_receiver: Receiver<Message>, incoming_receiver: Receiver<Message>) {
    println!("🦀 Rustui thread spawned");

    let mqttoptions = MqttOptions::new("netb-rustui", "localhost", 1883);
    let (mut client, connection) = rumqttc::Client::new(mqttoptions, 10);

    client.subscribe("rustui/#", QoS::AtLeastOnce).unwrap();

    thread::spawn(move || handle_rumqtt_event_loop(connection));
    thread::spawn(|| handle_incoming_messages(incoming_receiver));
    handle_outgoing_messages(outgoing_receiver, client);
}

fn handle_rumqtt_event_loop(mut connection: Connection) {
    for event in connection.iter() {
        match event.unwrap() {
            rumqttc::Event::Incoming(packet) => match packet {
                rumqttc::Packet::Publish(publish) => {
                    let now = std::time::Instant::now();
                    let vec_u8_payload = publish.payload.to_vec();
                    let payload: Result<Payloads, serde_json::Error> =
                        serde_json::from_slice(&vec_u8_payload[..]);
                    let elapsed = now.elapsed();
                    println!("🦀 Validated payload in {:.2?}", elapsed);

                    match payload {
                        Ok(payload) => {
                            let topic = publish.topic;
                            let message = Message { topic, payload };
                            let incoming_sender =
                                INCOMING.get().and_then(|mutex| mutex.lock().ok()).unwrap();
                            incoming_sender.send(message).unwrap();
                        }
                        Err(_) => println!("🦀 Unknown payload received, throwing away"),
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}

fn handle_outgoing_messages(outgoing_receiver: Receiver<Message>, mut client: Client) {
    while let Ok(message) = outgoing_receiver.recv() {
        println!("🦀 NETB -> MQTT : {:?}", message);
        let topic = message.topic;
        let payload = serde_json::to_vec(&message.payload).unwrap();
        client
            .publish(topic, QoS::AtLeastOnce, false, payload)
            .unwrap();
    }
}

fn handle_incoming_messages(incoming_receiver: Receiver<Message>) {
    while let Ok(message) = incoming_receiver.recv() {
        println!("🦀 MQTT -> NETB : {:?}", message);
        handle_message(message);
    }
}

fn send_message(message: Message) {
    let outgoing_sender = OUTGOING.get().and_then(|mutex| mutex.lock().ok()).unwrap();
    outgoing_sender.send(message).unwrap();
}
