/// HTTP request.
///
/// The task will be pushed to the worker as an HTTP request. If the worker
/// or the redirected worker acknowledges the task by returning a successful HTTP
/// response code ([`200` - `299`]), the task will be removed from the queue. If
/// any other HTTP response code is returned or no response is received, the
/// task will be retried according to the following:
///
/// * User-specified throttling: [retry configuration][google.cloud.tasks.v2.Queue.retry_config],
///   [rate limits][google.cloud.tasks.v2.Queue.rate_limits], and the [queue's state][google.cloud.tasks.v2.Queue.state].
///
/// * System throttling: To prevent the worker from overloading, Cloud Tasks may
///   temporarily reduce the queue's effective rate. User-specified settings
///   will not be changed.
///
///  System throttling happens because:
///
///   * Cloud Tasks backs off on all errors. Normally the backoff specified in
///     [rate limits][google.cloud.tasks.v2.Queue.rate_limits] will be used. But if the worker returns
///     `429` (Too Many Requests), `503` (Service Unavailable), or the rate of
///     errors is high, Cloud Tasks will use a higher backoff rate. The retry
///     specified in the `Retry-After` HTTP response header is considered.
///
///   * To prevent traffic spikes and to smooth sudden increases in traffic,
///     dispatches ramp up slowly when the queue is newly created or idle and
///     if large numbers of tasks suddenly become available to dispatch (due to
///     spikes in create task rates, the queue being unpaused, or many tasks
///     that are scheduled at the same time).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequest {
    /// Required. The full url path that the request will be sent to.
    ///
    /// This string must begin with either "http://" or "https://". Some examples
    /// are: `http://acme.com` and `https://acme.com/sales:8080`. Cloud Tasks will
    /// encode some characters for safety and compatibility. The maximum allowed
    /// URL length is 2083 characters after encoding.
    ///
    /// The `Location` header response from a redirect response [`300` - `399`]
    /// may be followed. The redirect is not counted as a separate attempt.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// The HTTP method to use for the request. The default is POST.
    #[prost(enumeration = "HttpMethod", tag = "2")]
    pub http_method: i32,
    /// HTTP request headers.
    ///
    /// This map contains the header field names and values.
    /// Headers can be set when the
    /// [task is created][google.cloud.tasks.v2beta3.CloudTasks.CreateTask].
    ///
    /// These headers represent a subset of the headers that will accompany the
    /// task's HTTP request. Some HTTP request headers will be ignored or replaced.
    ///
    /// A partial list of headers that will be ignored or replaced is:
    ///
    /// * Host: This will be computed by Cloud Tasks and derived from
    ///   [HttpRequest.url][google.cloud.tasks.v2.HttpRequest.url].
    /// * Content-Length: This will be computed by Cloud Tasks.
    /// * User-Agent: This will be set to `"Google-Cloud-Tasks"`.
    /// * X-Google-*: Google use only.
    /// * X-AppEngine-*: Google use only.
    ///
    /// `Content-Type` won't be set by Cloud Tasks. You can explicitly set
    /// `Content-Type` to a media type when the
    ///  [task is created][google.cloud.tasks.v2beta3.CloudTasks.CreateTask].
    ///  For example, `Content-Type` can be set to `"application/octet-stream"` or
    ///  `"application/json"`.
    ///
    /// Headers which can have multiple values (according to RFC2616) can be
    /// specified using comma-separated values.
    ///
    /// The size of the headers must be less than 80KB.
    #[prost(map = "string, string", tag = "3")]
    pub headers:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// HTTP request body.
    ///
    /// A request body is allowed only if the
    /// [HTTP method][google.cloud.tasks.v2.HttpRequest.http_method] is POST, PUT, or PATCH. It is an
    /// error to set body on a task with an incompatible [HttpMethod][google.cloud.tasks.v2.HttpMethod].
    #[prost(bytes = "vec", tag = "4")]
    pub body: ::prost::alloc::vec::Vec<u8>,
    /// The mode for generating an `Authorization` header for HTTP requests.
    ///
    /// If specified, all `Authorization` headers in the [HttpRequest.headers][google.cloud.tasks.v2.HttpRequest.headers]
    /// field will be overridden.
    #[prost(oneof = "http_request::AuthorizationHeader", tags = "5, 6")]
    pub authorization_header: ::core::option::Option<http_request::AuthorizationHeader>,
}
/// Nested message and enum types in `HttpRequest`.
pub mod http_request {
    /// The mode for generating an `Authorization` header for HTTP requests.
    ///
    /// If specified, all `Authorization` headers in the [HttpRequest.headers][google.cloud.tasks.v2.HttpRequest.headers]
    /// field will be overridden.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AuthorizationHeader {
        /// If specified, an
        /// [OAuth token](https://developers.google.com/identity/protocols/OAuth2)
        /// will be generated and attached as an `Authorization` header in the HTTP
        /// request.
        ///
        /// This type of authorization should generally only be used when calling
        /// Google APIs hosted on *.googleapis.com.
        #[prost(message, tag = "5")]
        OauthToken(super::OAuthToken),
        /// If specified, an
        /// [OIDC](https://developers.google.com/identity/protocols/OpenIDConnect)
        /// token will be generated and attached as an `Authorization` header in the
        /// HTTP request.
        ///
        /// This type of authorization can be used for many scenarios, including
        /// calling Cloud Run, or endpoints where you intend to validate the token
        /// yourself.
        #[prost(message, tag = "6")]
        OidcToken(super::OidcToken),
    }
}
/// App Engine HTTP request.
///
/// The message defines the HTTP request that is sent to an App Engine app when
/// the task is dispatched.
///
/// Using [AppEngineHttpRequest][google.cloud.tasks.v2.AppEngineHttpRequest] requires
/// [`appengine.applications.get`](https://cloud.google.com/appengine/docs/admin-api/access-control)
/// Google IAM permission for the project
/// and the following scope:
///
/// `https://www.googleapis.com/auth/cloud-platform`
///
/// The task will be delivered to the App Engine app which belongs to the same
/// project as the queue. For more information, see
/// [How Requests are
/// Routed](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed)
/// and how routing is affected by
/// [dispatch
/// files](https://cloud.google.com/appengine/docs/python/config/dispatchref).
/// Traffic is encrypted during transport and never leaves Google datacenters.
/// Because this traffic is carried over a communication mechanism internal to
/// Google, you cannot explicitly set the protocol (for example, HTTP or HTTPS).
/// The request to the handler, however, will appear to have used the HTTP
/// protocol.
///
/// The [AppEngineRouting][google.cloud.tasks.v2.AppEngineRouting] used to construct the URL that the task is
/// delivered to can be set at the queue-level or task-level:
///
/// * If [app_engine_routing_override is set on the
///    queue][Queue.app_engine_routing_override], this value is used for all
///    tasks in the queue, no matter what the setting is for the [task-level
///    app_engine_routing][AppEngineHttpRequest.app_engine_routing].
///
///
/// The `url` that the task will be sent to is:
///
/// * `url =` [host][google.cloud.tasks.v2.AppEngineRouting.host] `+`
///   [relative_uri][google.cloud.tasks.v2.AppEngineHttpRequest.relative_uri]
///
/// Tasks can be dispatched to secure app handlers, unsecure app handlers, and
/// URIs restricted with
/// [`login:
/// admin`](https://cloud.google.com/appengine/docs/standard/python/config/appref).
/// Because tasks are not run as any user, they cannot be dispatched to URIs
/// restricted with
/// [`login:
/// required`](https://cloud.google.com/appengine/docs/standard/python/config/appref)
/// Task dispatches also do not follow redirects.
///
/// The task attempt has succeeded if the app's request handler returns an HTTP
/// response code in the range [`200` - `299`]. The task attempt has failed if
/// the app's handler returns a non-2xx response code or Cloud Tasks does
/// not receive response before the [deadline][google.cloud.tasks.v2.Task.dispatch_deadline]. Failed
/// tasks will be retried according to the
/// [retry configuration][google.cloud.tasks.v2.Queue.retry_config]. `503` (Service Unavailable) is
/// considered an App Engine system error instead of an application error and
/// will cause Cloud Tasks' traffic congestion control to temporarily throttle
/// the queue's dispatches. Unlike other types of task targets, a `429` (Too Many
/// Requests) response from an app handler does not cause traffic congestion
/// control to throttle the queue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppEngineHttpRequest {
    /// The HTTP method to use for the request. The default is POST.
    ///
    /// The app's request handler for the task's target URL must be able to handle
    /// HTTP requests with this http_method, otherwise the task attempt will fail
    /// with error code 405 (Method Not Allowed). See
    /// [Writing a push task request
    /// handler](https://cloud.google.com/appengine/docs/java/taskqueue/push/creating-handlers#writing_a_push_task_request_handler)
    /// and the documentation for the request handlers in the language your app is
    /// written in e.g.
    /// [Python Request
    /// Handler](https://cloud.google.com/appengine/docs/python/tools/webapp/requesthandlerclass).
    #[prost(enumeration = "HttpMethod", tag = "1")]
    pub http_method: i32,
    /// Task-level setting for App Engine routing.
    ///
    /// * If [app_engine_routing_override is set on the
    ///    queue][Queue.app_engine_routing_override], this value is used for all
    ///    tasks in the queue, no matter what the setting is for the [task-level
    ///    app_engine_routing][AppEngineHttpRequest.app_engine_routing].
    #[prost(message, optional, tag = "2")]
    pub app_engine_routing: ::core::option::Option<AppEngineRouting>,
    /// The relative URI.
    ///
    /// The relative URI must begin with "/" and must be a valid HTTP relative URI.
    /// It can contain a path and query string arguments.
    /// If the relative URI is empty, then the root path "/" will be used.
    /// No spaces are allowed, and the maximum length allowed is 2083 characters.
    #[prost(string, tag = "3")]
    pub relative_uri: ::prost::alloc::string::String,
    /// HTTP request headers.
    ///
    /// This map contains the header field names and values.
    /// Headers can be set when the
    /// [task is created][google.cloud.tasks.v2.CloudTasks.CreateTask].
    /// Repeated headers are not supported but a header value can contain commas.
    ///
    /// Cloud Tasks sets some headers to default values:
    ///
    /// * `User-Agent`: By default, this header is
    ///   `"AppEngine-Google; (+http://code.google.com/appengine)"`.
    ///   This header can be modified, but Cloud Tasks will append
    ///   `"AppEngine-Google; (+http://code.google.com/appengine)"` to the
    ///   modified `User-Agent`.
    ///
    /// If the task has a [body][google.cloud.tasks.v2.AppEngineHttpRequest.body], Cloud
    /// Tasks sets the following headers:
    ///
    /// * `Content-Type`: By default, the `Content-Type` header is set to
    ///   `"application/octet-stream"`. The default can be overridden by explicitly
    ///   setting `Content-Type` to a particular media type when the
    ///   [task is created][google.cloud.tasks.v2.CloudTasks.CreateTask].
    ///   For example, `Content-Type` can be set to `"application/json"`.
    /// * `Content-Length`: This is computed by Cloud Tasks. This value is
    ///   output only.   It cannot be changed.
    ///
    /// The headers below cannot be set or overridden:
    ///
    /// * `Host`
    /// * `X-Google-*`
    /// * `X-AppEngine-*`
    ///
    /// In addition, Cloud Tasks sets some headers when the task is dispatched,
    /// such as headers containing information about the task; see
    /// [request
    /// headers](https://cloud.google.com/tasks/docs/creating-appengine-handlers#reading_request_headers).
    /// These headers are set only when the task is dispatched, so they are not
    /// visible when the task is returned in a Cloud Tasks response.
    ///
    /// Although there is no specific limit for the maximum number of headers or
    /// the size, there is a limit on the maximum size of the [Task][google.cloud.tasks.v2.Task]. For more
    /// information, see the [CreateTask][google.cloud.tasks.v2.CloudTasks.CreateTask] documentation.
    #[prost(map = "string, string", tag = "4")]
    pub headers:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// HTTP request body.
    ///
    /// A request body is allowed only if the HTTP method is POST or PUT. It is
    /// an error to set a body on a task with an incompatible [HttpMethod][google.cloud.tasks.v2.HttpMethod].
    #[prost(bytes = "vec", tag = "5")]
    pub body: ::prost::alloc::vec::Vec<u8>,
}
/// App Engine Routing.
///
/// Defines routing characteristics specific to App Engine - service, version,
/// and instance.
///
/// For more information about services, versions, and instances see
/// [An Overview of App
/// Engine](https://cloud.google.com/appengine/docs/python/an-overview-of-app-engine),
/// [Microservices Architecture on Google App
/// Engine](https://cloud.google.com/appengine/docs/python/microservices-on-app-engine),
/// [App Engine Standard request
/// routing](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed),
/// and [App Engine Flex request
/// routing](https://cloud.google.com/appengine/docs/flexible/python/how-requests-are-routed).
///
/// Using [AppEngineRouting][google.cloud.tasks.v2.AppEngineRouting] requires
/// [`appengine.applications.get`](https://cloud.google.com/appengine/docs/admin-api/access-control)
/// Google IAM permission for the project
/// and the following scope:
///
/// `https://www.googleapis.com/auth/cloud-platform`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppEngineRouting {
    /// App service.
    ///
    /// By default, the task is sent to the service which is the default
    /// service when the task is attempted.
    ///
    /// For some queues or tasks which were created using the App Engine
    /// Task Queue API, [host][google.cloud.tasks.v2.AppEngineRouting.host] is not parsable
    /// into [service][google.cloud.tasks.v2.AppEngineRouting.service],
    /// [version][google.cloud.tasks.v2.AppEngineRouting.version], and
    /// [instance][google.cloud.tasks.v2.AppEngineRouting.instance]. For example, some tasks
    /// which were created using the App Engine SDK use a custom domain
    /// name; custom domains are not parsed by Cloud Tasks. If
    /// [host][google.cloud.tasks.v2.AppEngineRouting.host] is not parsable, then
    /// [service][google.cloud.tasks.v2.AppEngineRouting.service],
    /// [version][google.cloud.tasks.v2.AppEngineRouting.version], and
    /// [instance][google.cloud.tasks.v2.AppEngineRouting.instance] are the empty string.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// App version.
    ///
    /// By default, the task is sent to the version which is the default
    /// version when the task is attempted.
    ///
    /// For some queues or tasks which were created using the App Engine
    /// Task Queue API, [host][google.cloud.tasks.v2.AppEngineRouting.host] is not parsable
    /// into [service][google.cloud.tasks.v2.AppEngineRouting.service],
    /// [version][google.cloud.tasks.v2.AppEngineRouting.version], and
    /// [instance][google.cloud.tasks.v2.AppEngineRouting.instance]. For example, some tasks
    /// which were created using the App Engine SDK use a custom domain
    /// name; custom domains are not parsed by Cloud Tasks. If
    /// [host][google.cloud.tasks.v2.AppEngineRouting.host] is not parsable, then
    /// [service][google.cloud.tasks.v2.AppEngineRouting.service],
    /// [version][google.cloud.tasks.v2.AppEngineRouting.version], and
    /// [instance][google.cloud.tasks.v2.AppEngineRouting.instance] are the empty string.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// App instance.
    ///
    /// By default, the task is sent to an instance which is available when
    /// the task is attempted.
    ///
    /// Requests can only be sent to a specific instance if
    /// [manual scaling is used in App Engine
    /// Standard](https://cloud.google.com/appengine/docs/python/an-overview-of-app-engine?hl=en_US#scaling_types_and_instance_classes).
    /// App Engine Flex does not support instances. For more information, see
    /// [App Engine Standard request
    /// routing](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed)
    /// and [App Engine Flex request
    /// routing](https://cloud.google.com/appengine/docs/flexible/python/how-requests-are-routed).
    #[prost(string, tag = "3")]
    pub instance: ::prost::alloc::string::String,
    /// Output only. The host that the task is sent to.
    ///
    /// The host is constructed from the domain name of the app associated with
    /// the queue's project ID (for example <app-id>.appspot.com), and the
    /// [service][google.cloud.tasks.v2.AppEngineRouting.service], [version][google.cloud.tasks.v2.AppEngineRouting.version],
    /// and [instance][google.cloud.tasks.v2.AppEngineRouting.instance]. Tasks which were created using
    /// the App Engine SDK might have a custom domain name.
    ///
    /// For more information, see
    /// [How Requests are
    /// Routed](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed).
    #[prost(string, tag = "4")]
    pub host: ::prost::alloc::string::String,
}
/// Contains information needed for generating an
/// [OAuth token](https://developers.google.com/identity/protocols/OAuth2).
/// This type of authorization should generally only be used when calling Google
/// APIs hosted on *.googleapis.com.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuthToken {
    /// [Service account email](https://cloud.google.com/iam/docs/service-accounts)
    /// to be used for generating OAuth token.
    /// The service account must be within the same project as the queue. The
    /// caller must have iam.serviceAccounts.actAs permission for the service
    /// account.
    #[prost(string, tag = "1")]
    pub service_account_email: ::prost::alloc::string::String,
    /// OAuth scope to be used for generating OAuth access token.
    /// If not specified, "https://www.googleapis.com/auth/cloud-platform"
    /// will be used.
    #[prost(string, tag = "2")]
    pub scope: ::prost::alloc::string::String,
}
/// Contains information needed for generating an
/// [OpenID Connect
/// token](https://developers.google.com/identity/protocols/OpenIDConnect).
/// This type of authorization can be used for many scenarios, including
/// calling Cloud Run, or endpoints where you intend to validate the token
/// yourself.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OidcToken {
    /// [Service account email](https://cloud.google.com/iam/docs/service-accounts)
    /// to be used for generating OIDC token.
    /// The service account must be within the same project as the queue. The
    /// caller must have iam.serviceAccounts.actAs permission for the service
    /// account.
    #[prost(string, tag = "1")]
    pub service_account_email: ::prost::alloc::string::String,
    /// Audience to be used when generating OIDC token. If not specified, the URI
    /// specified in target will be used.
    #[prost(string, tag = "2")]
    pub audience: ::prost::alloc::string::String,
}
/// The HTTP method used to deliver the task.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HttpMethod {
    /// HTTP method unspecified
    Unspecified = 0,
    /// HTTP POST
    Post = 1,
    /// HTTP GET
    Get = 2,
    /// HTTP HEAD
    Head = 3,
    /// HTTP PUT
    Put = 4,
    /// HTTP DELETE
    Delete = 5,
    /// HTTP PATCH
    Patch = 6,
    /// HTTP OPTIONS
    Options = 7,
}
/// A queue is a container of related tasks. Queues are configured to manage
/// how those tasks are dispatched. Configurable properties include rate limits,
/// retry options, queue types, and others.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Queue {
    /// Caller-specified and required in [CreateQueue][google.cloud.tasks.v2.CloudTasks.CreateQueue],
    /// after which it becomes output only.
    ///
    /// The queue name.
    ///
    /// The queue name must have the following format:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
    ///
    /// * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]),
    ///    hyphens (-), colons (:), or periods (.).
    ///    For more information, see
    ///    [Identifying
    ///    projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects)
    /// * `LOCATION_ID` is the canonical ID for the queue's location.
    ///    The list of available locations can be obtained by calling
    ///    [ListLocations][google.cloud.location.Locations.ListLocations].
    ///    For more information, see https://cloud.google.com/about/locations/.
    /// * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or
    ///   hyphens (-). The maximum length is 100 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Overrides for
    /// [task-level app_engine_routing][google.cloud.tasks.v2.AppEngineHttpRequest.app_engine_routing].
    /// These settings apply only to
    /// [App Engine tasks][google.cloud.tasks.v2.AppEngineHttpRequest] in this queue.
    /// [Http tasks][google.cloud.tasks.v2.HttpRequest] are not affected.
    ///
    /// If set, `app_engine_routing_override` is used for all
    /// [App Engine tasks][google.cloud.tasks.v2.AppEngineHttpRequest] in the queue, no matter what the
    /// setting is for the
    /// [task-level app_engine_routing][google.cloud.tasks.v2.AppEngineHttpRequest.app_engine_routing].
    #[prost(message, optional, tag = "2")]
    pub app_engine_routing_override: ::core::option::Option<AppEngineRouting>,
    /// Rate limits for task dispatches.
    ///
    /// [rate_limits][google.cloud.tasks.v2.Queue.rate_limits] and [retry_config][google.cloud.tasks.v2.Queue.retry_config] are
    /// related because they both control task attempts. However they control task
    /// attempts in different ways:
    ///
    /// * [rate_limits][google.cloud.tasks.v2.Queue.rate_limits] controls the total rate of
    ///   dispatches from a queue (i.e. all traffic dispatched from the
    ///   queue, regardless of whether the dispatch is from a first
    ///   attempt or a retry).
    /// * [retry_config][google.cloud.tasks.v2.Queue.retry_config] controls what happens to
    ///   particular a task after its first attempt fails. That is,
    ///   [retry_config][google.cloud.tasks.v2.Queue.retry_config] controls task retries (the
    ///   second attempt, third attempt, etc).
    ///
    /// The queue's actual dispatch rate is the result of:
    ///
    /// * Number of tasks in the queue
    /// * User-specified throttling: [rate_limits][google.cloud.tasks.v2.Queue.rate_limits],
    ///   [retry_config][google.cloud.tasks.v2.Queue.retry_config], and the
    ///   [queue's state][google.cloud.tasks.v2.Queue.state].
    /// * System throttling due to `429` (Too Many Requests) or `503` (Service
    ///   Unavailable) responses from the worker, high error rates, or to smooth
    ///   sudden large traffic spikes.
    #[prost(message, optional, tag = "3")]
    pub rate_limits: ::core::option::Option<RateLimits>,
    /// Settings that determine the retry behavior.
    ///
    /// * For tasks created using Cloud Tasks: the queue-level retry settings
    ///   apply to all tasks in the queue that were created using Cloud Tasks.
    ///   Retry settings cannot be set on individual tasks.
    /// * For tasks created using the App Engine SDK: the queue-level retry
    ///   settings apply to all tasks in the queue which do not have retry settings
    ///   explicitly set on the task and were created by the App Engine SDK. See
    ///   [App Engine
    ///   documentation](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/retrying-tasks).
    #[prost(message, optional, tag = "4")]
    pub retry_config: ::core::option::Option<RetryConfig>,
    /// Output only. The state of the queue.
    ///
    /// `state` can only be changed by called
    /// [PauseQueue][google.cloud.tasks.v2.CloudTasks.PauseQueue],
    /// [ResumeQueue][google.cloud.tasks.v2.CloudTasks.ResumeQueue], or uploading
    /// [queue.yaml/xml](https://cloud.google.com/appengine/docs/python/config/queueref).
    /// [UpdateQueue][google.cloud.tasks.v2.CloudTasks.UpdateQueue] cannot be used to change `state`.
    #[prost(enumeration = "queue::State", tag = "5")]
    pub state: i32,
    /// Output only. The last time this queue was purged.
    ///
    /// All tasks that were [created][google.cloud.tasks.v2.Task.create_time] before this time
    /// were purged.
    ///
    /// A queue can be purged using [PurgeQueue][google.cloud.tasks.v2.CloudTasks.PurgeQueue], the
    /// [App Engine Task Queue SDK, or the Cloud
    /// Console](https://cloud.google.com/appengine/docs/standard/python/taskqueue/push/deleting-tasks-and-queues#purging_all_tasks_from_a_queue).
    ///
    /// Purge time will be truncated to the nearest microsecond. Purge
    /// time will be unset if the queue has never been purged.
    #[prost(message, optional, tag = "6")]
    pub purge_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Configuration options for writing logs to
    /// [Stackdriver Logging](https://cloud.google.com/logging/docs/). If this
    /// field is unset, then no logs are written.
    #[prost(message, optional, tag = "9")]
    pub stackdriver_logging_config: ::core::option::Option<StackdriverLoggingConfig>,
}
/// Nested message and enum types in `Queue`.
pub mod queue {
    /// State of the queue.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// The queue is running. Tasks can be dispatched.
        ///
        /// If the queue was created using Cloud Tasks and the queue has
        /// had no activity (method calls or task dispatches) for 30 days,
        /// the queue may take a few minutes to re-activate. Some method
        /// calls may return [NOT_FOUND][google.rpc.Code.NOT_FOUND] and
        /// tasks may not be dispatched for a few minutes until the queue
        /// has been re-activated.
        Running = 1,
        /// Tasks are paused by the user. If the queue is paused then Cloud
        /// Tasks will stop delivering tasks from it, but more tasks can
        /// still be added to it by the user.
        Paused = 2,
        /// The queue is disabled.
        ///
        /// A queue becomes `DISABLED` when
        /// [queue.yaml](https://cloud.google.com/appengine/docs/python/config/queueref)
        /// or
        /// [queue.xml](https://cloud.google.com/appengine/docs/standard/java/config/queueref)
        /// is uploaded which does not contain the queue. You cannot directly disable
        /// a queue.
        ///
        /// When a queue is disabled, tasks can still be added to a queue
        /// but the tasks are not dispatched.
        ///
        /// To permanently delete this queue and all of its tasks, call
        /// [DeleteQueue][google.cloud.tasks.v2.CloudTasks.DeleteQueue].
        Disabled = 3,
    }
}
/// Rate limits.
///
/// This message determines the maximum rate that tasks can be dispatched by a
/// queue, regardless of whether the dispatch is a first task attempt or a retry.
///
/// Note: The debugging command, [RunTask][google.cloud.tasks.v2.CloudTasks.RunTask], will run a task
/// even if the queue has reached its [RateLimits][google.cloud.tasks.v2.RateLimits].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimits {
    /// The maximum rate at which tasks are dispatched from this queue.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    /// * The maximum allowed value is 500.
    ///
    ///
    /// This field has the same meaning as
    /// [rate in
    /// queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#rate).
    #[prost(double, tag = "1")]
    pub max_dispatches_per_second: f64,
    /// Output only. The max burst size.
    ///
    /// Max burst size limits how fast tasks in queue are processed when
    /// many tasks are in the queue and the rate is high. This field
    /// allows the queue to have a high rate so processing starts shortly
    /// after a task is enqueued, but still limits resource usage when
    /// many tasks are enqueued in a short period of time.
    ///
    /// The [token bucket](https://wikipedia.org/wiki/Token_Bucket)
    /// algorithm is used to control the rate of task dispatches. Each
    /// queue has a token bucket that holds tokens, up to the maximum
    /// specified by `max_burst_size`. Each time a task is dispatched, a
    /// token is removed from the bucket. Tasks will be dispatched until
    /// the queue's bucket runs out of tokens. The bucket will be
    /// continuously refilled with new tokens based on
    /// [max_dispatches_per_second][google.cloud.tasks.v2.RateLimits.max_dispatches_per_second].
    ///
    /// Cloud Tasks will pick the value of `max_burst_size` based on the
    /// value of
    /// [max_dispatches_per_second][google.cloud.tasks.v2.RateLimits.max_dispatches_per_second].
    ///
    /// For queues that were created or updated using
    /// `queue.yaml/xml`, `max_burst_size` is equal to
    /// [bucket_size](https://cloud.google.com/appengine/docs/standard/python/config/queueref#bucket_size).
    /// Since `max_burst_size` is output only, if
    /// [UpdateQueue][google.cloud.tasks.v2.CloudTasks.UpdateQueue] is called on a queue
    /// created by `queue.yaml/xml`, `max_burst_size` will be reset based
    /// on the value of
    /// [max_dispatches_per_second][google.cloud.tasks.v2.RateLimits.max_dispatches_per_second],
    /// regardless of whether
    /// [max_dispatches_per_second][google.cloud.tasks.v2.RateLimits.max_dispatches_per_second]
    /// is updated.
    ///
    #[prost(int32, tag = "2")]
    pub max_burst_size: i32,
    /// The maximum number of concurrent tasks that Cloud Tasks allows
    /// to be dispatched for this queue. After this threshold has been
    /// reached, Cloud Tasks stops dispatching tasks until the number of
    /// concurrent requests decreases.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    ///
    /// The maximum allowed value is 5,000.
    ///
    ///
    /// This field has the same meaning as
    /// [max_concurrent_requests in
    /// queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#max_concurrent_requests).
    #[prost(int32, tag = "3")]
    pub max_concurrent_dispatches: i32,
}
/// Retry config.
///
/// These settings determine when a failed task attempt is retried.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryConfig {
    /// Number of attempts per task.
    ///
    /// Cloud Tasks will attempt the task `max_attempts` times (that is, if the
    /// first attempt fails, then there will be `max_attempts - 1` retries). Must
    /// be >= -1.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    /// -1 indicates unlimited attempts.
    ///
    /// This field has the same meaning as
    /// [task_retry_limit in
    /// queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters).
    #[prost(int32, tag = "1")]
    pub max_attempts: i32,
    /// If positive, `max_retry_duration` specifies the time limit for
    /// retrying a failed task, measured from when the task was first
    /// attempted. Once `max_retry_duration` time has passed *and* the
    /// task has been attempted [max_attempts][google.cloud.tasks.v2.RetryConfig.max_attempts]
    /// times, no further attempts will be made and the task will be
    /// deleted.
    ///
    /// If zero, then the task age is unlimited.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    ///
    /// `max_retry_duration` will be truncated to the nearest second.
    ///
    /// This field has the same meaning as
    /// [task_age_limit in
    /// queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters).
    #[prost(message, optional, tag = "2")]
    pub max_retry_duration: ::core::option::Option<::prost_types::Duration>,
    /// A task will be [scheduled][google.cloud.tasks.v2.Task.schedule_time] for retry between
    /// [min_backoff][google.cloud.tasks.v2.RetryConfig.min_backoff] and
    /// [max_backoff][google.cloud.tasks.v2.RetryConfig.max_backoff] duration after it fails,
    /// if the queue's [RetryConfig][google.cloud.tasks.v2.RetryConfig] specifies that the task should be
    /// retried.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    ///
    /// `min_backoff` will be truncated to the nearest second.
    ///
    /// This field has the same meaning as
    /// [min_backoff_seconds in
    /// queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters).
    #[prost(message, optional, tag = "3")]
    pub min_backoff: ::core::option::Option<::prost_types::Duration>,
    /// A task will be [scheduled][google.cloud.tasks.v2.Task.schedule_time] for retry between
    /// [min_backoff][google.cloud.tasks.v2.RetryConfig.min_backoff] and
    /// [max_backoff][google.cloud.tasks.v2.RetryConfig.max_backoff] duration after it fails,
    /// if the queue's [RetryConfig][google.cloud.tasks.v2.RetryConfig] specifies that the task should be
    /// retried.
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    ///
    /// `max_backoff` will be truncated to the nearest second.
    ///
    /// This field has the same meaning as
    /// [max_backoff_seconds in
    /// queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters).
    #[prost(message, optional, tag = "4")]
    pub max_backoff: ::core::option::Option<::prost_types::Duration>,
    /// The time between retries will double `max_doublings` times.
    ///
    /// A task's retry interval starts at
    /// [min_backoff][google.cloud.tasks.v2.RetryConfig.min_backoff], then doubles
    /// `max_doublings` times, then increases linearly, and finally
    /// retries retries at intervals of
    /// [max_backoff][google.cloud.tasks.v2.RetryConfig.max_backoff] up to
    /// [max_attempts][google.cloud.tasks.v2.RetryConfig.max_attempts] times.
    ///
    /// For example, if [min_backoff][google.cloud.tasks.v2.RetryConfig.min_backoff] is 10s,
    /// [max_backoff][google.cloud.tasks.v2.RetryConfig.max_backoff] is 300s, and
    /// `max_doublings` is 3, then the a task will first be retried in
    /// 10s. The retry interval will double three times, and then
    /// increase linearly by 2^3 * 10s.  Finally, the task will retry at
    /// intervals of [max_backoff][google.cloud.tasks.v2.RetryConfig.max_backoff] until the
    /// task has been attempted [max_attempts][google.cloud.tasks.v2.RetryConfig.max_attempts]
    /// times. Thus, the requests will retry at 10s, 20s, 40s, 80s, 160s,
    /// 240s, 300s, 300s, ....
    ///
    /// If unspecified when the queue is created, Cloud Tasks will pick the
    /// default.
    ///
    ///
    /// This field has the same meaning as
    /// [max_doublings in
    /// queue.yaml/xml](https://cloud.google.com/appengine/docs/standard/python/config/queueref#retry_parameters).
    #[prost(int32, tag = "5")]
    pub max_doublings: i32,
}
/// Configuration options for writing logs to
/// [Stackdriver Logging](https://cloud.google.com/logging/docs/).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StackdriverLoggingConfig {
    /// Specifies the fraction of operations to write to
    /// [Stackdriver Logging](https://cloud.google.com/logging/docs/).
    /// This field may contain any value between 0.0 and 1.0, inclusive.
    /// 0.0 is the default and means that no operations are logged.
    #[prost(double, tag = "1")]
    pub sampling_ratio: f64,
}
/// A unit of scheduled work.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Optionally caller-specified in [CreateTask][google.cloud.tasks.v2.CloudTasks.CreateTask].
    ///
    /// The task name.
    ///
    /// The task name must have the following format:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
    ///
    /// * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]),
    ///    hyphens (-), colons (:), or periods (.).
    ///    For more information, see
    ///    [Identifying
    ///    projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects)
    /// * `LOCATION_ID` is the canonical ID for the task's location.
    ///    The list of available locations can be obtained by calling
    ///    [ListLocations][google.cloud.location.Locations.ListLocations].
    ///    For more information, see https://cloud.google.com/about/locations/.
    /// * `QUEUE_ID` can contain letters ([A-Za-z]), numbers ([0-9]), or
    ///   hyphens (-). The maximum length is 100 characters.
    /// * `TASK_ID` can contain only letters ([A-Za-z]), numbers ([0-9]),
    ///   hyphens (-), or underscores (_). The maximum length is 500 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The time when the task is scheduled to be attempted or retried.
    ///
    /// `schedule_time` will be truncated to the nearest microsecond.
    #[prost(message, optional, tag = "4")]
    pub schedule_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time that the task was created.
    ///
    /// `create_time` will be truncated to the nearest second.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The deadline for requests sent to the worker. If the worker does not
    /// respond by this deadline then the request is cancelled and the attempt
    /// is marked as a `DEADLINE_EXCEEDED` failure. Cloud Tasks will retry the
    /// task according to the [RetryConfig][google.cloud.tasks.v2.RetryConfig].
    ///
    /// Note that when the request is cancelled, Cloud Tasks will stop listing for
    /// the response, but whether the worker stops processing depends on the
    /// worker. For example, if the worker is stuck, it may not react to cancelled
    /// requests.
    ///
    /// The default and maximum values depend on the type of request:
    ///
    /// * For [HTTP tasks][google.cloud.tasks.v2.HttpRequest], the default is 10 minutes. The deadline
    ///   must be in the interval [15 seconds, 30 minutes].
    ///
    /// * For [App Engine tasks][google.cloud.tasks.v2.AppEngineHttpRequest], 0 indicates that the
    ///   request has the default deadline. The default deadline depends on the
    ///   [scaling
    ///   type](https://cloud.google.com/appengine/docs/standard/go/how-instances-are-managed#instance_scaling)
    ///   of the service: 10 minutes for standard apps with automatic scaling, 24
    ///   hours for standard apps with manual and basic scaling, and 60 minutes for
    ///   flex apps. If the request deadline is set, it must be in the interval [15
    ///   seconds, 24 hours 15 seconds]. Regardless of the task's
    ///   `dispatch_deadline`, the app handler will not run for longer than than
    ///   the service's timeout. We recommend setting the `dispatch_deadline` to
    ///   at most a few seconds more than the app handler's timeout. For more
    ///   information see
    ///   [Timeouts](https://cloud.google.com/tasks/docs/creating-appengine-handlers#timeouts).
    ///
    /// `dispatch_deadline` will be truncated to the nearest millisecond. The
    /// deadline is an approximate deadline.
    #[prost(message, optional, tag = "6")]
    pub dispatch_deadline: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The number of attempts dispatched.
    ///
    /// This count includes attempts which have been dispatched but haven't
    /// received a response.
    #[prost(int32, tag = "7")]
    pub dispatch_count: i32,
    /// Output only. The number of attempts which have received a response.
    #[prost(int32, tag = "8")]
    pub response_count: i32,
    /// Output only. The status of the task's first attempt.
    ///
    /// Only [dispatch_time][google.cloud.tasks.v2.Attempt.dispatch_time] will be set.
    /// The other [Attempt][google.cloud.tasks.v2.Attempt] information is not retained by Cloud Tasks.
    #[prost(message, optional, tag = "9")]
    pub first_attempt: ::core::option::Option<Attempt>,
    /// Output only. The status of the task's last attempt.
    #[prost(message, optional, tag = "10")]
    pub last_attempt: ::core::option::Option<Attempt>,
    /// Output only. The view specifies which subset of the [Task][google.cloud.tasks.v2.Task] has
    /// been returned.
    #[prost(enumeration = "task::View", tag = "11")]
    pub view: i32,
    /// Required. The message to send to the worker.
    #[prost(oneof = "task::MessageType", tags = "2, 3")]
    pub message_type: ::core::option::Option<task::MessageType>,
}
/// Nested message and enum types in `Task`.
pub mod task {
    /// The view specifies a subset of [Task][google.cloud.tasks.v2.Task] data.
    ///
    /// When a task is returned in a response, not all
    /// information is retrieved by default because some data, such as
    /// payloads, might be desirable to return only when needed because
    /// of its large size or because of the sensitivity of data that it
    /// contains.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum View {
        /// Unspecified. Defaults to BASIC.
        Unspecified = 0,
        /// The basic view omits fields which can be large or can contain
        /// sensitive data.
        ///
        /// This view does not include the
        /// [body in AppEngineHttpRequest][google.cloud.tasks.v2.AppEngineHttpRequest.body].
        /// Bodies are desirable to return only when needed, because they
        /// can be large and because of the sensitivity of the data that you
        /// choose to store in it.
        Basic = 1,
        /// All information is returned.
        ///
        /// Authorization for [FULL][google.cloud.tasks.v2.Task.View.FULL] requires
        /// `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/)
        /// permission on the [Queue][google.cloud.tasks.v2.Queue] resource.
        Full = 2,
    }
    /// Required. The message to send to the worker.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MessageType {
        /// HTTP request that is sent to the App Engine app handler.
        ///
        /// An App Engine task is a task that has [AppEngineHttpRequest][google.cloud.tasks.v2.AppEngineHttpRequest] set.
        #[prost(message, tag = "2")]
        AppEngineHttpRequest(super::AppEngineHttpRequest),
        /// HTTP request that is sent to the worker.
        ///
        /// An HTTP task is a task that has [HttpRequest][google.cloud.tasks.v2.HttpRequest] set.
        #[prost(message, tag = "3")]
        HttpRequest(super::HttpRequest),
    }
}
/// The status of a task attempt.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attempt {
    /// Output only. The time that this attempt was scheduled.
    ///
    /// `schedule_time` will be truncated to the nearest microsecond.
    #[prost(message, optional, tag = "1")]
    pub schedule_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time that this attempt was dispatched.
    ///
    /// `dispatch_time` will be truncated to the nearest microsecond.
    #[prost(message, optional, tag = "2")]
    pub dispatch_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time that this attempt response was received.
    ///
    /// `response_time` will be truncated to the nearest microsecond.
    #[prost(message, optional, tag = "3")]
    pub response_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The response from the worker for this attempt.
    ///
    /// If `response_time` is unset, then the task has not been attempted or is
    /// currently running and the `response_status` field is meaningless.
    #[prost(message, optional, tag = "4")]
    pub response_status: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Request message for [ListQueues][google.cloud.tasks.v2.CloudTasks.ListQueues].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQueuesRequest {
    /// Required. The location name.
    /// For example: `projects/PROJECT_ID/locations/LOCATION_ID`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// `filter` can be used to specify a subset of queues. Any [Queue][google.cloud.tasks.v2.Queue]
    /// field can be used as a filter and several operators as supported.
    /// For example: `<=, <, >=, >, !=, =, :`. The filter syntax is the same as
    /// described in
    /// [Stackdriver's Advanced Logs
    /// Filters](https://cloud.google.com/logging/docs/view/advanced_filters).
    ///
    /// Sample filter "state: PAUSED".
    ///
    /// Note that using filters might cause fewer queues than the
    /// requested page_size to be returned.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Requested page size.
    ///
    /// The maximum page size is 9800. If unspecified, the page size will
    /// be the maximum. Fewer queues than requested might be returned,
    /// even if more queues exist; use the
    /// [next_page_token][google.cloud.tasks.v2.ListQueuesResponse.next_page_token] in the
    /// response to determine if more queues exist.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// A token identifying the page of results to return.
    ///
    /// To request the first page results, page_token must be empty. To
    /// request the next page of results, page_token must be the value of
    /// [next_page_token][google.cloud.tasks.v2.ListQueuesResponse.next_page_token] returned
    /// from the previous call to [ListQueues][google.cloud.tasks.v2.CloudTasks.ListQueues]
    /// method. It is an error to switch the value of the
    /// [filter][google.cloud.tasks.v2.ListQueuesRequest.filter] while iterating through pages.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for [ListQueues][google.cloud.tasks.v2.CloudTasks.ListQueues].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQueuesResponse {
    /// The list of queues.
    #[prost(message, repeated, tag = "1")]
    pub queues: ::prost::alloc::vec::Vec<Queue>,
    /// A token to retrieve next page of results.
    ///
    /// To return the next page of results, call
    /// [ListQueues][google.cloud.tasks.v2.CloudTasks.ListQueues] with this value as the
    /// [page_token][google.cloud.tasks.v2.ListQueuesRequest.page_token].
    ///
    /// If the next_page_token is empty, there are no more results.
    ///
    /// The page token is valid for only 2 hours.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [GetQueue][google.cloud.tasks.v2.CloudTasks.GetQueue].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQueueRequest {
    /// Required. The resource name of the queue. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [CreateQueue][google.cloud.tasks.v2.CloudTasks.CreateQueue].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQueueRequest {
    /// Required. The location name in which the queue will be created.
    /// For example: `projects/PROJECT_ID/locations/LOCATION_ID`
    ///
    /// The list of allowed locations can be obtained by calling Cloud
    /// Tasks' implementation of
    /// [ListLocations][google.cloud.location.Locations.ListLocations].
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The queue to create.
    ///
    /// [Queue's name][google.cloud.tasks.v2.Queue.name] cannot be the same as an existing queue.
    #[prost(message, optional, tag = "2")]
    pub queue: ::core::option::Option<Queue>,
}
/// Request message for [UpdateQueue][google.cloud.tasks.v2.CloudTasks.UpdateQueue].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateQueueRequest {
    /// Required. The queue to create or update.
    ///
    /// The queue's [name][google.cloud.tasks.v2.Queue.name] must be specified.
    ///
    /// Output only fields cannot be modified using UpdateQueue.
    /// Any value specified for an output only field will be ignored.
    /// The queue's [name][google.cloud.tasks.v2.Queue.name] cannot be changed.
    #[prost(message, optional, tag = "1")]
    pub queue: ::core::option::Option<Queue>,
    /// A mask used to specify which fields of the queue are being updated.
    ///
    /// If empty, then all fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for [DeleteQueue][google.cloud.tasks.v2.CloudTasks.DeleteQueue].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteQueueRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [PurgeQueue][google.cloud.tasks.v2.CloudTasks.PurgeQueue].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeQueueRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/location/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [PauseQueue][google.cloud.tasks.v2.CloudTasks.PauseQueue].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseQueueRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/location/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [ResumeQueue][google.cloud.tasks.v2.CloudTasks.ResumeQueue].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeQueueRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/location/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for listing tasks using [ListTasks][google.cloud.tasks.v2.CloudTasks.ListTasks].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The response_view specifies which subset of the [Task][google.cloud.tasks.v2.Task] will be
    /// returned.
    ///
    /// By default response_view is [BASIC][google.cloud.tasks.v2.Task.View.BASIC]; not all
    /// information is retrieved by default because some data, such as
    /// payloads, might be desirable to return only when needed because
    /// of its large size or because of the sensitivity of data that it
    /// contains.
    ///
    /// Authorization for [FULL][google.cloud.tasks.v2.Task.View.FULL] requires
    /// `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/)
    /// permission on the [Task][google.cloud.tasks.v2.Task] resource.
    #[prost(enumeration = "task::View", tag = "2")]
    pub response_view: i32,
    /// Maximum page size.
    ///
    /// Fewer tasks than requested might be returned, even if more tasks exist; use
    /// [next_page_token][google.cloud.tasks.v2.ListTasksResponse.next_page_token] in the response to
    /// determine if more tasks exist.
    ///
    /// The maximum page size is 1000. If unspecified, the page size will be the
    /// maximum.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// A token identifying the page of results to return.
    ///
    /// To request the first page results, page_token must be empty. To
    /// request the next page of results, page_token must be the value of
    /// [next_page_token][google.cloud.tasks.v2.ListTasksResponse.next_page_token] returned
    /// from the previous call to [ListTasks][google.cloud.tasks.v2.CloudTasks.ListTasks]
    /// method.
    ///
    /// The page token is valid for only 2 hours.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing tasks using [ListTasks][google.cloud.tasks.v2.CloudTasks.ListTasks].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksResponse {
    /// The list of tasks.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
    /// A token to retrieve next page of results.
    ///
    /// To return the next page of results, call
    /// [ListTasks][google.cloud.tasks.v2.CloudTasks.ListTasks] with this value as the
    /// [page_token][google.cloud.tasks.v2.ListTasksRequest.page_token].
    ///
    /// If the next_page_token is empty, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for getting a task using [GetTask][google.cloud.tasks.v2.CloudTasks.GetTask].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskRequest {
    /// Required. The task name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The response_view specifies which subset of the [Task][google.cloud.tasks.v2.Task] will be
    /// returned.
    ///
    /// By default response_view is [BASIC][google.cloud.tasks.v2.Task.View.BASIC]; not all
    /// information is retrieved by default because some data, such as
    /// payloads, might be desirable to return only when needed because
    /// of its large size or because of the sensitivity of data that it
    /// contains.
    ///
    /// Authorization for [FULL][google.cloud.tasks.v2.Task.View.FULL] requires
    /// `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/)
    /// permission on the [Task][google.cloud.tasks.v2.Task] resource.
    #[prost(enumeration = "task::View", tag = "2")]
    pub response_view: i32,
}
/// Request message for [CreateTask][google.cloud.tasks.v2.CloudTasks.CreateTask].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskRequest {
    /// Required. The queue name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID`
    ///
    /// The queue must already exist.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The task to add.
    ///
    /// Task names have the following format:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`.
    /// The user can optionally specify a task [name][google.cloud.tasks.v2.Task.name]. If a
    /// name is not specified then the system will generate a random
    /// unique task id, which will be set in the task returned in the
    /// [response][google.cloud.tasks.v2.Task.name].
    ///
    /// If [schedule_time][google.cloud.tasks.v2.Task.schedule_time] is not set or is in the
    /// past then Cloud Tasks will set it to the current time.
    ///
    /// Task De-duplication:
    ///
    /// Explicitly specifying a task ID enables task de-duplication.  If
    /// a task's ID is identical to that of an existing task or a task
    /// that was deleted or executed recently then the call will fail
    /// with [ALREADY_EXISTS][google.rpc.Code.ALREADY_EXISTS].
    /// If the task's queue was created using Cloud Tasks, then another task with
    /// the same name can't be created for ~1hour after the original task was
    /// deleted or executed. If the task's queue was created using queue.yaml or
    /// queue.xml, then another task with the same name can't be created
    /// for ~9days after the original task was deleted or executed.
    ///
    /// Because there is an extra lookup cost to identify duplicate task
    /// names, these [CreateTask][google.cloud.tasks.v2.CloudTasks.CreateTask] calls have significantly
    /// increased latency. Using hashed strings for the task id or for
    /// the prefix of the task id is recommended. Choosing task ids that
    /// are sequential or have sequential prefixes, for example using a
    /// timestamp, causes an increase in latency and error rates in all
    /// task commands. The infrastructure relies on an approximately
    /// uniform distribution of task ids to store and serve tasks
    /// efficiently.
    #[prost(message, optional, tag = "2")]
    pub task: ::core::option::Option<Task>,
    /// The response_view specifies which subset of the [Task][google.cloud.tasks.v2.Task] will be
    /// returned.
    ///
    /// By default response_view is [BASIC][google.cloud.tasks.v2.Task.View.BASIC]; not all
    /// information is retrieved by default because some data, such as
    /// payloads, might be desirable to return only when needed because
    /// of its large size or because of the sensitivity of data that it
    /// contains.
    ///
    /// Authorization for [FULL][google.cloud.tasks.v2.Task.View.FULL] requires
    /// `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/)
    /// permission on the [Task][google.cloud.tasks.v2.Task] resource.
    #[prost(enumeration = "task::View", tag = "3")]
    pub response_view: i32,
}
/// Request message for deleting a task using
/// [DeleteTask][google.cloud.tasks.v2.CloudTasks.DeleteTask].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTaskRequest {
    /// Required. The task name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for forcing a task to run now using
