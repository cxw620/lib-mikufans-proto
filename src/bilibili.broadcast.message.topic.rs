// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailEventMessage {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(message, repeated, tag = "2")]
    pub pub_data: ::prost::alloc::vec::Vec<PubEvent>,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PubEvent {
    ///
    #[prost(int64, tag = "1")]
    pub dynamic_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
}
