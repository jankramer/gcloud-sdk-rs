/// Drive add-on manifest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriveAddOnManifest {
    /// If present, this overrides the configuration from
    /// `addOns.common.homepageTrigger`.
    #[prost(message, optional, tag = "1")]
    pub homepage_trigger: ::core::option::Option<super::HomepageExtensionPoint>,
    /// Corresponds to behvior that should execute when items are selected
    /// in relevant Drive view (e.g. the My Drive Doclist).
    #[prost(message, optional, tag = "2")]
    pub on_items_selected_trigger: ::core::option::Option<DriveExtensionPoint>,
}
/// A generic extension point with common features, e.g. something that simply
/// needs a corresponding run function to work.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriveExtensionPoint {
    /// Required. The endpoint to execute when this extension point is
    /// activated.
    #[prost(string, tag = "1")]
    pub run_function: ::prost::alloc::string::String,
}
