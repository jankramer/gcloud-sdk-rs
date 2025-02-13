/// Represents a Interactive Canvas response to be sent to the user.
/// This can be used in conjunction with the `first_simple` field in the
/// containing prompt to speak to the user in addition to displaying a
/// interactive canvas response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticCanvasPrompt {
    /// Required. URL of the web view to load.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// Optional. JSON data to be passed through to the immersive experience web page as an
    /// event. If the `override` field in the containing prompt is `false` data
    /// values defined in this Canvas prompt will be added after data values
    /// defined in previous Canvas prompts.
    #[prost(message, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<::prost_types::Value>,
    /// Optional. A true value means that the mic won't be opened for capturing input after
    /// this immersive response is presented to the user.
    #[prost(bool, tag = "3")]
    pub suppress_mic: bool,
    /// Optional. If `true`, conversation related metadata is included and send back to the
    /// canvas application.
    #[prost(bool, tag = "5")]
    pub send_state_data_to_canvas_app: bool,
    /// Optional. If `true` the canvas application occupies the full screen and won't
    /// have a header at the top. A toast message will also be displayed on the
    /// loading screen that includes the Action's display name, the developer's
    /// name, and instructions for exiting the Action. Default value: `false`.
    #[prost(bool, tag = "6")]
    pub enable_full_screen: bool,
}
/// An image displayed in the card.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticImagePrompt {
    /// Required. The source url of the image. Images can be JPG, PNG and GIF (animated and
    /// non-animated). For example,`<https://www.agentx.com/logo.png`.>
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// Required. A text description of the image to be used for accessibility, e.g. screen
    /// readers.
    #[prost(string, tag = "2")]
    pub alt: ::prost::alloc::string::String,
    /// Optional. The height of the image in pixels.
    #[prost(int32, tag = "3")]
    pub height: i32,
    /// Optional. The width of the image in pixels.
    #[prost(int32, tag = "4")]
    pub width: i32,
}
/// Nested message and enum types in `StaticImagePrompt`.
pub mod static_image_prompt {
    /// Possible image display options for affecting the presentation of the image.
    /// This should be used for when the image's aspect ratio does not match the
    /// image container's aspect ratio.
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
    pub enum ImageFill {
        /// ImageFill unspecified.
        Unspecified = 0,
        /// Fill the gaps between the image and the image container with gray bars.
        Gray = 1,
        /// Fill the gaps between the image and the image container with white bars.
        White = 2,
        /// Image is scaled such that the image width and height match or exceed the
        /// container dimensions. This may crop the top and bottom of the image if
        /// the scaled image height is greater than the container height, or crop the
        /// left and right of the image if the scaled image width is greater than the
        /// container width. This is similar to "Zoom Mode" on a widescreen TV when
        /// playing a 4:3 video.
        Cropped = 3,
    }
    impl ImageFill {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ImageFill::Unspecified => "UNSPECIFIED",
                ImageFill::Gray => "GRAY",
                ImageFill::White => "WHITE",
                ImageFill::Cropped => "CROPPED",
            }
        }
    }
}
/// Defines a link which will be displayed as a suggestion chip and can be opened
/// by the user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticLinkPrompt {
    /// Name of the link
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Defines behavior when the user opens the link.
    #[prost(message, optional, tag = "2")]
    pub open: ::core::option::Option<OpenUrl>,
}
/// Defines behavior when the user opens the link.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenUrl {
    /// The url field which could be any of:
    /// - http/https urls for opening an App-linked App or a webpage
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// Indicates a hint for the url type.
    #[prost(enumeration = "UrlHint", tag = "2")]
    pub hint: i32,
}
/// Different types of url hints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UrlHint {
    /// Unspecified
    HintUnspecified = 0,
    /// URL that points directly to AMP content, or to a canonical URL
    /// which refers to AMP content via `<link rel="amphtml">`.
    Amp = 1,
}
impl UrlHint {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UrlHint::HintUnspecified => "HINT_UNSPECIFIED",
            UrlHint::Amp => "AMP",
        }
    }
}
/// A basic card for displaying some information, e.g. an image and/or text.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticCardPrompt {
    /// Optional. Overall title of the card.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Optional. Subtitle of the card.
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    /// Required. Body text of the card which is needed unless image is present. Supports a
    /// limited set of markdown syntax for formatting.
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    /// Optional. A hero image for the card. The height is fixed to 192dp.
    #[prost(message, optional, tag = "4")]
    pub image: ::core::option::Option<StaticImagePrompt>,
    /// Optional. How the image background will be filled.
    #[prost(enumeration = "static_image_prompt::ImageFill", tag = "5")]
    pub image_fill: i32,
    /// Optional. A clickable button to be shown in the Card.
    #[prost(message, optional, tag = "6")]
    pub button: ::core::option::Option<StaticLinkPrompt>,
}
/// Presents a set of web documents as a collection of large-tile items. Items
/// may be selected to launch their associated web document in a web viewer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticCollectionBrowsePrompt {
    /// Items in the browse collection. The list size should be in the range [2,
    /// 10].
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<
        static_collection_browse_prompt::CollectionBrowseItem,
    >,
    /// Image display option for images in the collection.
    #[prost(enumeration = "static_image_prompt::ImageFill", tag = "2")]
    pub image_fill: i32,
}
/// Nested message and enum types in `StaticCollectionBrowsePrompt`.
pub mod static_collection_browse_prompt {
    /// Item in the collection.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CollectionBrowseItem {
        /// Required. Title of the collection item.
        #[prost(string, tag = "1")]
        pub title: ::prost::alloc::string::String,
        /// Description of the collection item.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// Footer text for the collection item, displayed below the description.
        /// Single line of text, truncated with an ellipsis.
        #[prost(string, tag = "3")]
        pub footer: ::prost::alloc::string::String,
        /// Image for the collection item.
        #[prost(message, optional, tag = "4")]
        pub image: ::core::option::Option<super::StaticImagePrompt>,
        /// Required. URI to open if the item selected.
        #[prost(message, optional, tag = "5")]
        pub open_uri_action: ::core::option::Option<super::OpenUrl>,
    }
}
/// A card for presenting a collection of options to select from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticCollectionPrompt {
    /// Optional. Title of the collection.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Optional. Subtitle of the collection.
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    /// Required. Collection items.
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<static_collection_prompt::CollectionItem>,
    /// Optional. Type of image display option.
    #[prost(enumeration = "static_image_prompt::ImageFill", tag = "4")]
    pub image_fill: i32,
}
/// Nested message and enum types in `StaticCollectionPrompt`.
pub mod static_collection_prompt {
    /// An item in the collection.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CollectionItem {
        /// Required. The NLU key that matches the entry key name in the associated
        /// Type. When item tapped, this key will be posted back as a select option
        /// parameter.
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// Required. Title of the item. When tapped, this text will be
        /// posted back to the conversation verbatim as if the user had typed it.
        /// Each title must be unique among the set of items.
        #[prost(string, tag = "2")]
        pub title: ::prost::alloc::string::String,
        /// Optional. Body text of the item.
        #[prost(string, tag = "3")]
        pub description: ::prost::alloc::string::String,
        /// Optional. Item image.
        #[prost(message, optional, tag = "4")]
        pub image: ::core::option::Option<super::StaticImagePrompt>,
    }
}
/// A card for presenting a list of options to select from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticListPrompt {
    /// Optional. Title of the list.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Optional. Subtitle of the list.
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    /// Required. List items.
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<static_list_prompt::ListItem>,
}
/// Nested message and enum types in `StaticListPrompt`.
pub mod static_list_prompt {
    /// An item in the list.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListItem {
        /// Required. The NLU key that matches the entry key name in the associated type. When
        /// item tapped, this key will be posted back as a select option parameter.
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// Required. Title of the item. When tapped, this text will be posted back to the
        /// conversation verbatim as if the user had typed it. Each title must be
        /// unique among the set of items.
        #[prost(string, tag = "2")]
        pub title: ::prost::alloc::string::String,
        /// Optional. Body text of the item.
        #[prost(string, tag = "3")]
        pub description: ::prost::alloc::string::String,
        /// Optional. Item image.
        #[prost(message, optional, tag = "4")]
        pub image: ::core::option::Option<super::StaticImagePrompt>,
    }
}
/// Contains information about the media, such as name, description, url, etc.
/// Next id: 11
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticMediaPrompt {
    /// Media type of this response.
    #[prost(enumeration = "static_media_prompt::MediaType", tag = "8")]
    pub media_type: i32,
    /// Start offset of the first media object.
    #[prost(message, optional, tag = "5")]
    pub start_offset: ::core::option::Option<::prost_types::Duration>,
    /// Optional media control types this media response session can support.
    /// If set, request will be made to 3p when a certain media event happens.
    /// If not set, 3p must still handle two default control type, FINISHED and
    /// FAILED.
    #[prost(
        enumeration = "static_media_prompt::OptionalMediaControls",
        repeated,
        tag = "6"
    )]
    pub optional_media_controls: ::prost::alloc::vec::Vec<i32>,
    /// List of media objects.
    #[prost(message, repeated, tag = "7")]
    pub media_objects: ::prost::alloc::vec::Vec<MediaObject>,
    /// Repeat mode for the list of Media Objects.
    #[prost(enumeration = "static_media_prompt::RepeatMode", tag = "9")]
    pub repeat_mode: i32,
}
/// Nested message and enum types in `StaticMediaPrompt`.
pub mod static_media_prompt {
    /// Media type of this response.
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
    pub enum MediaType {
        /// UNSPECIFIED value
        Unspecified = 0,
        /// Audio file.
        Audio = 1,
        /// Response to acknowledge a media status report.
        MediaStatusAck = 2,
    }
    impl MediaType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MediaType::Unspecified => "MEDIA_TYPE_UNSPECIFIED",
                MediaType::Audio => "AUDIO",
                MediaType::MediaStatusAck => "MEDIA_STATUS_ACK",
            }
        }
    }
    /// Media control types the media response can supported optionally
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
    pub enum OptionalMediaControls {
        /// Unspecified value
        Unspecified = 0,
        /// Paused event. Triggered when user pauses the media.
        Paused = 1,
        /// Stopped event. Triggered when user exit out 3p session during media play.
        Stopped = 2,
    }
    impl OptionalMediaControls {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OptionalMediaControls::Unspecified => {
                    "OPTIONAL_MEDIA_CONTROLS_UNSPECIFIED"
                }
                OptionalMediaControls::Paused => "PAUSED",
                OptionalMediaControls::Stopped => "STOPPED",
            }
        }
    }
    /// The types of repeat mode for a list of media objects.
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
    pub enum RepeatMode {
        /// Equivalent to OFF.
        Unspecified = 0,
        /// End media session at the end of the last media object.
        Off = 1,
        /// Loop to the beginning of the first media object when the end of the last
        /// media object is reached.
        All = 2,
    }
    impl RepeatMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RepeatMode::Unspecified => "REPEAT_MODE_UNSPECIFIED",
                RepeatMode::Off => "OFF",
                RepeatMode::All => "ALL",
            }
        }
    }
}
/// Represents a single media object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaObject {
    /// Name of this media object.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Description of this media object.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The url pointing to the media content.
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    /// Image to show with the media card.
    #[prost(message, optional, tag = "4")]
    pub image: ::core::option::Option<MediaImage>,
}
/// Image to be shown inside a MediaPrompt.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaImage {
    /// Only one type of MediaImage is allowed.
    #[prost(oneof = "media_image::Image", tags = "1, 2")]
    pub image: ::core::option::Option<media_image::Image>,
}
/// Nested message and enum types in `MediaImage`.
pub mod media_image {
    /// Only one type of MediaImage is allowed.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Image {
        /// A large image, such as the cover of the album, etc.
        #[prost(message, tag = "1")]
        Large(super::StaticImagePrompt),
        /// A small image icon displayed on the right from the title.
        /// It's resized to 36x36 dp.
        #[prost(message, tag = "2")]
        Icon(super::StaticImagePrompt),
    }
}
/// A table card for displaying a table of text.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticTablePrompt {
    /// Optional. Overall title of the table. Must be set if subtitle is set.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Optional. Subtitle for the table.
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    /// Optional. Image associated with the table.
    #[prost(message, optional, tag = "3")]
    pub image: ::core::option::Option<StaticImagePrompt>,
    /// Optional. Headers and alignment of columns.
    #[prost(message, repeated, tag = "4")]
    pub columns: ::prost::alloc::vec::Vec<TableColumn>,
    /// Optional. Row data of the table. The first 3 rows are guaranteed to be shown but
    /// others might be cut on certain surfaces. Please test with the simulator to
    /// see which rows will be shown for a given surface. On surfaces that support
    /// the `WEB_BROWSER` capability, you can point the user to
    /// a web page with more data.
    #[prost(message, repeated, tag = "5")]
    pub rows: ::prost::alloc::vec::Vec<TableRow>,
    /// Optional. Button.
    #[prost(message, optional, tag = "6")]
    pub button: ::core::option::Option<StaticLinkPrompt>,
}
/// Describes a column in the table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableColumn {
    /// Header text for the column.
    #[prost(string, tag = "1")]
    pub header: ::prost::alloc::string::String,
    /// Horizontal alignment of content w.r.t column. If unspecified, content
    /// will be aligned to the leading edge.
    #[prost(enumeration = "table_column::HorizontalAlignment", tag = "2")]
    pub align: i32,
}
/// Nested message and enum types in `TableColumn`.
pub mod table_column {
    /// The alignment of the content within the cell.
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
    pub enum HorizontalAlignment {
        /// HorizontalAlignment unspecified.
        Unspecified = 0,
        /// Leading edge of the cell. This is the default.
        Leading = 1,
        /// Content is aligned to the center of the column.
        Center = 2,
        /// Content is aligned to the trailing edge of the column.
        Trailing = 3,
    }
    impl HorizontalAlignment {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HorizontalAlignment::Unspecified => "UNSPECIFIED",
                HorizontalAlignment::Leading => "LEADING",
                HorizontalAlignment::Center => "CENTER",
                HorizontalAlignment::Trailing => "TRAILING",
            }
        }
    }
}
/// Describes a cell in a row.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableCell {
    /// Text content of the cell.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