/// [RunTask][google.cloud.tasks.v2.CloudTasks.RunTask].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTaskRequest {
    /// Required. The task name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/queues/QUEUE_ID/tasks/TASK_ID`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The response_view specifies which subset of the [Task][google.cloud.tasks.v2.Task] will be
    /// returned.
    ///
    /// By default response_view is [BASIC][google.cloud.tasks.v2.Task.View.BASIC]; not all
    /// information is retrieved by default because some data, such as
    /// payloads, might be desirable to return only when needed because
    /// of its large size or because of the sensitivity of data that it
    /// contains.
    ///
    /// Authorization for [FULL][google.cloud.tasks.v2.Task.View.FULL] requires
    /// `cloudtasks.tasks.fullView` [Google IAM](https://cloud.google.com/iam/)
    /// permission on the [Task][google.cloud.tasks.v2.Task] resource.
    #[prost(enumeration = "task::View", tag = "2")]
    pub response_view: i32,
}
#[doc = r" Generated client implementations."]
pub mod cloud_tasks_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Cloud Tasks allows developers to manage the execution of background"]
    #[doc = " work in their applications."]
    #[derive(Debug, Clone)]
    pub struct CloudTasksClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudTasksClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> CloudTasksClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CloudTasksClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists queues."]
        #[doc = ""]
        #[doc = " Queues are returned in lexicographical order."]
        pub async fn list_queues(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQueuesRequest>,
        ) -> Result<tonic::Response<super::ListQueuesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tasks.v2.CloudTasks/ListQueues",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a queue."]
        pub async fn get_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQueueRequest>,
        ) -> Result<tonic::Response<super::Queue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.tasks.v2.CloudTasks/GetQueue");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a queue."]
        #[doc = ""]
        #[doc = " Queues created with this method allow tasks to live for a maximum of 31"]
        #[doc = " days. After a task is 31 days old, the task will be deleted regardless of whether"]
        #[doc = " it was dispatched or not."]
        #[doc = ""]
        #[doc = " WARNING: Using this method may have unintended side effects if you are"]
        #[doc = " using an App Engine `queue.yaml` or `queue.xml` file to manage your queues."]
        #[doc = " Read"]
        #[doc = " [Overview of Queue Management and"]
        #[doc = " queue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using"]
        #[doc = " this method."]
        pub async fn create_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQueueRequest>,
        ) -> Result<tonic::Response<super::Queue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tasks.v2.CloudTasks/CreateQueue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a queue."]
        #[doc = ""]
        #[doc = " This method creates the queue if it does not exist and updates"]
        #[doc = " the queue if it does exist."]
        #[doc = ""]
        #[doc = " Queues created with this method allow tasks to live for a maximum of 31"]
        #[doc = " days. After a task is 31 days old, the task will be deleted regardless of whether"]
        #[doc = " it was dispatched or not."]
        #[doc = ""]
        #[doc = " WARNING: Using this method may have unintended side effects if you are"]
        #[doc = " using an App Engine `queue.yaml` or `queue.xml` file to manage your queues."]
        #[doc = " Read"]
        #[doc = " [Overview of Queue Management and"]
        #[doc = " queue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using"]
        #[doc = " this method."]
        pub async fn update_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateQueueRequest>,
        ) -> Result<tonic::Response<super::Queue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tasks.v2.CloudTasks/UpdateQueue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a queue."]
        #[doc = ""]
        #[doc = " This command will delete the queue even if it has tasks in it."]
        #[doc = ""]
        #[doc = " Note: If you delete a queue, a queue with the same name can't be created"]
        #[doc = " for 7 days."]
        #[doc = ""]
        #[doc = " WARNING: Using this method may have unintended side effects if you are"]
        #[doc = " using an App Engine `queue.yaml` or `queue.xml` file to manage your queues."]
        #[doc = " Read"]
        #[doc = " [Overview of Queue Management and"]
        #[doc = " queue.yaml](https://cloud.google.com/tasks/docs/queue-yaml) before using"]
        #[doc = " this method."]
        pub async fn delete_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteQueueRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tasks.v2.CloudTasks/DeleteQueue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Purges a queue by deleting all of its tasks."]
        #[doc = ""]
        #[doc = " All tasks created before this method is called are permanently deleted."]
        #[doc = ""]
        #[doc = " Purge operations can take up to one minute to take effect. Tasks"]
        #[doc = " might be dispatched before the purge takes effect. A purge is irreversible."]
        pub async fn purge_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::PurgeQueueRequest>,
        ) -> Result<tonic::Response<super::Queue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tasks.v2.CloudTasks/PurgeQueue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Pauses the queue."]
        #[doc = ""]
        #[doc = " If a queue is paused then the system will stop dispatching tasks"]
        #[doc = " until the queue is resumed via"]
        #[doc = " [ResumeQueue][google.cloud.tasks.v2.CloudTasks.ResumeQueue]. Tasks can still be added"]
        #[doc = " when the queue is paused. A queue is paused if its"]
        #[doc = " [state][google.cloud.tasks.v2.Queue.state] is [PAUSED][google.cloud.tasks.v2.Queue.State.PAUSED]."]
        pub async fn pause_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseQueueRequest>,
        ) -> Result<tonic::Response<super::Queue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tasks.v2.CloudTasks/PauseQueue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Resume a queue."]
        #[doc = ""]
        #[doc = " This method resumes a queue after it has been"]
        #[doc = " [PAUSED][google.cloud.tasks.v2.Queue.State.PAUSED] or"]
        #[doc = " [DISABLED][google.cloud.tasks.v2.Queue.State.DISABLED]. The state of a queue is stored"]
        #[doc = " in the queue's [state][google.cloud.tasks.v2.Queue.state]; after calling this method it"]
        #[doc = " will be set to [RUNNING][google.cloud.tasks.v2.Queue.State.RUNNING]."]
        #[doc = ""]
        #[doc = " WARNING: Resuming many high-QPS queues at the same time can"]
        #[doc = " lead to target overloading. If you are resuming high-QPS"]
        #[doc = " queues, follow the 500/50/5 pattern described in"]
        #[doc = " [Managing Cloud Tasks Scaling"]
        #[doc = " Risks](https://cloud.google.com/tasks/docs/manage-cloud-task-scaling)."]
        pub async fn resume_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeQueueRequest>,
        ) -> Result<tonic::Response<super::Queue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tasks.v2.CloudTasks/ResumeQueue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a [Queue][google.cloud.tasks.v2.Queue]."]
        #[doc = " Returns an empty policy if the resource exists and does not have a policy"]
        #[doc = " set."]
        #[doc = ""]
        #[doc = " Authorization requires the following"]
        #[doc = " [Google IAM](https://cloud.google.com/iam) permission on the specified"]
        #[doc = " resource parent:"]
        #[doc = ""]
        #[doc = " * `cloudtasks.queues.getIamPolicy`"]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tasks.v2.CloudTasks/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy for a [Queue][google.cloud.tasks.v2.Queue]. Replaces any existing"]
        #[doc = " policy."]
        #[doc = ""]
        #[doc = " Note: The Cloud Console does not check queue-level IAM permissions yet."]
        #[doc = " Project-level permissions are required to use the Cloud Console."]
        #[doc = ""]
        #[doc = " Authorization requires the following"]
        #[doc = " [Google IAM](https://cloud.google.com/iam) permission on the specified"]
        #[doc = " resource parent:"]
        #[doc = ""]
        #[doc = " * `cloudtasks.queues.setIamPolicy`"]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tasks.v2.CloudTasks/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that a caller has on a [Queue][google.cloud.tasks.v2.Queue]."]
        #[doc = " If the resource does not exist, this will return an empty set of"]
        #[doc = " permissions, not a [NOT_FOUND][google.rpc.Code.NOT_FOUND] error."]
        #[doc = ""]
        #[doc = " Note: This operation is designed to be used for building permission-aware"]
        #[doc = " UIs and command-line tools, not for authorization checking. This operation"]
        #[doc = " may \"fail open\" without warning."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.cloud.tasks.v2.CloudTasks/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the tasks in a queue."]
        #[doc = ""]
        #[doc = " By default, only the [BASIC][google.cloud.tasks.v2.Task.View.BASIC] view is retrieved"]
        #[doc = " due to performance considerations;"]
        #[doc = " [response_view][google.cloud.tasks.v2.ListTasksRequest.response_view] controls the"]
        #[doc = " subset of information which is returned."]
        #[doc = ""]
        #[doc = " The tasks may be returned in any order. The ordering may change at any"]
        #[doc = " time."]
        pub async fn list_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTasksRequest>,
        ) -> Result<tonic::Response<super::ListTasksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.tasks.v2.CloudTasks/ListTasks");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a task."]
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaskRequest>,
        ) -> Result<tonic::Response<super::Task>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.tasks.v2.CloudTasks/GetTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a task and adds it to a queue."]
        #[doc = ""]
        #[doc = " Tasks cannot be updated after creation; there is no UpdateTask command."]
        #[doc = ""]
        #[doc = " * The maximum task size is 100KB."]
        pub async fn create_task(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTaskRequest>,
        ) -> Result<tonic::Response<super::Task>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tasks.v2.CloudTasks/CreateTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a task."]
        #[doc = ""]
        #[doc = " A task can be deleted if it is scheduled or dispatched. A task"]
        #[doc = " cannot be deleted if it has executed successfully or permanently"]
        #[doc = " failed."]
        pub async fn delete_task(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTaskRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tasks.v2.CloudTasks/DeleteTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Forces a task to run now."]
        #[doc = ""]
        #[doc = " When this method is called, Cloud Tasks will dispatch the task, even if"]
        #[doc = " the task is already running, the queue has reached its [RateLimits][google.cloud.tasks.v2.RateLimits] or"]
        #[doc = " is [PAUSED][google.cloud.tasks.v2.Queue.State.PAUSED]."]
        #[doc = ""]
        #[doc = " This command is meant to be used for manual debugging. For"]
        #[doc = " example, [RunTask][google.cloud.tasks.v2.CloudTasks.RunTask] can be used to retry a failed"]
        #[doc = " task after a fix has been made or to manually force a task to be"]
        #[doc = " dispatched now."]
        #[doc = ""]
        #[doc = " The dispatched task is returned. That is, the task that is returned"]
        #[doc = " contains the [status][Task.status] after the task is dispatched but"]
        #[doc = " before the task is received by its target."]
        #[doc = ""]
        #[doc = " If Cloud Tasks receives a successful response from the task's"]
        #[doc = " target, then the task will be deleted; otherwise the task's"]
        #[doc = " [schedule_time][google.cloud.tasks.v2.Task.schedule_time] will be reset to the time that"]
        #[doc = " [RunTask][google.cloud.tasks.v2.CloudTasks.RunTask] was called plus the retry delay specified"]
        #[doc = " in the queue's [RetryConfig][google.cloud.tasks.v2.RetryConfig]."]
        #[doc = ""]
        #[doc = " [RunTask][google.cloud.tasks.v2.CloudTasks.RunTask] returns"]
        #[doc = " [NOT_FOUND][google.rpc.Code.NOT_FOUND] when it is called on a"]
        #[doc = " task that has already succeeded or permanently failed."]
        pub async fn run_task(
            &mut self,
            request: impl tonic::IntoRequest<super::RunTaskRequest>,
        ) -> Result<tonic::Response<super::Task>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.tasks.v2.CloudTasks/RunTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
