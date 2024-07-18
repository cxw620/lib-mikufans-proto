// This file is @generated by prost-build.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiExtra {
    ///
    #[prost(string, tag = "1")]
    pub track_id: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dislike {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub reasons: ::prost::alloc::vec::Vec<DislikeReason>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DislikeReason {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(int32, tag = "3")]
    pub rid: i32,
    ///
    #[prost(int64, tag = "4")]
    pub tag_id: i64,
    ///
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElecRank {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<ElecRankItem>,
    ///
    #[prost(int64, tag = "2")]
    pub count: i64,
    ///
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElecRankItem {
    ///
    #[prost(string, tag = "1")]
    pub avatar: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub nickname: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Experiment {
    ///
    #[prost(message, optional, tag = "1")]
    pub share_guide: ::core::option::Option<ShareGuide>,
    ///
    #[prost(message, optional, tag = "2")]
    pub follow_guide: ::core::option::Option<FollowGuide>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowGuide {
    ///
    #[prost(bool, tag = "1")]
    pub is_view: bool,
    ///
    #[prost(string, tag = "2")]
    pub view_text: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "3")]
    pub is_thumb: bool,
    ///
    #[prost(string, tag = "4")]
    pub thumb_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Premiere {
    ///
    #[prost(enumeration = "PremiereState", tag = "1")]
    pub premiere_state: i32,
    ///
    #[prost(int64, tag = "2")]
    pub start_time: i64,
    ///
    #[prost(int64, tag = "3")]
    pub service_time: i64,
    ///
    #[prost(int64, tag = "4")]
    pub room_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PremiereReserve {
    ///
    #[prost(int64, tag = "1")]
    pub reserve_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub count: i64,
    ///
    #[prost(bool, tag = "3")]
    pub is_follow: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PremiereResource {
    ///
    #[prost(message, optional, tag = "1")]
    pub premiere: ::core::option::Option<Premiere>,
    ///
    #[prost(message, optional, tag = "2")]
    pub reserve: ::core::option::Option<PremiereReserve>,
    ///
    #[prost(message, optional, tag = "3")]
    pub text: ::core::option::Option<PremiereText>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PremiereText {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub online_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub online_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub online_icon_dark: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub intro_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub intro_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub guidance_pulldown: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub guidance_entry: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub intro_icon_night: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareGuide {
    ///
    #[prost(bool, tag = "1")]
    pub hit_a: bool,
    ///
    #[prost(int32, tag = "2")]
    pub duration: i32,
    ///
    #[prost(int32, tag = "3")]
    pub count_a: i32,
    ///
    #[prost(bool, tag = "4")]
    pub hit_b: bool,
    ///
    #[prost(int32, tag = "5")]
    pub count_b: i32,
    ///
    #[prost(string, tag = "6")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub emphasized_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct UgcSeasonConf {
    ///
    #[prost(bool, tag = "1")]
    pub season_unfold: bool,
    ///
    #[prost(int64, tag = "2")]
    pub fold_time: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewUgcAny {
    ///
    #[prost(message, optional, tag = "1")]
    pub premiere: ::core::option::Option<PremiereResource>,
    ///
    #[prost(message, optional, tag = "2")]
    pub dislike: ::core::option::Option<Dislike>,
    ///
    #[prost(string, tag = "3")]
    pub short_link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub share_subtitle: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "5")]
    pub pages: ::prost::alloc::vec::Vec<super::common::Page>,
    ///
    #[prost(message, optional, tag = "6")]
    pub elec_rank: ::core::option::Option<ElecRank>,
    ///
    #[prost(message, optional, tag = "7")]
    pub ugc_season_conf: ::core::option::Option<UgcSeasonConf>,
    ///
    #[prost(message, optional, tag = "8")]
    pub ai_extra: ::core::option::Option<AiExtra>,
    ///
    #[prost(message, optional, tag = "9")]
    pub experiment: ::core::option::Option<Experiment>,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PremiereState {
    ///
    PremiereNone = 0,
    ///
    PremiereBefore = 1,
    ///
    PremiereIn = 2,
    ///
    PremiereAfter = 3,
}
impl PremiereState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PremiereState::PremiereNone => "premiere_none",
            PremiereState::PremiereBefore => "premiere_before",
            PremiereState::PremiereIn => "premiere_in",
            PremiereState::PremiereAfter => "premiere_after",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "premiere_none" => Some(Self::PremiereNone),
            "premiere_before" => Some(Self::PremiereBefore),
            "premiere_in" => Some(Self::PremiereIn),
            "premiere_after" => Some(Self::PremiereAfter),
            _ => None,
        }
    }
}