/// Describes a row in the table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableRow {
    /// Cells in this row. The first 3 cells are guaranteed to be shown but
    /// others might be cut on certain surfaces. Please test with the simulator
    /// to see which cells will be shown for a given surface.
    #[prost(message, repeated, tag = "1")]
    pub cells: ::prost::alloc::vec::Vec<TableCell>,
    /// Indicates whether there should be a divider after each row.
    #[prost(bool, tag = "2")]
    pub divider: bool,
}
/// A placeholder for the Content part of a StaticPrompt.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticContentPrompt {
    /// Only one type of content can be present in a Prompt.
    #[prost(oneof = "static_content_prompt::Content", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub content: ::core::option::Option<static_content_prompt::Content>,
}
/// Nested message and enum types in `StaticContentPrompt`.
pub mod static_content_prompt {
    /// Only one type of content can be present in a Prompt.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// A basic card.
        #[prost(message, tag = "1")]
        Card(super::StaticCardPrompt),
        /// An image.
        #[prost(message, tag = "2")]
        Image(super::StaticImagePrompt),
        /// Table card.
        #[prost(message, tag = "3")]
        Table(super::StaticTablePrompt),
        /// Response indicating a set of media to be played.
        #[prost(message, tag = "4")]
        Media(super::StaticMediaPrompt),
        /// A card for presenting a list of options to select from.
        #[prost(message, tag = "5")]
        List(super::StaticListPrompt),
        /// A card presenting a list of options to select from.
        #[prost(message, tag = "6")]
        Collection(super::StaticCollectionPrompt),
        /// A card presenting a collection of web pages to open.
        #[prost(message, tag = "7")]
        CollectionBrowse(super::StaticCollectionBrowsePrompt),
    }
}
/// Represents a simple prompt to be send to a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticSimplePrompt {
    /// List of possible variants.
    #[prost(message, repeated, tag = "1")]
    pub variants: ::prost::alloc::vec::Vec<static_simple_prompt::Variant>,
}
/// Nested message and enum types in `StaticSimplePrompt`.
pub mod static_simple_prompt {
    /// Represents a variant which is part of the simple prompt.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Variant {
        /// Optional. Represents the speech to be spoken to the user.  Can be SSML or text to
        /// speech.
        /// By default, speech will be appended to previous Simple prompt's
        /// speech. If the `override` field in the containing prompt is `true` the
        /// speech defined in this field will override previous Simple prompt's
        /// speech.
        #[prost(string, tag = "1")]
        pub speech: ::prost::alloc::string::String,
        /// Optional. Text to display in the chat bubble. If not given, a display rendering of
        /// the speech field above will be used. Limited to 640 chars.
        /// By default, text will be appended to previous Simple prompt's text.
        /// If the `override` field in the containing prompt is `true` the text
        /// defined in this field will override previous Simple prompt's text.
        #[prost(string, tag = "2")]
        pub text: ::prost::alloc::string::String,
    }
}
/// Represents a suggestion chip, a UI element shown to the user for convenience.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Suggestion {
    /// Required. The text shown in the suggestion chip. When tapped, this text will be
    /// posted back to the conversation verbatim as if the user had typed it.
    /// Each title must be unique among the set of suggestion chips.
    /// Max 25 chars
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
}
/// Represents the surface the user is using to make a request to the Action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SurfaceCapabilities {
    /// Required. The capabilities of the surface making a request to the Action.
    #[prost(
        enumeration = "surface_capabilities::Capability",
        repeated,
        packed = "false",
        tag = "1"
    )]
    pub capabilities: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `SurfaceCapabilities`.
