// This file is @generated by prost-build.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbTest {
    ///
    #[prost(map = "string, int32", tag = "1")]
    pub exp: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatalogueInfo {
    ///
    #[prost(message, repeated, tag = "1")]
    pub catalogues: ::prost::alloc::vec::Vec<SeasonCatalogue>,
    ///
    #[prost(message, optional, tag = "2")]
    pub catalogue_live_info: ::core::option::Option<CatalogueLiveInfo>,
    ///
    #[prost(string, tag = "3")]
    pub catalogue_update_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatalogueLiveInfo {
    ///
    #[prost(int64, tag = "1")]
    pub episode_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub button_text: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "LiveStatus", tag = "5")]
    pub status: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CourseCoach {
    ///
    #[prost(string, tag = "1")]
    pub detail_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub directory_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Courseware {
    ///
    #[prost(int64, tag = "1")]
    pub file_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub file_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub file_type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub file_size: i64,
    ///
    #[prost(string, tag = "5")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub file_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoursewareInfo {
    ///
    #[prost(message, repeated, tag = "1")]
    pub coursewares: ::prost::alloc::vec::Vec<Courseware>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CredentialInfo {
    ///
    #[prost(bool, tag = "1")]
    pub tab_show: bool,
    ///
    #[prost(bool, tag = "2")]
    pub tab_badge_show: bool,
    ///
    #[prost(string, tag = "3")]
    pub tab_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmakuControl {
    ///
    #[prost(bool, tag = "1")]
    pub disabled: bool,
    ///
    #[prost(message, optional, tag = "2")]
    pub extra_content: ::core::option::Option<ExtraContent>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Episode {
    ///
    #[prost(enumeration = "EpisodeType", tag = "1")]
    pub r#type: i32,
    ///
    #[prost(oneof = "episode::Data", tags = "2, 3")]
    pub data: ::core::option::Option<episode::Data>,
}
/// Nested message and enum types in `Episode`.
pub mod episode {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        ///
        #[prost(message, tag = "2")]
        VideoEpisode(super::VideoEpisode),
        ///
        #[prost(message, tag = "3")]
        LiveEpisode(super::LiveEpisode),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct EpisodeDimension {
    ///
    #[prost(int64, tag = "1")]
    pub width: i64,
    ///
    #[prost(int64, tag = "2")]
    pub height: i64,
    ///
    #[prost(int64, tag = "3")]
    pub rotate: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpisodeHistory {
    ///
    #[prost(bool, tag = "1")]
    pub last_play: bool,
    ///
    #[prost(string, tag = "2")]
    pub last_play_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub max_progress: i64,
    ///
    #[prost(string, tag = "4")]
    pub full_watched_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpisodeLabel {
    ///
    #[prost(string, tag = "1")]
    pub type_label: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpisodeSelectionLabel {
    ///
    #[prost(string, tag = "1")]
    pub type_label: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtraContent {
    ///
    #[prost(string, tag = "1")]
    pub disabled_reason: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GiftInfo {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveEpisode {
    ///
    #[prost(int64, tag = "1")]
    pub episode_id: i64,
    ///
    #[prost(enumeration = "LiveStatus", tag = "2")]
    pub status: i32,
    ///
    #[prost(bool, tag = "3")]
    pub jump: bool,
    ///
    #[prost(int64, tag = "4")]
    pub live_teacher_mid: i64,
    ///
    #[prost(bool, tag = "5")]
    pub show_subscription_prebook_button: bool,
    ///
    #[prost(bool, tag = "6")]
    pub be_subscription_prebook: bool,
    ///
    #[prost(int64, tag = "7")]
    pub index: i64,
    ///
    #[prost(string, tag = "8")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub play_way_subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "11")]
    pub show_lock_icon: bool,
    ///
    #[prost(message, optional, tag = "12")]
    pub episode_label: ::core::option::Option<EpisodeLabel>,
    ///
    #[prost(message, optional, tag = "13")]
    pub selection_label: ::core::option::Option<EpisodeSelectionLabel>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LiveInfo {
    ///
    #[prost(int64, tag = "1")]
    pub episode_id: i64,
    ///
    #[prost(enumeration = "LiveStatus", tag = "2")]
    pub status: i32,
    ///
    #[prost(bool, tag = "3")]
    pub jump: bool,
    ///
    #[prost(int64, tag = "4")]
    pub live_teacher_mid: i64,
    ///
    #[prost(bool, tag = "5")]
    pub show_subscription_prebook_button: bool,
    ///
    #[prost(bool, tag = "6")]
    pub be_subscription_prebook: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetdiskCourseware {
    ///
    #[prost(int64, tag = "1")]
    pub courseware_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub remark: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetdiskCoursewareInfo {
    ///
    #[prost(message, repeated, tag = "1")]
    pub coursewares: ::prost::alloc::vec::Vec<NetdiskCourseware>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationArea {
    ///
    #[prost(message, repeated, tag = "1")]
    pub buttons: ::prost::alloc::vec::Vec<OperationAreaButton>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationAreaButton {
    ///
    #[prost(enumeration = "OperationAreaButtonType", tag = "1")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "3")]
    pub disabled: bool,
    ///
    #[prost(string, tag = "4")]
    pub link: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostPurchaseInfo {
    ///
    #[prost(string, tag = "1")]
    pub pop_up_pic: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub pop_up_interval: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonCatalogue {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub index: i64,
    ///
    #[prost(int64, tag = "3")]
    pub start_ep_index: i64,
    ///
    #[prost(int64, tag = "4")]
    pub end_ep_index: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonCoupon {
    ///
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub start_time: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub expire_time: ::prost::alloc::string::String,
    ///
    #[prost(double, tag = "5")]
    pub amount: f64,
    ///
    #[prost(string, tag = "6")]
    pub show_amount: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "SeasonCouponStatus", tag = "7")]
    pub status: i32,
    ///
    #[prost(enumeration = "SeasonCouponType", tag = "8")]
    pub coupon_type: i32,
    ///
    #[prost(string, tag = "9")]
    pub short_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub expire_minute: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub use_scope: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub discount_amount: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "13")]
    pub receive_expire_time: i64,
    ///
    #[prost(int64, tag = "14")]
    pub use_expire_time: i64,
    ///
    #[prost(string, tag = "15")]
    pub scene_mark: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "16")]
    pub scene_background_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "17")]
    pub scene_benefit_img: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "18")]
    pub scene_countdown: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonCustom {
    ///
    #[prost(message, optional, tag = "1")]
    pub water_mark: ::core::option::Option<WaterMark>,
    ///
    #[prost(message, optional, tag = "2")]
    pub danmaku_control: ::core::option::Option<DanmakuControl>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonOverview {
    ///
    #[prost(int64, tag = "1")]
    pub season_id: i64,
    ///
    #[prost(bool, tag = "2")]
    pub support_cash: bool,
    ///
    #[prost(enumeration = "SeasonStatus", tag = "3")]
    pub status: i32,
    ///
    #[prost(enumeration = "SeasonType", tag = "4")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonPayment {
    ///
    #[prost(string, tag = "1")]
    pub price_unit: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub discount_price: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub discount_price_desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub original_price: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub original_price_desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonSection {
    ///
    #[prost(int64, tag = "1")]
    pub section_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "SeasonSectionType", tag = "3")]
    pub r#type: i32,
    ///
    #[prost(message, repeated, tag = "4")]
    pub coursewares: ::prost::alloc::vec::Vec<Courseware>,
    ///
    #[prost(message, repeated, tag = "5")]
    pub episodes: ::prost::alloc::vec::Vec<Episode>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SeasonStat {
    ///
    #[prost(int64, tag = "1")]
    pub fav: i64,
    ///
    #[prost(int64, tag = "2")]
    pub share: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SectionInfo {
    ///
    #[prost(message, repeated, tag = "1")]
    pub sections: ::prost::alloc::vec::Vec<SeasonSection>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoEpisode {
    ///
    #[prost(message, repeated, tag = "1")]
    pub coursewares: ::prost::alloc::vec::Vec<Courseware>,
    ///
    #[prost(int64, tag = "2")]
    pub index: i64,
    ///
    #[prost(enumeration = "EpisodeStatus", tag = "3")]
    pub status: i32,
    ///
    #[prost(int64, tag = "4")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "5")]
    pub cid: i64,
    ///
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub play_way_subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "9")]
    pub duration: i64,
    ///
    #[prost(bool, tag = "10")]
    pub can_switch: bool,
    ///
    #[prost(bool, tag = "11")]
    pub can_play: bool,
    ///
    #[prost(bool, tag = "12")]
    pub show_lock_icon: bool,
    ///
    #[prost(message, optional, tag = "13")]
    pub episode_label: ::core::option::Option<EpisodeLabel>,
    ///
    #[prost(message, optional, tag = "14")]
    pub selection_label: ::core::option::Option<EpisodeSelectionLabel>,
    ///
    #[prost(message, optional, tag = "15")]
    pub history: ::core::option::Option<EpisodeHistory>,
    ///
    #[prost(enumeration = "VideoPlayWay", tag = "16")]
    pub play_way: i32,
    ///
    #[prost(int64, tag = "17")]
    pub episode_id: i64,
    ///
    #[prost(string, tag = "18")]
    pub share_link: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "19")]
    pub dimension: ::core::option::Option<EpisodeDimension>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewPugvAny {
    ///
    #[prost(message, optional, tag = "1")]
    pub season_overview: ::core::option::Option<SeasonOverview>,
    ///
    #[prost(message, optional, tag = "2")]
    pub season_payment: ::core::option::Option<SeasonPayment>,
    ///
    #[prost(message, optional, tag = "3")]
    pub season_coupon: ::core::option::Option<SeasonCoupon>,
    ///
    #[prost(message, optional, tag = "4")]
    pub catalogue_info: ::core::option::Option<CatalogueInfo>,
    ///
    #[prost(message, optional, tag = "5")]
    pub season_custom: ::core::option::Option<SeasonCustom>,
    ///
    #[prost(message, optional, tag = "6")]
    pub courseware_info: ::core::option::Option<CoursewareInfo>,
    ///
    #[prost(message, optional, tag = "7")]
    pub course_coach: ::core::option::Option<CourseCoach>,
    ///
    #[prost(message, optional, tag = "8")]
    pub section_info: ::core::option::Option<SectionInfo>,
    ///
    #[prost(message, optional, tag = "9")]
    pub ab_test: ::core::option::Option<AbTest>,
    ///
    #[prost(message, optional, tag = "10")]
    pub operation_area: ::core::option::Option<OperationArea>,
    ///
    #[prost(message, optional, tag = "11")]
    pub season_stat: ::core::option::Option<SeasonStat>,
    ///
    #[prost(message, optional, tag = "12")]
    pub post_purchase_info: ::core::option::Option<PostPurchaseInfo>,
    ///
    #[prost(message, optional, tag = "13")]
    pub gift_info: ::core::option::Option<GiftInfo>,
    ///
    #[prost(message, optional, tag = "14")]
    pub netdisk_courseware_info: ::core::option::Option<NetdiskCoursewareInfo>,
    ///
    #[prost(message, optional, tag = "15")]
    pub credential_info: ::core::option::Option<CredentialInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct WaterMark {
    ///
    #[prost(bool, tag = "1")]
    pub show_watermark: bool,
    ///
    #[prost(int64, tag = "2")]
    pub watermark_interval: i64,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EpisodeStatus {
    ///
    Unspecified = 0,
    ///
    TryWatchWhole = 1,
    ///
    NotTryWatch = 2,
    ///
    TryWatch5Minutes = 3,
}
impl EpisodeStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EpisodeStatus::Unspecified => "EPISODE_STATUS_UNSPECIFIED",
            EpisodeStatus::TryWatchWhole => "EPISODE_STATUS_TRY_WATCH_WHOLE",
            EpisodeStatus::NotTryWatch => "EPISODE_STATUS_NOT_TRY_WATCH",
            EpisodeStatus::TryWatch5Minutes => "EPISODE_STATUS_TRY_WATCH_5_MINUTES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EPISODE_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "EPISODE_STATUS_TRY_WATCH_WHOLE" => Some(Self::TryWatchWhole),
            "EPISODE_STATUS_NOT_TRY_WATCH" => Some(Self::NotTryWatch),
            "EPISODE_STATUS_TRY_WATCH_5_MINUTES" => Some(Self::TryWatch5Minutes),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EpisodeType {
    ///
    Unspecified = 0,
    ///
    Video = 1,
    ///
    Live = 2,
}
impl EpisodeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EpisodeType::Unspecified => "EPISODE_TYPE_UNSPECIFIED",
            EpisodeType::Video => "EPISODE_TYPE_VIDEO",
            EpisodeType::Live => "EPISODE_TYPE_LIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EPISODE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EPISODE_TYPE_VIDEO" => Some(Self::Video),
            "EPISODE_TYPE_LIVE" => Some(Self::Live),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LiveStatus {
    ///
    Unspecified = 0,
    ///
    Prepare = 1,
    ///
    Live = 2,
    ///
    PlaybackGenerating = 3,
}
impl LiveStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LiveStatus::Unspecified => "LIVE_STATUS_UNSPECIFIED",
            LiveStatus::Prepare => "LIVE_STATUS_PREPARE",
            LiveStatus::Live => "LIVE_STATUS_LIVE",
            LiveStatus::PlaybackGenerating => "LIVE_STATUS_PLAYBACK_GENERATING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIVE_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "LIVE_STATUS_PREPARE" => Some(Self::Prepare),
            "LIVE_STATUS_LIVE" => Some(Self::Live),
            "LIVE_STATUS_PLAYBACK_GENERATING" => Some(Self::PlaybackGenerating),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationAreaButtonType {
    ///
    Unspecified = 0,
    ///
    Favorite = 1,
    ///
    Consult = 2,
    ///
    Share = 3,
    ///
    Purchase = 4,
}
impl OperationAreaButtonType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationAreaButtonType::Unspecified => {
                "OPERATION_AREA_BUTTON_TYPE_UNSPECIFIED"
            }
            OperationAreaButtonType::Favorite => "OPERATION_AREA_BUTTON_TYPE_FAVORITE",
            OperationAreaButtonType::Consult => "OPERATION_AREA_BUTTON_TYPE_CONSULT",
            OperationAreaButtonType::Share => "OPERATION_AREA_BUTTON_TYPE_SHARE",
            OperationAreaButtonType::Purchase => "OPERATION_AREA_BUTTON_TYPE_PURCHASE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_AREA_BUTTON_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_AREA_BUTTON_TYPE_FAVORITE" => Some(Self::Favorite),
            "OPERATION_AREA_BUTTON_TYPE_CONSULT" => Some(Self::Consult),
            "OPERATION_AREA_BUTTON_TYPE_SHARE" => Some(Self::Share),
            "OPERATION_AREA_BUTTON_TYPE_PURCHASE" => Some(Self::Purchase),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeasonCouponStatus {
    ///
    Unspecified = 0,
    ///
    Received = 1,
    ///
    NotReceived = 2,
    ///
    Invalid = 3,
}
impl SeasonCouponStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SeasonCouponStatus::Unspecified => "SEASON_COUPON_STATUS_UNSPECIFIED",
            SeasonCouponStatus::Received => "SEASON_COUPON_STATUS_RECEIVED",
            SeasonCouponStatus::NotReceived => "SEASON_COUPON_STATUS_NOT_RECEIVED",
            SeasonCouponStatus::Invalid => "SEASON_COUPON_STATUS_INVALID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEASON_COUPON_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SEASON_COUPON_STATUS_RECEIVED" => Some(Self::Received),
            "SEASON_COUPON_STATUS_NOT_RECEIVED" => Some(Self::NotReceived),
            "SEASON_COUPON_STATUS_INVALID" => Some(Self::Invalid),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeasonCouponType {
    ///
    Unspecified = 0,
    ///
    Discount = 1,
    ///
    Decrease = 2,
}
impl SeasonCouponType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SeasonCouponType::Unspecified => "SEASON_COUPON_TYPE_UNSPECIFIED",
            SeasonCouponType::Discount => "SEASON_COUPON_TYPE_DISCOUNT",
            SeasonCouponType::Decrease => "SEASON_COUPON_TYPE_DECREASE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEASON_COUPON_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SEASON_COUPON_TYPE_DISCOUNT" => Some(Self::Discount),
            "SEASON_COUPON_TYPE_DECREASE" => Some(Self::Decrease),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeasonSectionType {
    ///
    Unspecified = 0,
    ///
    Default = 1,
    ///
    Custom = 2,
}
impl SeasonSectionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SeasonSectionType::Unspecified => "SEASON_SECTION_TYPE_UNSPECIFIED",
            SeasonSectionType::Default => "SEASON_SECTION_TYPE_DEFAULT",
            SeasonSectionType::Custom => "SEASON_SECTION_TYPE_CUSTOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEASON_SECTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SEASON_SECTION_TYPE_DEFAULT" => Some(Self::Default),
            "SEASON_SECTION_TYPE_CUSTOM" => Some(Self::Custom),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeasonStatus {
    ///
    Unspecified = 0,
    ///
    Offline = 1,
}
impl SeasonStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SeasonStatus::Unspecified => "SEASON_STATUS_UNSPECIFIED",
            SeasonStatus::Offline => "SEASON_STATUS_OFFLINE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEASON_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SEASON_STATUS_OFFLINE" => Some(Self::Offline),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeasonType {
    ///
    Unspecified = 0,
    ///
    Installment = 1,
    ///
    Free = 2,
    ///
    Subscription = 3,
}
impl SeasonType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SeasonType::Unspecified => "SEASON_TYPE_UNSPECIFIED",
            SeasonType::Installment => "SEASON_TYPE_INSTALLMENT",
            SeasonType::Free => "SEASON_TYPE_FREE",
            SeasonType::Subscription => "SEASON_TYPE_SUBSCRIPTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEASON_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SEASON_TYPE_INSTALLMENT" => Some(Self::Installment),
            "SEASON_TYPE_FREE" => Some(Self::Free),
            "SEASON_TYPE_SUBSCRIPTION" => Some(Self::Subscription),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoPlayWay {
    ///
    Unspecified = 0,
    ///
    Vod = 1,
    ///
    LivePlayback = 2,
}
impl VideoPlayWay {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoPlayWay::Unspecified => "VIDEO_PLAY_WAY_UNSPECIFIED",
            VideoPlayWay::Vod => "VIDEO_PLAY_WAY_VOD",
            VideoPlayWay::LivePlayback => "VIDEO_PLAY_WAY_LIVE_PLAYBACK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VIDEO_PLAY_WAY_UNSPECIFIED" => Some(Self::Unspecified),
            "VIDEO_PLAY_WAY_VOD" => Some(Self::Vod),
            "VIDEO_PLAY_WAY_LIVE_PLAYBACK" => Some(Self::LivePlayback),
            _ => None,
        }
    }
}
