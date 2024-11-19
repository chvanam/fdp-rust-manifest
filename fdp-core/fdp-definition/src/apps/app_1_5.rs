#![doc = include_str!("../doc/app_1.md")]

#[fdp::definition]
pub mod definition {

    pub mod broadcasted_events {

        #[fdp::topic("app_1/random_number")]
        #[fdp::event]
        pub struct RandomNumber {
            pub value: i32,
        }

        #[fdp::topic("app_1/status_update")]
        #[fdp::event]
        pub struct StatusUpdate {
            pub status: String,
            pub timestamp: i64,
        }

        #[fdp::topic("app_1/sensor_data")]
        #[fdp::event]
        pub struct SensorData {
            pub temperature: f32,
            pub humidity: f32,
            pub pressure: f32,
        }

        #[fdp::topic("app_1/user_event")]
        #[fdp::event]
        pub struct UserEvent {
            pub user_id: String,
            pub event_type: String,
            pub details: String,
        }

        #[fdp::topic("app_1/system_metrics")]
        #[fdp::event]
        pub struct SystemMetrics {
            pub cpu_usage: f32,
            pub memory_usage: f32,
            pub disk_space: i64,
        }
    }

    pub mod listened_events {}

    pub mod incoming_requests {}

    pub mod outgoing_responses {}

    pub mod emitted_requests {}
}
