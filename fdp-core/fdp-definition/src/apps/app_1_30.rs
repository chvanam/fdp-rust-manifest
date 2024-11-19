#![doc = include_str!("../doc/app_1.md")]

#[fdp::definition]
pub mod definition {

    pub mod broadcasted_events {

        #[fdp::topic("app_1/sensor/temperature")]
        #[fdp::event]
        pub struct Temperature {
            pub celsius: f32,
            pub location: String,
        }

        #[fdp::topic("app_1/sensor/humidity")]
        #[fdp::event]
        pub struct Humidity {
            pub percentage: f32,
            pub location: String,
        }

        #[fdp::topic("app_1/sensor/pressure")]
        #[fdp::event]
        pub struct Pressure {
            pub hectopascals: f32,
            pub location: String,
        }

        #[fdp::topic("app_1/user/login")]
        #[fdp::event]
        pub struct UserLogin {
            pub user_id: String,
            pub timestamp: i64,
        }

        #[fdp::topic("app_1/user/logout")]
        #[fdp::event]
        pub struct UserLogout {
            pub user_id: String,
            pub timestamp: i64,
        }

        #[fdp::topic("app_1/system/cpu")]
        #[fdp::event]
        pub struct CpuMetrics {
            pub usage_percent: f32,
            pub temperature: f32,
        }

        #[fdp::topic("app_1/system/memory")]
        #[fdp::event]
        pub struct MemoryMetrics {
            pub used_mb: i32,
            pub total_mb: i32,
        }

        #[fdp::topic("app_1/system/disk")]
        #[fdp::event]
        pub struct DiskMetrics {
            pub used_gb: i32,
            pub total_gb: i32,
        }

        #[fdp::topic("app_1/network/bandwidth")]
        #[fdp::event]
        pub struct NetworkBandwidth {
            pub upload_mbps: f32,
            pub download_mbps: f32,
        }

        #[fdp::topic("app_1/network/latency")]
        #[fdp::event]
        pub struct NetworkLatency {
            pub milliseconds: f32,
            pub destination: String,
        }

        #[fdp::topic("app_1/alerts/critical")]
        #[fdp::event]
        pub struct CriticalAlert {
            pub message: String,
            pub timestamp: i64,
        }

        #[fdp::topic("app_1/alerts/warning")]
        #[fdp::event]
        pub struct WarningAlert {
            pub message: String,
            pub timestamp: i64,
        }

        #[fdp::topic("app_1/database/query_time")]
        #[fdp::event]
        pub struct DatabaseQueryTime {
            pub query_id: String,
            pub milliseconds: f32,
        }

        #[fdp::topic("app_1/database/connections")]
        #[fdp::event]
        pub struct DatabaseConnections {
            pub active: i32,
            pub idle: i32,
        }

        #[fdp::topic("app_1/cache/hit_rate")]
        #[fdp::event]
        pub struct CacheHitRate {
            pub percentage: f32,
            pub cache_name: String,
        }
    }

    pub mod listened_events {
        pub use crate::apps::app_2_30::broadcasted_events::ApiLatency;
        pub use crate::apps::app_2_30::broadcasted_events::AppHealth;
        pub use crate::apps::app_2_30::broadcasted_events::BackupStatus;
        pub use crate::apps::app_2_30::broadcasted_events::BatchJobStatus;
        pub use crate::apps::app_2_30::broadcasted_events::ConfigUpdate;
        pub use crate::apps::app_2_30::broadcasted_events::ErrorRate;
        pub use crate::apps::app_2_30::broadcasted_events::FileSystemStatus;
        pub use crate::apps::app_2_30::broadcasted_events::LoadBalancerStatus;
        pub use crate::apps::app_2_30::broadcasted_events::LogEvent;
        pub use crate::apps::app_2_30::broadcasted_events::MessageQueueStatus;
        pub use crate::apps::app_2_30::broadcasted_events::SecurityAlert;
        pub use crate::apps::app_2_30::broadcasted_events::ServiceStatus;
        pub use crate::apps::app_2_30::broadcasted_events::TaskCompletion;
        pub use crate::apps::app_2_30::broadcasted_events::UserActivity;
        pub use crate::apps::app_2_30::broadcasted_events::WebhookDelivery;
    }

    pub mod incoming_requests {}

    pub mod outgoing_responses {}

    pub mod emitted_requests {}
}
