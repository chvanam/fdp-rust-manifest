use fdp_common::mqtt::Message;

#[fdp::topic("test/topic")]
struct TestMessage {
    field: String,
}

fn main() {
    assert_eq!(TestMessage::topic(), "test/topic");
    assert_eq!(
        TestMessage::schema(),
        schemars::schema_for!(TestMessage).schema
    );
}
