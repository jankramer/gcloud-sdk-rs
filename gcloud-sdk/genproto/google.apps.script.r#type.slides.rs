/// Slides add-on manifest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlidesAddOnManifest {
    /// If present, this overrides the configuration from
    /// `addOns.common.homepageTrigger`.
    #[prost(message, optional, tag = "1")]
    pub homepage_trigger: ::core::option::Option<super::HomepageExtensionPoint>,
    /// Endpoint to execute when file scope authorization is granted
    /// for this document/user pair.
    #[prost(message, optional, tag = "2")]
    pub on_file_scope_granted_trigger: ::core::option::Option<SlidesExtensionPoint>,
}
/// Common format for declaring a Slides add-on's triggers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlidesExtensionPoint {
    /// Required. The endpoint to execute when this extension point is activated.
    #[prost(string, tag = "1")]
    pub run_function: ::prost::alloc::string::String,
}
