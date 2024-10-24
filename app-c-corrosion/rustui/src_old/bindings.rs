use crate::Message;
use fdp_lib_rust::{Filter, PayloadA, NetbIncomingPayloads};

use crate::{init, send_message};

#[no_mangle]
pub extern "C" fn rustui_init() {
    init();
}

#[no_mangle]
pub extern "C" fn send_first_message() {
    let topic = "rustui/netb/hello".to_string();

    let payload = NetbIncomingPayloads::PayloadA(PayloadA {
        value: 4,
        origin: "message from netb".to_string(),
        filter: Filter::All,
        reply_to: "rustui/netb/reply".to_string(),
    });

    let message = Message { topic, payload };

    send_message(message);
}
