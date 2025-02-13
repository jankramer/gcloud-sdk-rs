/// Logged event relating to a specific secret
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretEvent {
    /// Resource name of the secret in the format `projects/*/secrets/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Type of event that is being logged for the secret
    #[prost(enumeration = "secret_event::EventType", tag = "2")]
    pub r#type: i32,
    /// Human readable message describing the event
    #[prost(string, tag = "3")]
    pub log_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SecretEvent`.
pub mod secret_event {
    /// Describes the type of event that is being logged. All logs have exactly one
    /// EventType.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EventType {
        /// An unrecognized event type. Should never be used.
        Unspecified = 0,
        /// The secret is scheduled to expire in 30 days.
        ExpiresIn30Days = 1,
        /// The secret is scheduled to expire in 7 days.
        ExpiresIn7Days = 2,
        /// The secret is scheduled to expire in 1 day.
        ExpiresIn1Day = 3,
        /// The secret is scheduled to expire in 6 hours.
        ExpiresIn6Hours = 4,
        /// The secret is scheduled to expire in 1 hour.
        ExpiresIn1Hour = 5,
        /// The secret's expire-time has passed and it has expired.
        Expired = 6,
        /// A Pub/Sub topic configured on the secret could not be found.
        TopicNotFound = 7,
        /// A Pub/Sub topic configured on the secret does not have the needed
        /// permissions. The Secret Manager P4SA must be granted
        /// 'pubsub.topic.publish' permission (or 'roles/pubsub.publisher') on the
        /// topic.
        TopicPermissionDenied = 8,
    }
    impl EventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventType::Unspecified => "EVENT_TYPE_UNSPECIFIED",
                EventType::ExpiresIn30Days => "EXPIRES_IN_30_DAYS",
                EventType::ExpiresIn7Days => "EXPIRES_IN_7_DAYS",
                EventType::ExpiresIn1Day => "EXPIRES_IN_1_DAY",
                EventType::ExpiresIn6Hours => "EXPIRES_IN_6_HOURS",
                EventType::ExpiresIn1Hour => "EXPIRES_IN_1_HOUR",
                EventType::Expired => "EXPIRED",
                EventType::TopicNotFound => "TOPIC_NOT_FOUND",
                EventType::TopicPermissionDenied => "TOPIC_PERMISSION_DENIED",
            }
        }
    }
}
