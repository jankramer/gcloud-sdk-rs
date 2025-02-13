/// Payload proto for sensitiveaction.googleapis.com/action
/// Platform Log event that describes a sensitive action
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SensitiveAction {
    /// The type of action (eg. "change_organization_policy").
    #[prost(string, tag = "1")]
    pub action_type: ::prost::alloc::string::String,
    /// The time this action was detected.
    #[prost(message, optional, tag = "2")]
    pub action_time: ::core::option::Option<::prost_types::Timestamp>,
    /// GCP resources that are involved in the action.
    #[prost(string, repeated, tag = "3")]
    pub affected_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Cloud Logging entries that were used to identify the action.
    #[prost(message, repeated, tag = "4")]
    pub source_log_ids: ::prost::alloc::vec::Vec<sensitive_action::SourceLogId>,
    /// Link to documentation where you can learn more about sensitive actions.
    #[prost(string, tag = "5")]
    pub learn_more_uri: ::prost::alloc::string::String,
    /// Access details associated to the sensitive action.
    #[prost(message, optional, tag = "6")]
    pub access: ::core::option::Option<super::super::super::securitycenter::v1::Access>,
}
/// Nested message and enum types in `SensitiveAction`.
pub mod sensitive_action {
    /// Used to reference a specific Cloud Logging LogEntry.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourceLogId {
        /// The GCP resource (organization, folder, or project) that the LogEntry
        /// came from.
        #[prost(string, tag = "1")]
        pub resource_container: ::prost::alloc::string::String,
        /// The timestamp of the LogEntry.
        #[prost(message, optional, tag = "2")]
        pub log_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The insert_id of the LogEntry.
        #[prost(string, tag = "3")]
        pub insert_id: ::prost::alloc::string::String,
        /// A link to the Cloud Logging dashboard with a query for the LogEntry.
        #[prost(string, tag = "4")]
        pub query_uri: ::prost::alloc::string::String,
    }
}
