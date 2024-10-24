use std::{ffi::CStr, sync::OnceLock, thread};
use tokio::{
    runtime::Runtime,
    sync::mpsc::{unbounded_channel, UnboundedSender},
};

use crate::implementation::rustui;

mod implementation;

// TODO: pick a message structure
pub struct Message;

/// Entry point of the rustui library. It sets up the MQTT client
/// and the queues for the messages. It should not block.
#[no_mangle]
pub extern "C" fn rustui_init() {
    println!("Hello from Rust!");
    init();
}

#[no_mangle]
pub extern "C" fn send_message_to_rustui() {
    // add an outgoing message to the outgoing queue
    // TODO: make Rust functions exposeable to C, what API to use?
}

/// Messages from rustui to the MQTT client
static OUTGOING: OnceLock<UnboundedSender<Message>> = OnceLock::new();

/// Called by the C code to initialize the rustui library
/// It should not block and start a tokio runtime
pub fn init() {
    let (outgoing_sender, _outgoing_receiver) = unbounded_channel::<Message>();
    OUTGOING.set(outgoing_sender).unwrap();
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            rustui().await;
        });
    });
    println!("🦀 Initialized rustui outgoing/incoming channels and spawned rustui thread!");
}
