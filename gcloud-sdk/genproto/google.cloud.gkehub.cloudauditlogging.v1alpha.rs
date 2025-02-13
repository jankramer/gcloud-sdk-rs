/// **Cloud Audit Logging**: Spec for Audit Logging Allowlisting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureSpec {
    /// Service account that should be allowlisted to send the audit logs; eg
    /// cloudauditlogging@gcp-project.iam.gserviceaccount.com. These accounts must
    /// already exist, but do not need to have any permissions granted to them.
    /// The customer's entitlements will be checked prior to allowlisting (i.e.
    /// the customer must be an Anthos customer.)
    #[prost(string, repeated, tag = "1")]
    pub allowlisted_service_accounts: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
