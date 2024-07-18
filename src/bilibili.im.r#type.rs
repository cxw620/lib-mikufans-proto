// This file is @generated by prost-build.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiLogo {
    ///
    #[prost(string, tag = "1")]
    pub ai_mark: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub limit_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountInfo {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub pic_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiCardInfo {
    ///
    #[prost(int64, tag = "1")]
    pub ai_uid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub ai_status: i64,
    ///
    #[prost(message, optional, tag = "3")]
    pub u_info: ::core::option::Option<UInfo>,
    ///
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "6")]
    pub ai_logo: ::core::option::Option<AiLogo>,
    ///
    #[prost(int64, tag = "7")]
    pub uid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiEntry {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub subtitle: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiInfo {
    ///
    #[prost(message, optional, tag = "1")]
    pub card_info: ::core::option::Option<AiCardInfo>,
    ///
    #[prost(message, optional, tag = "2")]
    pub im_info: ::core::option::Option<ImInfo>,
    ///
    #[prost(message, optional, tag = "3")]
    pub ai_entry: ::core::option::Option<AiEntry>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestationDisplay {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub common_info: ::core::option::Option<CommonInfo>,
    ///
    #[prost(message, optional, tag = "3")]
    pub splice_info: ::core::option::Option<SpliceInfo>,
    ///
    #[prost(string, tag = "4")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Card {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub sex: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub sign: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub rank: i32,
    ///
    #[prost(int32, tag = "7")]
    pub level: i32,
    ///
    #[prost(int32, tag = "8")]
    pub silence: i32,
    ///
    #[prost(message, optional, tag = "9")]
    pub vip: ::core::option::Option<VipInfo>,
    ///
    #[prost(message, optional, tag = "10")]
    pub pendant: ::core::option::Option<PendantInfo>,
    ///
    #[prost(message, optional, tag = "11")]
    pub nameplate: ::core::option::Option<NameplateInfo>,
    ///
    #[prost(message, optional, tag = "12")]
    pub official: ::core::option::Option<OfficialInfo>,
    ///
    #[prost(int64, tag = "13")]
    pub birthday: i64,
    ///
    #[prost(int32, tag = "20")]
    pub is_fake_account: i32,
    ///
    #[prost(int32, tag = "21")]
    pub is_deleted: i32,
    ///
    #[prost(int32, tag = "22")]
    pub in_reg_audit: i32,
    ///
    #[prost(int32, tag = "23")]
    pub face_nft: i32,
    ///
    #[prost(int32, tag = "24")]
    pub face_nft_new: i32,
    ///
    #[prost(int32, tag = "25")]
    pub is_senior_member: i32,
    ///
    #[prost(string, tag = "26")]
    pub digital_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "27")]
    pub digital_type: i64,
    ///
    #[prost(message, optional, tag = "28")]
    pub attestation: ::core::option::Option<AttestationDisplay>,
    ///
    #[prost(message, optional, tag = "29")]
    pub expert_info: ::core::option::Option<ExpertInfo>,
    ///
    #[prost(message, optional, tag = "30")]
    pub honours: ::core::option::Option<UserHonourInfo>,
    ///
    #[prost(message, optional, tag = "31")]
    pub name_render: ::core::option::Option<
        super::super::account::service::v1::NameRender,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonInfo {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub prefix: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub prefix_title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpertInfo {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub state: i32,
    ///
    #[prost(int32, tag = "3")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendRelation {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(string, tag = "2")]
    pub user_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub vip_level: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GptMsgContent {
    ///
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<RichTextMsgContent>,
    ///
    #[prost(bool, tag = "2")]
    pub show_like: bool,
    ///
    #[prost(bool, tag = "3")]
    pub show_change: bool,
    ///
    #[prost(int64, tag = "4")]
    pub gpt_session_id: i64,
    ///
    #[prost(string, tag = "5")]
    pub gpt_bind_query: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub session_closed_line: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub voice_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub sub_type: i64,
    ///
    #[prost(int64, tag = "9")]
    pub voice_time: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GptRcmdQuestionBizInfo {
    ///
    #[prost(string, tag = "1")]
    pub question: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupRelation {
    ///
    #[prost(int64, tag = "1")]
    pub group_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub owner_uid: i64,
    ///
    #[prost(int32, tag = "3")]
    pub group_type: i32,
    ///
    #[prost(int32, tag = "4")]
    pub group_level: i32,
    ///
    #[prost(string, tag = "5")]
    pub group_cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub group_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub group_notice: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub status: i32,
    ///
    #[prost(int32, tag = "9")]
    pub member_role: i32,
    ///
    #[prost(string, tag = "10")]
    pub fans_medal_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "11")]
    pub room_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HighText {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub index: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HonourTag {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub web_link: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub r#type: i32,
    ///
    #[prost(string, repeated, tag = "5")]
    pub scene: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int32, tag = "6")]
    pub priority_level: i32,
    ///
    #[prost(string, tag = "7")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub year: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImInfo {
    ///
    #[prost(string, tag = "1")]
    pub background_url: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "2")]
    pub ai_prompt: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImgInfo {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub width: i32,
    ///
    #[prost(int32, tag = "3")]
    pub height: i32,
    ///
    #[prost(string, tag = "4")]
    pub image_type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyHitInfos {
    ///
    #[prost(string, tag = "1")]
    pub toast: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub rule_id: i32,
    ///
    #[prost(message, repeated, tag = "3")]
    pub high_text: ::prost::alloc::vec::Vec<HighText>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Medal {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub medal_id: i32,
    ///
    #[prost(int32, tag = "3")]
    pub level: i32,
    ///
    #[prost(string, tag = "4")]
    pub medal_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub score: i32,
    ///
    #[prost(int32, tag = "6")]
    pub intimacy: i32,
    ///
    #[prost(int32, tag = "7")]
    pub master_status: i32,
    ///
    #[prost(int32, tag = "8")]
    pub is_receive: i32,
    ///
    #[prost(int64, tag = "9")]
    pub medal_color_start: i64,
    ///
    #[prost(int64, tag = "10")]
    pub medal_color_end: i64,
    ///
    #[prost(int64, tag = "11")]
    pub medal_color_border: i64,
    ///
    #[prost(int64, tag = "12")]
    pub medal_color_name: i64,
    ///
    #[prost(int64, tag = "13")]
    pub medal_color_level: i64,
    ///
    #[prost(int64, tag = "14")]
    pub guard_level: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    ///
    #[prost(int64, tag = "1")]
    pub sender_uid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub receiver_type: i32,
    ///
    #[prost(int64, tag = "3")]
    pub receiver_id: i64,
    ///
    #[prost(int64, tag = "4")]
    pub cli_msg_id: i64,
    ///
    #[prost(int32, tag = "5")]
    pub msg_type: i32,
    ///
    #[prost(string, tag = "6")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub msg_seqno: i64,
    ///
    #[prost(int64, tag = "8")]
    pub timestamp: i64,
    ///
    #[prost(int64, repeated, tag = "9")]
    pub at_uids: ::prost::alloc::vec::Vec<i64>,
    ///
    #[prost(int64, repeated, tag = "10")]
    pub recver_ids: ::prost::alloc::vec::Vec<i64>,
    ///
    #[prost(int64, tag = "11")]
    pub msg_key: i64,
    ///
    #[prost(int32, tag = "12")]
    pub msg_status: i32,
    ///
    #[prost(bool, tag = "13")]
    pub sys_cancel: bool,
    ///
    #[prost(string, tag = "14")]
    pub notify_code: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "15")]
    pub msg_source: i32,
    ///
    #[prost(int32, tag = "16")]
    pub new_face_version: i32,
    ///
    #[prost(message, optional, tag = "17")]
    pub key_hit_infos: ::core::option::Option<KeyHitInfos>,
    ///
    #[prost(message, optional, tag = "18")]
    pub account_info: ::core::option::Option<AccountInfo>,
    ///
    #[prost(message, optional, tag = "19")]
    pub gpt_msg_content: ::core::option::Option<GptMsgContent>,
    ///
    #[prost(string, tag = "20")]
    pub canal_token: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameplateInfo {
    ///
    #[prost(int32, tag = "1")]
    pub nid: i32,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub image_small: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub level: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub condition: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialInfo {
    ///
    #[prost(int32, tag = "1")]
    pub role: i32,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendantInfo {
    ///
    #[prost(int32, tag = "1")]
    pub pid: i32,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub expire: i64,
    ///
    #[prost(string, tag = "5")]
    pub image_enhance: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub image_enhance_frame: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationLog {
    ///
    #[prost(int32, tag = "1")]
    pub log_type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub oplog_seqno: i64,
    ///
    #[prost(message, optional, tag = "3")]
    pub friend_relation: ::core::option::Option<FriendRelation>,
    ///
    #[prost(message, optional, tag = "4")]
    pub group_relation: ::core::option::Option<GroupRelation>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RichTextMsgContent {
    ///
    #[prost(message, repeated, tag = "1")]
    pub paragraphs: ::prost::alloc::vec::Vec<super::super::app::dynamic::v2::Paragraph>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionInfo {
    ///
    #[prost(int64, tag = "1")]
    pub talker_id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub session_type: i32,
    ///
    #[prost(int64, tag = "3")]
    pub at_seqno: i64,
    ///
    #[prost(int64, tag = "4")]
    pub top_ts: i64,
    ///
    #[prost(string, tag = "5")]
    pub group_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub group_cover: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub is_follow: i32,
    ///
    #[prost(int32, tag = "8")]
    pub is_dnd: i32,
    ///
    #[prost(int64, tag = "9")]
    pub ack_seqno: i64,
    ///
    #[prost(int64, tag = "10")]
    pub ack_ts: i64,
    ///
    #[prost(int64, tag = "11")]
    pub session_ts: i64,
    ///
    #[prost(int32, tag = "12")]
    pub unread_count: i32,
    ///
    #[prost(message, optional, tag = "13")]
    pub last_msg: ::core::option::Option<Msg>,
    ///
    #[prost(int32, tag = "14")]
    pub group_type: i32,
    ///
    #[prost(int32, tag = "15")]
    pub can_fold: i32,
    ///
    #[prost(int32, tag = "16")]
    pub status: i32,
    ///
    #[prost(int64, tag = "17")]
    pub max_seqno: i64,
    ///
    #[prost(int32, tag = "18")]
    pub new_push_msg: i32,
    ///
    #[prost(int32, tag = "19")]
    pub setting: i32,
    ///
    #[prost(int32, tag = "20")]
    pub is_guardian: i32,
    ///
    #[prost(int32, tag = "21")]
    pub is_intercept: i32,
    ///
    #[prost(int32, tag = "22")]
    pub is_trust: i32,
    ///
    #[prost(int32, tag = "23")]
    pub system_msg_type: i32,
    ///
    #[prost(message, optional, tag = "24")]
    pub account_info: ::core::option::Option<AccountInfo>,
    ///
    #[prost(int32, tag = "25")]
    pub live_status: i32,
    ///
    #[prost(int32, tag = "26")]
    pub biz_msg_unread_count: i32,
    ///
    #[prost(message, optional, tag = "27")]
    pub user_label: ::core::option::Option<UserLabel>,
    ///
    #[prost(int32, tag = "28")]
    pub is_huahuo: i32,
    ///
    #[prost(message, optional, tag = "29")]
    pub u_info: ::core::option::Option<UInfo>,
    ///
    #[prost(int32, tag = "30")]
    pub stranger: i32,
    ///
    #[prost(message, optional, tag = "31")]
    pub ai_info: ::core::option::Option<AiInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpliceInfo {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInfo {
    ///
    #[prost(message, optional, tag = "1")]
    pub ava: ::core::option::Option<
        super::super::dagw::component::avatar::v1::AvatarItem,
    >,
    ///
    #[prost(message, optional, tag = "2")]
    pub card: ::core::option::Option<Card>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserHonourInfo {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(message, optional, tag = "2")]
    pub colour: ::core::option::Option<UserHonourStyle>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub tags: ::prost::alloc::vec::Vec<HonourTag>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserHonourStyle {
    ///
    #[prost(string, tag = "1")]
    pub dark: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub normal: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLabel {
    ///
    #[prost(int32, tag = "1")]
    pub label_type: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub medal: ::core::option::Option<Medal>,
    ///
    #[prost(int32, tag = "3")]
    pub guardian_relation: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipInfo {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "2")]
    pub status: i32,
    ///
    #[prost(int64, tag = "3")]
    pub due_date: i64,
    ///
    #[prost(int32, tag = "4")]
    pub vip_pay_type: i32,
    ///
    #[prost(int32, tag = "5")]
    pub theme_type: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub label: ::core::option::Option<VipLabel>,
    ///
    #[prost(int32, tag = "7")]
    pub avatar_subscript: i32,
    ///
    #[prost(string, tag = "8")]
    pub nickname_color: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "9")]
    pub role: i64,
    ///
    #[prost(string, tag = "10")]
    pub avatar_subscript_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "11")]
    pub tv_vip_status: i32,
    ///
    #[prost(int32, tag = "12")]
    pub tv_vip_pay_type: i32,
    ///
    #[prost(int64, tag = "13")]
    pub tv_due_date: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipLabel {
    ///
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub label_theme: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub bg_style: i32,
    ///
    #[prost(string, tag = "7")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub border_color: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "9")]
    pub use_img_label: bool,
    ///
    #[prost(string, tag = "10")]
    pub img_label_uri_hans: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub img_label_uri_hant: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub img_label_uri_hans_static: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub img_label_uri_hant_static: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Exp {
    ///
    Invalid = 0,
    ///
    NewAva = 1,
}
impl Exp {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Exp::Invalid => "Invalid",
            Exp::NewAva => "New_Ava",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Invalid" => Some(Self::Invalid),
            "New_Ava" => Some(Self::NewAva),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    ///
    EnInvalidMsgType = 0,
    ///
    EnMsgTypeText = 1,
    ///
    EnMsgTypePic = 2,
    ///
    EnMsgTypeAudio = 3,
    ///
    EnMsgTypeShare = 4,
    ///
    EnMsgTypeDrawBack = 5,
    ///
    EnMsgTypeCustomFace = 6,
    ///
    EnMsgTypeShareV2 = 7,
    ///
    EnMsgTypeSysCancel = 8,
    ///
    EnMsgTypeMiniProgram = 9,
    ///
    EnMsgTypeNotifyMsg = 10,
    ///
    EnMsgTypeVideoCard = 11,
    ///
    EnMsgTypeArticleCard = 12,
    ///
    EnMsgTypePictureCard = 13,
    ///
    EnMsgTypeCommonShareCard = 14,
    ///
    EnMsgTypeTextShare = 15,
    ///
    EnMsgTypeTipMessage = 18,
    ///
    EnMsgTypeGptMessage = 19,
    ///
    EnMsgTypeBizMsgType = 50,
    ///
    EnMsgTypeModifyMsgType = 51,
    ///
    EnMsgTypeGroupMemberChanged = 101,
    ///
    EnMsgTypeGroupStatusChanged = 102,
    ///
    EnMsgTypeGroupDynamicChanged = 103,
    ///
    EnMsgTypeGroupListChanged = 104,
    ///
    EmMsgTypeFriendListChanged = 105,
    ///
    EnMsgTypeGroupDetailChanged = 106,
    ///
    EnMsgTypeGroupMemberRoleChanged = 107,
    ///
    EnMsgTypeNoticeWatchList = 108,
    ///
    EnMsgTypeNotifyNewReplyRecieved = 109,
    ///
    EnMsgTypeNotifyNewAtRecieved = 110,
    ///
    EnMsgTypeNotifyNewPraiseRecieved = 111,
    ///
    EnMsgTypeNotifyNewUpRecieved = 112,
    ///
    EnMsgTypeNotifyNewReplyRecievedV2 = 113,
    ///
    EnMsgTypeNotifyNewAtRecievedV2 = 114,
    ///
    EnMsgTypeNotifyNewPraiseRecievedV2 = 115,
    ///
    EnMsgTypeGroupDetailChangedMulti = 116,
    ///
    EnMsgTypeGroupMemberRoleChangedMulti = 117,
    ///
    EnMsgTypeNotifyAntiDisturb = 118,
    ///
    EnMsgTypeSysGroupDissolved = 201,
    ///
    EnMsgTypeSysGroupJoined = 202,
    ///
    EnMsgTypeSysGroupMemberExited = 203,
    ///
    EnMsgTypeSysGroupAdminFired = 204,
    ///
    EnMsgTypeSysGroupMemberKicked = 205,
    ///
    EnMsgTypeSysGroupAdminKickOff = 206,
    ///
    EnMsgTypeSysGroupAdminDuty = 207,
    ///
    EnMsgTypeSysGroupAutoCreated = 208,
    ///
    EnMsgTypeSysFriendApply = 210,
    ///
    EnMsgTypeSysFriendApplyAck = 211,
    ///
    EnMsgTypeSysGroupApplyForJoining = 212,
    ///
    EnMsgTypeSysGroupAdminAcceptedUserApply = 213,
    ///
    EnMsgTypeChatMemberJoined = 301,
    ///
    EnMsgTypeChatMemberExited = 302,
    ///
    EnMsgTypeChatGroupFreezed = 303,
    ///
    EnMsgTypeChatGroupDissolved = 304,
    ///
    EnMsgTypeChatGroupCreated = 305,
    ///
    EnMsgTypeChatPopupSession = 306,
    ///
    EnMsgTypeCustomRankUpdate = 400,
    ///
    EnMsgTypeCustomMsgNotice = 401,
}
impl MsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MsgType::EnInvalidMsgType => "EN_INVALID_MSG_TYPE",
            MsgType::EnMsgTypeText => "EN_MSG_TYPE_TEXT",
            MsgType::EnMsgTypePic => "EN_MSG_TYPE_PIC",
            MsgType::EnMsgTypeAudio => "EN_MSG_TYPE_AUDIO",
            MsgType::EnMsgTypeShare => "EN_MSG_TYPE_SHARE",
            MsgType::EnMsgTypeDrawBack => "EN_MSG_TYPE_DRAW_BACK",
            MsgType::EnMsgTypeCustomFace => "EN_MSG_TYPE_CUSTOM_FACE",
            MsgType::EnMsgTypeShareV2 => "EN_MSG_TYPE_SHARE_V2",
            MsgType::EnMsgTypeSysCancel => "EN_MSG_TYPE_SYS_CANCEL",
            MsgType::EnMsgTypeMiniProgram => "EN_MSG_TYPE_MINI_PROGRAM",
            MsgType::EnMsgTypeNotifyMsg => "EN_MSG_TYPE_NOTIFY_MSG",
            MsgType::EnMsgTypeVideoCard => "EN_MSG_TYPE_VIDEO_CARD",
            MsgType::EnMsgTypeArticleCard => "EN_MSG_TYPE_ARTICLE_CARD",
            MsgType::EnMsgTypePictureCard => "EN_MSG_TYPE_PICTURE_CARD",
            MsgType::EnMsgTypeCommonShareCard => "EN_MSG_TYPE_COMMON_SHARE_CARD",
            MsgType::EnMsgTypeTextShare => "EN_MSG_TYPE_TEXT_SHARE",
            MsgType::EnMsgTypeTipMessage => "EN_MSG_TYPE_TIP_MESSAGE",
            MsgType::EnMsgTypeGptMessage => "EN_MSG_TYPE_GPT_MESSAGE",
            MsgType::EnMsgTypeBizMsgType => "EN_MSG_TYPE_BIZ_MSG_TYPE",
            MsgType::EnMsgTypeModifyMsgType => "EN_MSG_TYPE_MODIFY_MSG_TYPE",
            MsgType::EnMsgTypeGroupMemberChanged => "EN_MSG_TYPE_GROUP_MEMBER_CHANGED",
            MsgType::EnMsgTypeGroupStatusChanged => "EN_MSG_TYPE_GROUP_STATUS_CHANGED",
            MsgType::EnMsgTypeGroupDynamicChanged => "EN_MSG_TYPE_GROUP_DYNAMIC_CHANGED",
            MsgType::EnMsgTypeGroupListChanged => "EN_MSG_TYPE_GROUP_LIST_CHANGED",
            MsgType::EmMsgTypeFriendListChanged => "EM_MSG_TYPE_FRIEND_LIST_CHANGED",
            MsgType::EnMsgTypeGroupDetailChanged => "EN_MSG_TYPE_GROUP_DETAIL_CHANGED",
            MsgType::EnMsgTypeGroupMemberRoleChanged => {
                "EN_MSG_TYPE_GROUP_MEMBER_ROLE_CHANGED"
            }
            MsgType::EnMsgTypeNoticeWatchList => "EN_MSG_TYPE_NOTICE_WATCH_LIST",
            MsgType::EnMsgTypeNotifyNewReplyRecieved => {
                "EN_MSG_TYPE_NOTIFY_NEW_REPLY_RECIEVED"
            }
            MsgType::EnMsgTypeNotifyNewAtRecieved => "EN_MSG_TYPE_NOTIFY_NEW_AT_RECIEVED",
            MsgType::EnMsgTypeNotifyNewPraiseRecieved => {
                "EN_MSG_TYPE_NOTIFY_NEW_PRAISE_RECIEVED"
            }
            MsgType::EnMsgTypeNotifyNewUpRecieved => "EN_MSG_TYPE_NOTIFY_NEW_UP_RECIEVED",
            MsgType::EnMsgTypeNotifyNewReplyRecievedV2 => {
                "EN_MSG_TYPE_NOTIFY_NEW_REPLY_RECIEVED_V2"
            }
            MsgType::EnMsgTypeNotifyNewAtRecievedV2 => {
                "EN_MSG_TYPE_NOTIFY_NEW_AT_RECIEVED_V2"
            }
            MsgType::EnMsgTypeNotifyNewPraiseRecievedV2 => {
                "EN_MSG_TYPE_NOTIFY_NEW_PRAISE_RECIEVED_V2"
            }
            MsgType::EnMsgTypeGroupDetailChangedMulti => {
                "EN_MSG_TYPE_GROUP_DETAIL_CHANGED_MULTI"
            }
            MsgType::EnMsgTypeGroupMemberRoleChangedMulti => {
                "EN_MSG_TYPE_GROUP_MEMBER_ROLE_CHANGED_MULTI"
            }
            MsgType::EnMsgTypeNotifyAntiDisturb => "EN_MSG_TYPE_NOTIFY_ANTI_DISTURB",
            MsgType::EnMsgTypeSysGroupDissolved => "EN_MSG_TYPE_SYS_GROUP_DISSOLVED",
            MsgType::EnMsgTypeSysGroupJoined => "EN_MSG_TYPE_SYS_GROUP_JOINED",
            MsgType::EnMsgTypeSysGroupMemberExited => {
                "EN_MSG_TYPE_SYS_GROUP_MEMBER_EXITED"
            }
            MsgType::EnMsgTypeSysGroupAdminFired => "EN_MSG_TYPE_SYS_GROUP_ADMIN_FIRED",
            MsgType::EnMsgTypeSysGroupMemberKicked => {
                "EN_MSG_TYPE_SYS_GROUP_MEMBER_KICKED"
            }
            MsgType::EnMsgTypeSysGroupAdminKickOff => {
                "EN_MSG_TYPE_SYS_GROUP_ADMIN_KICK_OFF"
            }
            MsgType::EnMsgTypeSysGroupAdminDuty => "EN_MSG_TYPE_SYS_GROUP_ADMIN_DUTY",
            MsgType::EnMsgTypeSysGroupAutoCreated => "EN_MSG_TYPE_SYS_GROUP_AUTO_CREATED",
            MsgType::EnMsgTypeSysFriendApply => "EN_MSG_TYPE_SYS_FRIEND_APPLY",
            MsgType::EnMsgTypeSysFriendApplyAck => "EN_MSG_TYPE_SYS_FRIEND_APPLY_ACK",
            MsgType::EnMsgTypeSysGroupApplyForJoining => {
                "EN_MSG_TYPE_SYS_GROUP_APPLY_FOR_JOINING"
            }
            MsgType::EnMsgTypeSysGroupAdminAcceptedUserApply => {
                "EN_MSG_TYPE_SYS_GROUP_ADMIN_ACCEPTED_USER_APPLY"
            }
            MsgType::EnMsgTypeChatMemberJoined => "EN_MSG_TYPE_CHAT_MEMBER_JOINED",
            MsgType::EnMsgTypeChatMemberExited => "EN_MSG_TYPE_CHAT_MEMBER_EXITED",
            MsgType::EnMsgTypeChatGroupFreezed => "EN_MSG_TYPE_CHAT_GROUP_FREEZED",
            MsgType::EnMsgTypeChatGroupDissolved => "EN_MSG_TYPE_CHAT_GROUP_DISSOLVED",
            MsgType::EnMsgTypeChatGroupCreated => "EN_MSG_TYPE_CHAT_GROUP_CREATED",
            MsgType::EnMsgTypeChatPopupSession => "EN_MSG_TYPE_CHAT_POPUP_SESSION",
            MsgType::EnMsgTypeCustomRankUpdate => "EN_MSG_TYPE_CUSTOM_RANK_UPDATE",
            MsgType::EnMsgTypeCustomMsgNotice => "EN_MSG_TYPE_CUSTOM_MSG_NOTICE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EN_INVALID_MSG_TYPE" => Some(Self::EnInvalidMsgType),
            "EN_MSG_TYPE_TEXT" => Some(Self::EnMsgTypeText),
            "EN_MSG_TYPE_PIC" => Some(Self::EnMsgTypePic),
            "EN_MSG_TYPE_AUDIO" => Some(Self::EnMsgTypeAudio),
            "EN_MSG_TYPE_SHARE" => Some(Self::EnMsgTypeShare),
            "EN_MSG_TYPE_DRAW_BACK" => Some(Self::EnMsgTypeDrawBack),
            "EN_MSG_TYPE_CUSTOM_FACE" => Some(Self::EnMsgTypeCustomFace),
            "EN_MSG_TYPE_SHARE_V2" => Some(Self::EnMsgTypeShareV2),
            "EN_MSG_TYPE_SYS_CANCEL" => Some(Self::EnMsgTypeSysCancel),
            "EN_MSG_TYPE_MINI_PROGRAM" => Some(Self::EnMsgTypeMiniProgram),
            "EN_MSG_TYPE_NOTIFY_MSG" => Some(Self::EnMsgTypeNotifyMsg),
            "EN_MSG_TYPE_VIDEO_CARD" => Some(Self::EnMsgTypeVideoCard),
            "EN_MSG_TYPE_ARTICLE_CARD" => Some(Self::EnMsgTypeArticleCard),
            "EN_MSG_TYPE_PICTURE_CARD" => Some(Self::EnMsgTypePictureCard),
            "EN_MSG_TYPE_COMMON_SHARE_CARD" => Some(Self::EnMsgTypeCommonShareCard),
            "EN_MSG_TYPE_TEXT_SHARE" => Some(Self::EnMsgTypeTextShare),
            "EN_MSG_TYPE_TIP_MESSAGE" => Some(Self::EnMsgTypeTipMessage),
            "EN_MSG_TYPE_GPT_MESSAGE" => Some(Self::EnMsgTypeGptMessage),
            "EN_MSG_TYPE_BIZ_MSG_TYPE" => Some(Self::EnMsgTypeBizMsgType),
            "EN_MSG_TYPE_MODIFY_MSG_TYPE" => Some(Self::EnMsgTypeModifyMsgType),
            "EN_MSG_TYPE_GROUP_MEMBER_CHANGED" => Some(Self::EnMsgTypeGroupMemberChanged),
            "EN_MSG_TYPE_GROUP_STATUS_CHANGED" => Some(Self::EnMsgTypeGroupStatusChanged),
            "EN_MSG_TYPE_GROUP_DYNAMIC_CHANGED" => {
                Some(Self::EnMsgTypeGroupDynamicChanged)
            }
            "EN_MSG_TYPE_GROUP_LIST_CHANGED" => Some(Self::EnMsgTypeGroupListChanged),
            "EM_MSG_TYPE_FRIEND_LIST_CHANGED" => Some(Self::EmMsgTypeFriendListChanged),
            "EN_MSG_TYPE_GROUP_DETAIL_CHANGED" => Some(Self::EnMsgTypeGroupDetailChanged),
            "EN_MSG_TYPE_GROUP_MEMBER_ROLE_CHANGED" => {
                Some(Self::EnMsgTypeGroupMemberRoleChanged)
            }
            "EN_MSG_TYPE_NOTICE_WATCH_LIST" => Some(Self::EnMsgTypeNoticeWatchList),
            "EN_MSG_TYPE_NOTIFY_NEW_REPLY_RECIEVED" => {
                Some(Self::EnMsgTypeNotifyNewReplyRecieved)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_AT_RECIEVED" => {
                Some(Self::EnMsgTypeNotifyNewAtRecieved)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_PRAISE_RECIEVED" => {
                Some(Self::EnMsgTypeNotifyNewPraiseRecieved)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_UP_RECIEVED" => {
                Some(Self::EnMsgTypeNotifyNewUpRecieved)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_REPLY_RECIEVED_V2" => {
                Some(Self::EnMsgTypeNotifyNewReplyRecievedV2)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_AT_RECIEVED_V2" => {
                Some(Self::EnMsgTypeNotifyNewAtRecievedV2)
            }
            "EN_MSG_TYPE_NOTIFY_NEW_PRAISE_RECIEVED_V2" => {
                Some(Self::EnMsgTypeNotifyNewPraiseRecievedV2)
            }
            "EN_MSG_TYPE_GROUP_DETAIL_CHANGED_MULTI" => {
                Some(Self::EnMsgTypeGroupDetailChangedMulti)
            }
            "EN_MSG_TYPE_GROUP_MEMBER_ROLE_CHANGED_MULTI" => {
                Some(Self::EnMsgTypeGroupMemberRoleChangedMulti)
            }
            "EN_MSG_TYPE_NOTIFY_ANTI_DISTURB" => Some(Self::EnMsgTypeNotifyAntiDisturb),
            "EN_MSG_TYPE_SYS_GROUP_DISSOLVED" => Some(Self::EnMsgTypeSysGroupDissolved),
            "EN_MSG_TYPE_SYS_GROUP_JOINED" => Some(Self::EnMsgTypeSysGroupJoined),
            "EN_MSG_TYPE_SYS_GROUP_MEMBER_EXITED" => {
                Some(Self::EnMsgTypeSysGroupMemberExited)
            }
            "EN_MSG_TYPE_SYS_GROUP_ADMIN_FIRED" => {
                Some(Self::EnMsgTypeSysGroupAdminFired)
            }
            "EN_MSG_TYPE_SYS_GROUP_MEMBER_KICKED" => {
                Some(Self::EnMsgTypeSysGroupMemberKicked)
            }
            "EN_MSG_TYPE_SYS_GROUP_ADMIN_KICK_OFF" => {
                Some(Self::EnMsgTypeSysGroupAdminKickOff)
            }
            "EN_MSG_TYPE_SYS_GROUP_ADMIN_DUTY" => Some(Self::EnMsgTypeSysGroupAdminDuty),
            "EN_MSG_TYPE_SYS_GROUP_AUTO_CREATED" => {
                Some(Self::EnMsgTypeSysGroupAutoCreated)
            }
            "EN_MSG_TYPE_SYS_FRIEND_APPLY" => Some(Self::EnMsgTypeSysFriendApply),
            "EN_MSG_TYPE_SYS_FRIEND_APPLY_ACK" => Some(Self::EnMsgTypeSysFriendApplyAck),
            "EN_MSG_TYPE_SYS_GROUP_APPLY_FOR_JOINING" => {
                Some(Self::EnMsgTypeSysGroupApplyForJoining)
            }
            "EN_MSG_TYPE_SYS_GROUP_ADMIN_ACCEPTED_USER_APPLY" => {
                Some(Self::EnMsgTypeSysGroupAdminAcceptedUserApply)
            }
            "EN_MSG_TYPE_CHAT_MEMBER_JOINED" => Some(Self::EnMsgTypeChatMemberJoined),
            "EN_MSG_TYPE_CHAT_MEMBER_EXITED" => Some(Self::EnMsgTypeChatMemberExited),
            "EN_MSG_TYPE_CHAT_GROUP_FREEZED" => Some(Self::EnMsgTypeChatGroupFreezed),
            "EN_MSG_TYPE_CHAT_GROUP_DISSOLVED" => Some(Self::EnMsgTypeChatGroupDissolved),
            "EN_MSG_TYPE_CHAT_GROUP_CREATED" => Some(Self::EnMsgTypeChatGroupCreated),
            "EN_MSG_TYPE_CHAT_POPUP_SESSION" => Some(Self::EnMsgTypeChatPopupSession),
            "EN_MSG_TYPE_CUSTOM_RANK_UPDATE" => Some(Self::EnMsgTypeCustomRankUpdate),
            "EN_MSG_TYPE_CUSTOM_MSG_NOTICE" => Some(Self::EnMsgTypeCustomMsgNotice),
            _ => None,
        }
    }
}
