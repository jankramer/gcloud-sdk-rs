/// A service that is available for use by the consumer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// The resource name of the consumer and service.
    ///
    /// A valid name would be:
    /// - projects/123/services/serviceusage.googleapis.com
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The resource name of the consumer.
    ///
    /// A valid name would be:
    /// - projects/123
    #[prost(string, tag = "5")]
    pub parent: ::prost::alloc::string::String,
    /// The service configuration of the available service.
    /// Some fields may be filtered out of the configuration in responses to
    /// the `ListServices` method. These fields are present only in responses to
    /// the `GetService` method.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<ServiceConfig>,
    /// Whether or not the service has been enabled for use by the consumer.
    #[prost(enumeration = "State", tag = "4")]
    pub state: i32,
}
/// The configuration of the service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceConfig {
    /// The DNS address at which this service is available.
    ///
    /// An example DNS address would be:
    /// `calendar.googleapis.com`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The product title for this service.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// A list of API interfaces exported by this service. Contains only the names,
    /// versions, and method names of the interfaces.
    #[prost(message, repeated, tag = "3")]
    pub apis: ::prost::alloc::vec::Vec<::prost_types::Api>,
    /// Additional API documentation. Contains only the summary and the
    /// documentation URL.
    #[prost(message, optional, tag = "6")]
    pub documentation: ::core::option::Option<super::super::Documentation>,
    /// Quota configuration.
    #[prost(message, optional, tag = "10")]
    pub quota: ::core::option::Option<super::super::Quota>,
    /// Auth configuration. Contains only the OAuth rules.
    #[prost(message, optional, tag = "11")]
    pub authentication: ::core::option::Option<super::super::Authentication>,
    /// Configuration controlling usage of this service.
    #[prost(message, optional, tag = "15")]
    pub usage: ::core::option::Option<super::super::Usage>,
    /// Configuration for network endpoints. Contains only the names and aliases
    /// of the endpoints.
    #[prost(message, repeated, tag = "18")]
    pub endpoints: ::prost::alloc::vec::Vec<super::super::Endpoint>,
    /// Defines the monitored resources used by this service. This is required
    /// by the \[Service.monitoring][google.api.Service.monitoring\] and \[Service.logging][google.api.Service.logging\] configurations.
    #[prost(message, repeated, tag = "25")]
    pub monitored_resources: ::prost::alloc::vec::Vec<super::super::MonitoredResourceDescriptor>,
    /// Monitoring configuration.
    /// This should not include the 'producer_destinations' field.
    #[prost(message, optional, tag = "28")]
    pub monitoring: ::core::option::Option<super::super::Monitoring>,
}
/// The operation metadata returned for the batchend services operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The full name of the resources that this operation is directly
    /// associated with.
    #[prost(string, repeated, tag = "2")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Whether or not a service has been enabled for use by a consumer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// The default value, which indicates that the enabled state of the service
    /// is unspecified or not meaningful. Currently, all consumers other than
    /// projects (such as folders and organizations) are always in this state.
    Unspecified = 0,
    /// The service cannot be used by this consumer. It has either been explicitly
    /// disabled, or has never been enabled.
    Disabled = 1,
    /// The service has been explicitly enabled for use by this consumer.
    Enabled = 2,
}
/// Request message for the `EnableService` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableServiceRequest {
    /// Name of the consumer and service to enable the service on.
    ///
    /// The `EnableService` and `DisableService` methods currently only support
    /// projects.
    ///
    /// Enabling a service requires that the service is public or is shared with
    /// the user enabling the service.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com` where `123` is the
    /// project number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for the `EnableService` method.
/// This response message is assigned to the `response` field of the returned
/// Operation when that operation is done.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableServiceResponse {
    /// The new state of the service after enabling.
    #[prost(message, optional, tag = "1")]
    pub service: ::core::option::Option<Service>,
}
/// Request message for the `DisableService` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableServiceRequest {
    /// Name of the consumer and service to disable the service on.
    ///
    /// The enable and disable methods currently only support projects.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com` where `123` is the
    /// project number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates if services that are enabled and which depend on this service
    /// should also be disabled. If not set, an error will be generated if any
    /// enabled services depend on the service to be disabled. When set, the
    /// service, and any enabled services that depend on it, will be disabled
    /// together.
    #[prost(bool, tag = "2")]
    pub disable_dependent_services: bool,
    /// Defines the behavior for checking service usage when disabling a service.
    #[prost(enumeration = "disable_service_request::CheckIfServiceHasUsage", tag = "3")]
    pub check_if_service_has_usage: i32,
}
/// Nested message and enum types in `DisableServiceRequest`.
pub mod disable_service_request {
    /// Enum to determine if service usage should be checked when disabling a
    /// service.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CheckIfServiceHasUsage {
        /// When unset, the default behavior is used, which is SKIP.
        Unspecified = 0,
        /// If set, skip checking service usage when disabling a service.
        Skip = 1,
        /// If set, service usage is checked when disabling the service. If a
        /// service, or its dependents, has usage in the last 30 days, the request
        /// returns a FAILED_PRECONDITION error.
        Check = 2,
    }
}
/// Response message for the `DisableService` method.
/// This response message is assigned to the `response` field of the returned
/// Operation when that operation is done.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableServiceResponse {
    /// The new state of the service after disabling.
    #[prost(message, optional, tag = "1")]
    pub service: ::core::option::Option<Service>,
}
/// Request message for the `GetService` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Name of the consumer and service to get the `ConsumerState` for.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com` where `123` is the
    /// project number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `ListServices` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Parent to search for services on.
    ///
    /// An example name would be:
    /// `projects/123` where `123` is the project number.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested size of the next page of data.
    /// Requested page size cannot exceed 200.
    /// If not set, the default page size is 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token identifying which result to start with, which is returned by a
    /// previous list call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Only list services that conform to the given filter.
    /// The allowed filter strings are `state:ENABLED` and `state:DISABLED`.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for the `ListServices` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The available services for the requested project.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// Token that can be passed to `ListServices` to resume a paginated
    /// query.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the `BatchEnableServices` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchEnableServicesRequest {
    /// Parent to enable services on.
    ///
    /// An example name would be:
    /// `projects/123` where `123` is the project number.
    ///
    /// The `BatchEnableServices` method currently only supports projects.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The identifiers of the services to enable on the project.
    ///
    /// A valid identifier would be:
    /// serviceusage.googleapis.com
    ///
    /// Enabling services requires that each service is public or is shared with
    /// the user enabling the service.
    ///
    /// A single request can enable a maximum of 20 services at a time. If more
    /// than 20 services are specified, the request will fail, and no state changes
    /// will occur.
    #[prost(string, repeated, tag = "2")]
    pub service_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for the `BatchEnableServices` method.
