#![doc = include_str!("../doc/app_2.md")]

#[fdp::definition]
pub mod definition {
    pub mod broadcasted_events {}

    pub mod listened_events {
        pub use crate::apps::app_1::broadcasted_events::RandomNumber;
    }

    pub mod incoming_requests {}

    pub mod outgoing_responses {}

    pub mod emitted_requests {}
}
