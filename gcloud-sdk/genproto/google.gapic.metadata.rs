/// Metadata about a GAPIC library for a specific combination of API, version, and
/// computer language.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GapicMetadata {
    /// Schema version of this proto. Current value: 1.0
    #[prost(string, tag = "1")]
    pub schema: ::prost::alloc::string::String,
    /// Any human-readable comments to be included in this file.
    #[prost(string, tag = "2")]
    pub comment: ::prost::alloc::string::String,
    /// Computer language of this generated language. This must be
    /// spelled out as it spoken in English, with no capitalization or
    /// separators (e.g. "csharp", "nodejs").
    #[prost(string, tag = "3")]
    pub language: ::prost::alloc::string::String,
    /// The proto package containing the API definition for which this
    /// GAPIC library was generated.
    #[prost(string, tag = "4")]
    pub proto_package: ::prost::alloc::string::String,
    /// The language-specific library package for this GAPIC library.
    #[prost(string, tag = "5")]
    pub library_package: ::prost::alloc::string::String,
    /// A map from each proto-defined service to ServiceForTransports,
    /// which allows listing information about transport-specific
    /// implementations of the service.
    ///
    /// The key is the name of the service as it appears in the .proto
    /// file.
    #[prost(map = "string, message", tag = "6")]
    pub services: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        gapic_metadata::ServiceForTransport,
    >,
}
/// Nested message and enum types in `GapicMetadata`.
pub mod gapic_metadata {
    /// A map from a transport name to ServiceAsClient, which allows
    /// listing information about the client objects that implement the
    /// parent RPC service for the specified transport.
    ///
    /// The key name is the transport, lower-cased with no separators
    /// (e.g. "grpc", "rest").
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServiceForTransport {
        #[prost(map = "string, message", tag = "1")]
        pub clients: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ServiceAsClient,
        >,
    }
    /// Information about a specific client implementing a proto-defined service.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServiceAsClient {
        /// The name of the library client formatted as it appears in the source code
        #[prost(string, tag = "1")]
        pub library_client: ::prost::alloc::string::String,
        /// A mapping from each proto-defined RPC name to the the list of
        /// methods in library_client that implement it. There can be more
        /// than one library_client method for each RPC. RPCs with no
        /// library_client methods need not be included.
        ///
        /// The key name is the name of the RPC as defined and formated in
        /// the proto file.
        #[prost(map = "string, message", tag = "2")]
        pub rpcs: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            MethodList,
        >,
    }
    /// List of GAPIC client methods implementing the proto-defined RPC
    /// for the transport and service specified in the containing
    /// structures.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MethodList {
        /// List of methods for a specific proto-service client in the
        /// GAPIC. These names should be formatted as they appear in the
        /// source code.
        #[prost(string, repeated, tag = "1")]
        pub methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
