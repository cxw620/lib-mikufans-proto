// This file is @generated by prost-build.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorderConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub color: ::core::option::Option<super::super::common::ColorConfig>,
    ///
    #[prost(double, tag = "2")]
    pub border_width: f64,
    ///
    #[prost(double, tag = "3")]
    pub ratio: f64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentDoubleClickConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub interaction: ::core::option::Option<Interaction>,
    ///
    #[prost(double, tag = "2")]
    pub animation_scale: f64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowActionConfig {
    ///
    #[prost(bool, tag = "1")]
    pub has_follow: bool,
    ///
    #[prost(message, optional, tag = "2")]
    pub icon_res: ::core::option::Option<super::super::common::ResourceSource>,
    ///
    #[prost(double, tag = "3")]
    pub border_width: f64,
    ///
    #[prost(message, optional, tag = "4")]
    pub border_color: ::core::option::Option<super::super::common::ColorConfig>,
    ///
    #[prost(int64, tag = "5")]
    pub mid: i64,
    ///
    #[prost(double, tag = "6")]
    pub icon_width_ratio: f64,
    ///
    #[prost(double, tag = "7")]
    pub icon_size_offset: f64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowIconConfig {
    ///
    #[prost(bool, tag = "1")]
    pub has_follow: bool,
    ///
    #[prost(message, optional, tag = "2")]
    pub icon_res: ::core::option::Option<super::super::common::ResourceSource>,
    ///
    #[prost(double, tag = "3")]
    pub border_width: f64,
    ///
    #[prost(message, optional, tag = "4")]
    pub border_color: ::core::option::Option<super::super::common::ColorConfig>,
    ///
    #[prost(int64, tag = "5")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GyroConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub gyroscope: ::core::option::Option<NftImageV2>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GyroscopeContentV2 {
    ///
    #[prost(string, tag = "1")]
    pub file_url: ::prost::alloc::string::String,
    ///
    #[prost(float, tag = "2")]
    pub scale: f32,
    ///
    #[prost(message, repeated, tag = "3")]
    pub physical_orientation: ::prost::alloc::vec::Vec<PhysicalOrientationV2>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GyroscopeEntityV2 {
    ///
    #[prost(string, tag = "1")]
    pub display_type: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<GyroscopeContentV2>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interaction {
    ///
    #[prost(string, tag = "1")]
    pub nft_id: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "2")]
    pub enabled: bool,
    ///
    #[prost(string, tag = "3")]
    pub itype: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub metadata_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveAnimeConfig {
    ///
    #[prost(bool, tag = "1")]
    pub is_live: bool,
    ///
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<LiveTextConfig>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<LiveAnimeItem>,
    ///
    #[prost(message, repeated, tag = "4")]
    pub border_config: ::prost::alloc::vec::Vec<BorderConfig>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveAnimeItem {
    ///
    #[prost(message, optional, tag = "1")]
    pub color: ::core::option::Option<super::super::common::ColorConfig>,
    ///
    #[prost(double, tag = "2")]
    pub start_ratio: f64,
    ///
    #[prost(double, tag = "3")]
    pub end_ratio: f64,
    ///
    #[prost(double, tag = "4")]
    pub start_stroke: f64,
    ///
    #[prost(double, tag = "5")]
    pub start_opacity: f64,
    ///
    #[prost(int64, tag = "6")]
    pub phase: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveTextConfig {
    ///
    #[prost(double, tag = "1")]
    pub width: f64,
    ///
    #[prost(double, tag = "2")]
    pub height: f64,
    ///
    #[prost(double, tag = "3")]
    pub offset_y: f64,
    ///
    #[prost(double, tag = "4")]
    pub border_width: f64,
    ///
    #[prost(double, tag = "5")]
    pub text_size: f64,
    ///
    #[prost(message, optional, tag = "7")]
    pub border_color: ::core::option::Option<super::super::common::ColorConfig>,
    ///
    #[prost(message, optional, tag = "8")]
    pub background: ::core::option::Option<super::super::common::ColorConfig>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftImageV2 {
    ///
    #[prost(message, repeated, tag = "1")]
    pub gyroscope: ::prost::alloc::vec::Vec<GyroscopeEntityV2>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalOrientationAnimation {
    ///
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(float, repeated, tag = "2")]
    pub value: ::prost::alloc::vec::Vec<f32>,
    ///
    #[prost(string, tag = "3")]
    pub bezier: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalOrientationV2 {
    ///
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(float, repeated, tag = "2")]
    pub angle: ::prost::alloc::vec::Vec<f32>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub animations: ::prost::alloc::vec::Vec<PhysicalOrientationAnimation>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct WebLiveAnimeConfig {
    ///
    #[prost(double, tag = "1")]
    pub circle_gap_width: f64,
    ///
    #[prost(double, tag = "2")]
    pub pink_circle_width: f64,
    ///
    #[prost(double, tag = "3")]
    pub live_label_width: f64,
    ///
    #[prost(double, tag = "4")]
    pub live_label_height: f64,
    ///
    #[prost(double, tag = "5")]
    pub live_label_offset_y: f64,
    ///
    #[prost(double, tag = "6")]
    pub live_label_border_width: f64,
}
