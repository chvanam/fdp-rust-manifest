#![doc = include_str!("../doc/app_2.md")]

#[fdp::definition]
pub mod definition {
    pub mod broadcasted_events {
        #[fdp::topic("app_2/service/status")]
        #[fdp::event]
        pub struct ServiceStatus {
            pub service_name: String,
            pub status: String,
        }

        #[fdp::topic("app_2/api/latency")]
        #[fdp::event]
        pub struct ApiLatency {
            pub endpoint: String,
            pub response_time_ms: f32,
        }

        #[fdp::topic("app_2/error/rate")]
        #[fdp::event]
        pub struct ErrorRate {
            pub service: String,
            pub errors_per_minute: f32,
        }

        #[fdp::topic("app_2/batch/job_status")]
        #[fdp::event]
        pub struct BatchJobStatus {
            pub job_id: String,
            pub status: String,
            pub progress: f32,
        }

        #[fdp::topic("app_2/user/activity")]
        #[fdp::event]
        pub struct UserActivity {
            pub user_id: String,
            pub action: String,
            pub timestamp: i64,
        }

        #[fdp::topic("app_2/config/update")]
        #[fdp::event]
        pub struct ConfigUpdate {
            pub component: String,
            pub new_value: String,
        }

        #[fdp::topic("app_2/security/alert")]
        #[fdp::event]
        pub struct SecurityAlert {
            pub severity: String,
            pub description: String,
        }

        #[fdp::topic("app_2/task/completion")]
        #[fdp::event]
        pub struct TaskCompletion {
            pub task_id: String,
            pub success: bool,
        }

        #[fdp::topic("app_2/log/event")]
        #[fdp::event]
        pub struct LogEvent {
            pub level: String,
            pub message: String,
        }

        #[fdp::topic("app_2/health/status")]
        #[fdp::event]
        pub struct AppHealth {
            pub healthy: bool,
            pub message: String,
        }

        #[fdp::topic("app_2/filesystem/status")]
        #[fdp::event]
        pub struct FileSystemStatus {
            pub path: String,
            pub space_available_gb: f32,
        }

        #[fdp::topic("app_2/backup/status")]
        #[fdp::event]
        pub struct BackupStatus {
            pub backup_id: String,
            pub success: bool,
            pub size_mb: i32,
        }

        #[fdp::topic("app_2/queue/status")]
        #[fdp::event]
        pub struct MessageQueueStatus {
            pub queue_name: String,
            pub message_count: i32,
        }

        #[fdp::topic("app_2/loadbalancer/status")]
        #[fdp::event]
        pub struct LoadBalancerStatus {
            pub healthy_nodes: i32,
            pub total_nodes: i32,
        }

        #[fdp::topic("app_2/webhook/delivery")]
        #[fdp::event]
        pub struct WebhookDelivery {
            pub webhook_id: String,
            pub success: bool,
        }
    }

    pub mod listened_events {
        pub use crate::apps::app_1_30::broadcasted_events::CacheHitRate;
        pub use crate::apps::app_1_30::broadcasted_events::CpuMetrics;
        pub use crate::apps::app_1_30::broadcasted_events::CriticalAlert;
        pub use crate::apps::app_1_30::broadcasted_events::DatabaseConnections;
        pub use crate::apps::app_1_30::broadcasted_events::DatabaseQueryTime;
        pub use crate::apps::app_1_30::broadcasted_events::DiskMetrics;
        pub use crate::apps::app_1_30::broadcasted_events::Humidity;
        pub use crate::apps::app_1_30::broadcasted_events::MemoryMetrics;
        pub use crate::apps::app_1_30::broadcasted_events::NetworkBandwidth;
        pub use crate::apps::app_1_30::broadcasted_events::NetworkLatency;
        pub use crate::apps::app_1_30::broadcasted_events::Pressure;
        pub use crate::apps::app_1_30::broadcasted_events::Temperature;
        pub use crate::apps::app_1_30::broadcasted_events::UserLogin;
        pub use crate::apps::app_1_30::broadcasted_events::UserLogout;
        pub use crate::apps::app_1_30::broadcasted_events::WarningAlert;
    }

    pub mod incoming_requests {}
    pub mod outgoing_responses {}
    pub mod emitted_requests {}
}
