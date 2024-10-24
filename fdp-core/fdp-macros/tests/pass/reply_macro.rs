mod outgoing_responses {
    #[fdp::topic("test/reply")]
    pub struct Acknowledge;
}

#[fdp::topic("test/topic")]
#[fdp::replies_with(outgoing_responses::Acknowledge)]
struct TestRequest {
    data: i32,
}

fn main() {
    type InferedResponse = <TestRequest as fdp_common::mqtt::Request>::Response;

    assert_eq!(
        std::any::TypeId::of::<InferedResponse>(),
        std::any::TypeId::of::<outgoing_responses::Acknowledge>()
    );
}
