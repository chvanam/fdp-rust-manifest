#![doc = include_str!("../doc/app_1.md")]

#[fdp::definition]
pub mod definition {

    pub mod broadcasted_events {

        #[fdp::topic("app_1/random_number_broadcast")]
        #[fdp::event]
        pub struct RandomNumber {
            pub value: i32,
        }
    }

    pub mod listened_events {}

    pub mod incoming_requests {}

    pub mod outgoing_responses {}

    pub mod emitted_requests {}
}
