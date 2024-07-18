// This file is @generated by prost-build.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendReportInfoReq {
    ///
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub report_tag: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub report_reason: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub pic_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub play_stream: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendReportInfoResp {
    ///
    #[prost(enumeration = "ReportStatus", tag = "1")]
    pub report_status: i32,
    ///
    #[prost(string, tag = "2")]
    pub toast: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReportStatus {
    ///
    ReportException = 0,
    ///
    ReportSuccess = 1,
}
impl ReportStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReportStatus::ReportException => "REPORT_EXCEPTION",
            ReportStatus::ReportSuccess => "REPORT_SUCCESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REPORT_EXCEPTION" => Some(Self::ReportException),
            "REPORT_SUCCESS" => Some(Self::ReportSuccess),
            _ => None,
        }
    }
}
