/// Identifies a package vulnerability found within a workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vulnerability {
    /// package name where vulnerability detected
    #[prost(string, tag = "1")]
    pub package_name: ::prost::alloc::string::String,
    /// affected package version
    #[prost(string, tag = "2")]
    pub affected_package_version: ::prost::alloc::string::String,
    /// title of vulnerability assigned by CVE
    #[prost(string, tag = "3")]
    pub cve_id: ::prost::alloc::string::String,
    /// cpe_uri where vulnerability detected
    #[prost(string, tag = "4")]
    pub cpe_uri: ::prost::alloc::string::String,
    /// assigned severity for vulnerability
    #[prost(enumeration = "Severity", tag = "5")]
    pub severity: i32,
    /// overall CVSS score
    #[prost(float, tag = "6")]
    pub cvss_score: f32,
    /// detailed CVSS score, format `CVSS:3.1/AV:A/AC:L/PR:N/UI:N/S:U/C:L/I:L/A:N`
    #[prost(string, tag = "7")]
    pub cvss_vector: ::prost::alloc::string::String,
    /// cpe_uri where vulnerability is fixed
    #[prost(string, tag = "8")]
    pub fixed_cpe_uri: ::prost::alloc::string::String,
    /// type of package (os, maven, go)
    #[prost(string, tag = "9")]
    pub package_type: ::prost::alloc::string::String,
    /// package name where vulnerability is fixed
    #[prost(string, tag = "10")]
    pub fixed_package: ::prost::alloc::string::String,
    /// fixed package version
    #[prost(string, tag = "11")]
    pub fixed_package_version: ::prost::alloc::string::String,
    /// detailed description
    #[prost(string, tag = "12")]
    pub description: ::prost::alloc::string::String,
    /// reference URL for source CVE database
    #[prost(string, repeated, tag = "13")]
    pub related_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// affected images
    #[prost(string, repeated, tag = "14")]
    pub affected_images: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A security concern for an asset(i.e cluster, workload, etc). Each finding
/// corresponds to a type of security concern. A finding is created during the
/// scan of an asset by any one of the GKE Security Posture features that are
/// enabled.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Finding {
    /// Fully qualified resource name of the k8s resource, e.g.:
    /// {api}/{version}/namespaces/{namespace}/{kind}/{workload name}
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The type of security finding this is.
    #[prost(enumeration = "FindingType", tag = "2")]
    pub r#type: i32,
    /// State determines whether the finding still exists or has been resolved.
    #[prost(enumeration = "finding::State", tag = "3")]
    pub state: i32,
    /// The human readable representation of the specific security finding.
    /// e.g. RUN_AS_NONROOT, CVE_ID_0 etc depending on the type.
    #[prost(string, tag = "4")]
    pub finding: ::prost::alloc::string::String,
    /// Severity determines the recommended actions for this finding.
    #[prost(enumeration = "Severity", tag = "5")]
    pub severity: i32,
    /// The time this finding was found/remediated.
    #[prost(message, optional, tag = "6")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Specific details about the security finding if there are any.
    #[prost(oneof = "finding::Details", tags = "7")]
    pub details: ::core::option::Option<finding::Details>,
}
/// Nested message and enum types in `Finding`.
pub mod finding {
    /// The current state of the finding(e.g still active, has been fixed etc).
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
    pub enum State {
        /// Default value, only used to determine that nothing was specified.
        Unspecified = 0,
        /// Active state means that the finding exists on the asset.
        Active = 1,
        /// Remediated means that the finding has been fixed on the asset.
        Remediated = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::Remediated => "REMEDIATED",
            }
        }
    }
    /// Specific details about the security finding if there are any.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        #[prost(message, tag = "7")]
        Vulnerability(super::Vulnerability),
    }
}
/// FindingType is an enumeration of all possible finding types in GKE Security
/// Posture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FindingType {
    /// Default value, unspecified.
    Unspecified = 0,
    /// Workload misconfiguration policy audit.
    Misconfig = 1,
    /// Workload vulnerabilities scanning.
    Vulnerability = 2,
}
impl FindingType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FindingType::Unspecified => "FINDING_TYPE_UNSPECIFIED",
            FindingType::Misconfig => "FINDING_TYPE_MISCONFIG",
            FindingType::Vulnerability => "FINDING_TYPE_VULNERABILITY",
        }
    }
}
/// Severity is an enumeration of all the possible severities of a violation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Severity {
    /// Default value, only used to determine that nothing was specified.
    Unspecified = 0,
    /// SEVERITY_CRITICAL recommends taking action immediately.
    Critical = 1,
    /// SEVERITY_HIGH recommends taking action if possible.
    High = 2,
    /// SEVERITY_MEDIUM recommends investigation.
    Medium = 3,
    /// SEVERITY_LOW recommends being aware of the problem.
    Low = 4,
}
impl Severity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Severity::Unspecified => "SEVERITY_UNSPECIFIED",
            Severity::Critical => "SEVERITY_CRITICAL",
            Severity::High => "SEVERITY_HIGH",
            Severity::Medium => "SEVERITY_MEDIUM",
            Severity::Low => "SEVERITY_LOW",
        }
    }
}
