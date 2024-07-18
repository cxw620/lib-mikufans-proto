// This file is @generated by prost-build.
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
    #[prost(int32, tag = "14")]
    pub is_multi_chat: i32,
    ///
    #[prost(int64, tag = "15")]
    pub withdraw_seqno: i64,
    ///
    #[prost(string, tag = "16")]
    pub notify_code: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "17")]
    pub msg_source: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct NotifyInfo {
    ///
    #[prost(int32, tag = "1")]
    pub msg_type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub talker_id: i64,
    ///
    #[prost(int32, tag = "3")]
    pub session_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyRsp {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub cmd: i64,
    ///
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    ///
    #[prost(enumeration = "PlType", tag = "4")]
    pub payload_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqServerNotify {
    ///
    #[prost(int64, tag = "1")]
    pub lastest_seqno: i64,
    ///
    #[prost(message, optional, tag = "2")]
    pub instant_msg: ::core::option::Option<Msg>,
    ///
    #[prost(message, optional, tag = "3")]
    pub notify_info: ::core::option::Option<NotifyInfo>,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CmdId {
    ///
    EnCmdIdInvalid = 0,
    ///
    EnCmdIdMsgNotify = 1,
    ///
    EnCmdIdKickOut = 2,
}
impl CmdId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CmdId::EnCmdIdInvalid => "EN_CMD_ID_INVALID",
            CmdId::EnCmdIdMsgNotify => "EN_CMD_ID_MSG_NOTIFY",
            CmdId::EnCmdIdKickOut => "EN_CMD_ID_KICK_OUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EN_CMD_ID_INVALID" => Some(Self::EnCmdIdInvalid),
            "EN_CMD_ID_MSG_NOTIFY" => Some(Self::EnCmdIdMsgNotify),
            "EN_CMD_ID_KICK_OUT" => Some(Self::EnCmdIdKickOut),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlType {
    ///
    EnPayloadNormal = 0,
    ///
    EnPayloadBase64 = 1,
}
impl PlType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlType::EnPayloadNormal => "EN_PAYLOAD_NORMAL",
            PlType::EnPayloadBase64 => "EN_PAYLOAD_BASE64",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EN_PAYLOAD_NORMAL" => Some(Self::EnPayloadNormal),
            "EN_PAYLOAD_BASE64" => Some(Self::EnPayloadBase64),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod notify_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct NotifyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NotifyClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> NotifyClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            NotifyClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        pub async fn watch_notify(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::NotifyRsp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.broadcast.message.im.Notify/WatchNotify",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.broadcast.message.im.Notify",
                        "WatchNotify",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod notify_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with NotifyServer.
    #[async_trait]
    pub trait Notify: Send + Sync + 'static {
        ///
        async fn watch_notify(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<tonic::Response<super::NotifyRsp>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct NotifyServer<T: Notify> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: Notify> NotifyServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NotifyServer<T>
    where
        T: Notify,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/bilibili.broadcast.message.im.Notify/WatchNotify" => {
                    #[allow(non_camel_case_types)]
                    struct WatchNotifySvc<T: Notify>(pub Arc<T>);
                    impl<T: Notify> tonic::server::UnaryService<()>
                    for WatchNotifySvc<T> {
                        type Response = super::NotifyRsp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Notify>::watch_notify(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = WatchNotifySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", tonic::Code::Unimplemented as i32)
                                .header(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                )
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Notify> Clone for NotifyServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Notify> tonic::server::NamedService for NotifyServer<T> {
        const NAME: &'static str = "bilibili.broadcast.message.im.Notify";
    }
}
