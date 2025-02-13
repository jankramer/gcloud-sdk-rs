/// Client information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfo {
    /// Application ID, such as the package name on Android and the bundle
    /// identifier on iOS platforms.
    #[prost(string, tag = "1")]
    pub application_id: ::prost::alloc::string::String,
    /// Application version number, such as "1.2.3". The exact format is
    /// application-dependent.
    #[prost(string, tag = "2")]
    pub application_version: ::prost::alloc::string::String,
    /// Platform where the application is running.
    #[prost(enumeration = "client_info::Platform", tag = "3")]
    pub platform: i32,
    /// Operating system name and version as reported by the OS. For example,
    /// "Mac OS X 10.10.4". The exact format is platform-dependent.
    #[prost(string, tag = "4")]
    pub operating_system: ::prost::alloc::string::String,
    /// API client name and version. For example, the SDK calling the API. The
    /// exact format is up to the client.
    #[prost(string, tag = "5")]
    pub api_client: ::prost::alloc::string::String,
    /// Device model as reported by the device. The exact format is
    /// platform-dependent.
    #[prost(string, tag = "6")]
    pub device_model: ::prost::alloc::string::String,
    /// Language code (in BCP-47 format) indicating the UI language of the client.
    /// Examples are "en", "en-US" or "ja-Latn". For more information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
    #[prost(string, tag = "7")]
    pub language_code: ::prost::alloc::string::String,
    /// Build number/version of the operating system. e.g., the contents of
    /// android.os.Build.ID in Android, or the contents of sysctl "kern.osversion"
    /// in iOS.
    #[prost(string, tag = "8")]
    pub operating_system_build: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ClientInfo`.
pub mod client_info {
    /// Platform enum.
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
    pub enum Platform {
        /// Unspecified or unknown OS.
        Unspecified = 0,
        /// Development environment.
        Editor = 1,
        /// macOS.
        MacOs = 2,
        /// Windows.
        Windows = 3,
        /// Linux
        Linux = 4,
        /// Android
        Android = 5,
        /// iOS
        Ios = 6,
        /// WebGL.
        WebGl = 7,
    }
    impl Platform {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Platform::Unspecified => "PLATFORM_UNSPECIFIED",
                Platform::Editor => "EDITOR",
                Platform::MacOs => "MAC_OS",
                Platform::Windows => "WINDOWS",
                Platform::Linux => "LINUX",
                Platform::Android => "ANDROID",
                Platform::Ios => "IOS",
                Platform::WebGl => "WEB_GL",
            }
        }
    }
}
