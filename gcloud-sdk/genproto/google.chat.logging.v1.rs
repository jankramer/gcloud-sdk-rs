/// JSON payload of error messages. If the Cloud Logging API is enabled, these
/// error messages are logged to
/// [Google Cloud Logging](<https://cloud.google.com/logging/docs>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatAppLogEntry {
    /// The deployment that caused the error. For Chat bots built in Apps Script,
    /// this is the deployment ID defined by Apps Script.
    #[prost(string, tag = "1")]
    pub deployment: ::prost::alloc::string::String,
    /// The error code and message.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The unencrypted `callback_method` name that was running when the error was
    /// encountered.
    #[prost(string, tag = "3")]
    pub deployment_function: ::prost::alloc::string::String,
}