/// This response message is assigned to the `response` field of the returned
/// Operation when that operation is done.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchEnableServicesResponse {
    /// The new state of the services after enabling.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// If allow_partial_success is true, and one or more services could not be
    /// enabled, this field contains the details about each failure.
    #[prost(message, repeated, tag = "2")]
    pub failures: ::prost::alloc::vec::Vec<batch_enable_services_response::EnableFailure>,
}
/// Nested message and enum types in `BatchEnableServicesResponse`.
pub mod batch_enable_services_response {
    /// Provides error messages for the failing services.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnableFailure {
        /// The service id of a service that could not be enabled.
        #[prost(string, tag = "1")]
        pub service_id: ::prost::alloc::string::String,
        /// An error message describing why the service could not be enabled.
        #[prost(string, tag = "2")]
        pub error_message: ::prost::alloc::string::String,
    }
}
/// Request message for the `BatchGetServices` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetServicesRequest {
    /// Parent to retrieve services from.
    /// If this is set, the parent of all of the services specified in `names` must
    /// match this field. An example name would be: `projects/123` where `123` is
    /// the project number. The `BatchGetServices` method currently only supports
    /// projects.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Names of the services to retrieve.
    ///
    /// An example name would be:
    /// `projects/123/services/serviceusage.googleapis.com` where `123` is the
    /// project number.
    /// A single request can get a maximum of 30 services at a time.
    #[prost(string, repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for the `BatchGetServices` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetServicesResponse {
    /// The requested Service states.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
}
#[doc = r" Generated client implementations."]
pub mod service_usage_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Enables services that service consumers want to use on Google Cloud Platform,"]
    #[doc = " lists the available or enabled services, or disables services that service"]
    #[doc = " consumers no longer use."]
    #[doc = ""]
    #[doc = " See [Service Usage API](https://cloud.google.com/service-usage/docs/overview)"]
    #[derive(Debug, Clone)]
    pub struct ServiceUsageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ServiceUsageClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ServiceUsageClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ServiceUsageClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Enable a service so that it can be used with a project."]
        pub async fn enable_service(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.serviceusage.v1.ServiceUsage/EnableService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Disable a service so that it can no longer be used with a project."]
        #[doc = " This prevents unintended usage that may cause unexpected billing"]
        #[doc = " charges or security leaks."]
        #[doc = ""]
        #[doc = " It is not valid to call the disable method on a service that is not"]
        #[doc = " currently enabled. Callers will receive a `FAILED_PRECONDITION` status if"]
        #[doc = " the target service is not currently enabled."]
        pub async fn disable_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.serviceusage.v1.ServiceUsage/DisableService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the service configuration and enabled state for a given service."]
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.serviceusage.v1.ServiceUsage/GetService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List all services available to the specified project, and the current"]
        #[doc = " state of those services with respect to the project. The list includes"]
        #[doc = " all public services, all services for which the calling user has the"]
        #[doc = " `servicemanagement.services.bind` permission, and all services that have"]
        #[doc = " already been enabled on the project. The list can be filtered to"]
        #[doc = " only include services in a specific state, for example to only include"]
        #[doc = " services enabled on the project."]
        #[doc = ""]
        #[doc = " WARNING: If you need to query enabled services frequently or across"]
        #[doc = " an organization, you should use"]
        #[doc = " [Cloud Asset Inventory"]
        #[doc = " API](https://cloud.google.com/asset-inventory/docs/apis), which provides"]
        #[doc = " higher throughput and richer filtering capability."]
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.serviceusage.v1.ServiceUsage/ListServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Enable multiple services on a project. The operation is atomic: if enabling"]
        #[doc = " any service fails, then the entire batch fails, and no state changes occur."]
        #[doc = " To enable a single service, use the `EnableService` method instead."]
        pub async fn batch_enable_services(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchEnableServicesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.serviceusage.v1.ServiceUsage/BatchEnableServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the service configurations and enabled states for a given list of"]
        #[doc = " services."]
        pub async fn batch_get_services(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetServicesRequest>,
        ) -> Result<tonic::Response<super::BatchGetServicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.serviceusage.v1.ServiceUsage/BatchGetServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
