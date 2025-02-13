/// Admin Console legacy audit log.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditData {
    /// Text description of the admin event.
    /// This is the "Event" column in Admin Console's Admin Logs.
    #[prost(string, tag = "1")]
    pub event_message: ::prost::alloc::string::String,
    /// Arbitrary event data.
    /// This is the "Result" column in Admin Console's Admin Logs.
    #[prost(map = "string, string", tag = "2")]
    pub event_data: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
