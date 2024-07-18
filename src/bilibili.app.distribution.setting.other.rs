// This file is @generated by prost-build.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OtherSettingsConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub watermark_type: ::core::option::Option<super::super::Int64Value>,
    ///
    #[prost(message, optional, tag = "2")]
    pub web_image_quality_type: ::core::option::Option<super::super::Int64Value>,
    ///
    #[prost(message, optional, tag = "3")]
    pub enable_read_pasteboard: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "4")]
    pub paste_auto_jump: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "5")]
    pub mini_screen_play_when_back: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "6")]
    pub enable_resume_playing: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "7")]
    pub enable_wifi_auto_update: ::core::option::Option<super::super::BoolValue>,
    ///
    #[prost(message, optional, tag = "8")]
    pub enable_guide_screenshot_share: ::core::option::Option<super::super::BoolValue>,
}
/// Nested message and enum types in `OtherSettingsConfig`.
pub mod other_settings_config {
    ///
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
    pub enum WatermarkType {
        ///
        WtUnknow = 0,
        ///
        None = 1,
        ///
        Center = 2,
        ///
        BottomRight = 3,
    }
    impl WatermarkType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WatermarkType::WtUnknow => "WT_UNKNOW",
                WatermarkType::None => "None",
                WatermarkType::Center => "Center",
                WatermarkType::BottomRight => "BottomRight",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WT_UNKNOW" => Some(Self::WtUnknow),
                "None" => Some(Self::None),
                "Center" => Some(Self::Center),
                "BottomRight" => Some(Self::BottomRight),
                _ => None,
            }
        }
    }
    ///
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
    pub enum WebImageQualityType {
        ///
        High = 0,
        ///
        Low = 1,
        ///
        Auto = 2,
    }
    impl WebImageQualityType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WebImageQualityType::High => "High",
                WebImageQualityType::Low => "Low",
                WebImageQualityType::Auto => "Auto",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "High" => Some(Self::High),
                "Low" => Some(Self::Low),
                "Auto" => Some(Self::Auto),
                _ => None,
            }
        }
    }
}
