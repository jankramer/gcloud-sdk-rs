#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatoryInterceptAckLogEntry {
    /// The id of the user that triggered the Regulatory Intercept.
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// The id of the GCP resource associated with the Assured Workload applicable
    /// to the request. Must be of the format
    /// //cloudresourcemanager.googleapis.com/{type}/{id}
    #[prost(string, tag = "2")]
    pub assured_workload_resource_id: ::prost::alloc::string::String,
}