pub mod surface_capabilities {
    /// Capabilities the device surface supports at the time of the request.
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
    pub enum Capability {
        /// Unspecified surface capability.
        Unspecified = 0,
        /// Device can speak to the user via text-to-speech or SSML.
        Speech = 1,
        /// Device can display rich responses like cards, lists and tables.
        RichResponse = 2,
        /// Device can play long form audio media like music and podcasts.
        LongFormAudio = 3,
        /// Device can display a interactive canvas response.
        InteractiveCanvas = 4,
        /// Device can use web links in rich responses to open a web browser.
        WebLink = 5,
        /// Device can support saving and fetching home storage.
        HomeStorage = 6,
    }
    impl Capability {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Capability::Unspecified => "UNSPECIFIED",
                Capability::Speech => "SPEECH",
                Capability::RichResponse => "RICH_RESPONSE",
                Capability::LongFormAudio => "LONG_FORM_AUDIO",
                Capability::InteractiveCanvas => "INTERACTIVE_CANVAS",
                Capability::WebLink => "WEB_LINK",
                Capability::HomeStorage => "HOME_STORAGE",
            }
        }
    }
}
/// Represents a list of prompt candidates, one of which will be selected as the
/// prompt to be shown in the response to the user.
/// **This message is localizable.**
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticPrompt {
    /// The list of candidate prompts to be sent to the client. Each prompt has a
    /// selector to determine when it can be used. The first selector that matches
    /// a request will be sent and the rest will be ignored.
    #[prost(message, repeated, tag = "1")]
    pub candidates: ::prost::alloc::vec::Vec<static_prompt::StaticPromptCandidate>,
}
/// Nested message and enum types in `StaticPrompt`.
pub mod static_prompt {
    /// Represents a static prompt candidate.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StaticPromptCandidate {
        /// Optional. The criteria for whether this prompt matches a request. If the selector
        /// is empty, this prompt will always be triggered.
        #[prost(message, optional, tag = "1")]
        pub selector: ::core::option::Option<Selector>,
        /// The prompt response associated with the selector.
        #[prost(message, optional, tag = "2")]
        pub prompt_response: ::core::option::Option<
            static_prompt_candidate::StaticPromptResponse,
        >,
    }
    /// Nested message and enum types in `StaticPromptCandidate`.
    pub mod static_prompt_candidate {
        /// Represents structured responses to send to the user, such as text,
        /// speech, cards, canvas data, suggestion chips, etc.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct StaticPromptResponse {
            /// Optional. The first voice and text-only response.
            #[prost(message, optional, tag = "2")]
            pub first_simple: ::core::option::Option<super::super::StaticSimplePrompt>,
            /// Optional. A content like a card, list or media to display to the user.
            #[prost(message, optional, tag = "3")]
            pub content: ::core::option::Option<super::super::StaticContentPrompt>,
            /// Optional. The last voice and text-only response.
            #[prost(message, optional, tag = "4")]
            pub last_simple: ::core::option::Option<super::super::StaticSimplePrompt>,
            /// Optional. Suggestions to be displayed to the user which will always
            /// appear at the end of the response. If the `append` field in the
            /// containing prompt is `true` the titles defined in this field will be
            /// added to titles defined in any previously defined suggestions prompts
            /// and duplicate values will be removed.
            #[prost(message, repeated, tag = "5")]
            pub suggestions: ::prost::alloc::vec::Vec<super::super::Suggestion>,
            /// Optional. An additional suggestion chip that can link out to the associated app
            /// or site.
            /// The chip will be rendered with the title "Open <name>". Max 20 chars.
            #[prost(message, optional, tag = "6")]
            pub link: ::core::option::Option<super::super::StaticLinkPrompt>,
            /// Optional. Mode for how this messages should be merged with previously defined
            /// messages.
            /// `true` will clear all previously defined messages (first and last
            /// simple, content, suggestions link and canvas) and add messages defined
            /// in this prompt. `false` will add messages defined in this prompt to
            /// messages defined in previous responses. Setting this field to `false`
            /// will also enable appending to some fields inside Simple prompts, the
            /// Suggestions prompt and the Canvas prompt (part of the Content prompt).
            /// The Content and Link messages will always be overwritten if defined in
            /// the prompt. Default value is `false`.
            #[prost(bool, tag = "7")]
            pub r#override: bool,
            /// A response to be used for interactive canvas experience.
            #[prost(message, optional, tag = "8")]
            pub canvas: ::core::option::Option<super::super::StaticCanvasPrompt>,
        }
    }
    /// Defines the criteria for whether a prompt matches a request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Selector {
        /// The set of required surface capabilities.
        #[prost(message, optional, tag = "1")]
        pub surface_capabilities: ::core::option::Option<super::SurfaceCapabilities>,
    }
}
