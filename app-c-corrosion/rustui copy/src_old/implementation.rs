#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/netb_bindings.rs"));

// This file has access to all functions defined in ui.h and can call them in unsafe blocks

use crate::Message;

pub fn handle_message(message: Message) {
    match message.payload {
        _ => todo!(),
    }
}
