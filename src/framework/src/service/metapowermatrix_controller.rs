#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPayOrdersRequest {
    #[prost(string, tag = "1")]
    pub order: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub buyer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPayOrdersResponse {
    #[prost(string, tag = "1")]
    pub order: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub paid: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditCardPayRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub item: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub amount: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditCardPayResponse {
    #[prost(string, tag = "1")]
    pub pay_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Knowledge {
    #[prost(string, tag = "1")]
    pub sig: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub summary: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedKnowledgesResponse {
    #[prost(message, repeated, tag = "1")]
    pub books: ::prost::alloc::vec::Vec<Knowledge>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotAi {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub talks: i32,
    #[prost(string, tag = "4")]
    pub pros: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotAiResponse {
    #[prost(message, repeated, tag = "1")]
    pub sheniu: ::prost::alloc::vec::Vec<HotAi>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotTopicResponse {
    #[prost(string, repeated, tag = "1")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(float, tag = "2")]
    pub amount: f32,
    #[prost(string, tag = "3")]
    pub sub_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DonationRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(float, tag = "2")]
    pub amount: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResonse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NearbyRequest {
    #[prost(int64, tag = "1")]
    pub sn: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NearbyRespnse {
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrayRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakeProfessionRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub knowledge: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub file_sig: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakePlanRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub refresh: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakePlanResponse {
    #[prost(string, tag = "1")]
    pub plan_file: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaceRequest {
    #[prost(string, tag = "1")]
    pub place_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaceResonse {
    #[prost(int64, repeated, tag = "1")]
    pub sn: ::prost::alloc::vec::Vec<i64>,
}
/// Generated client implementations.
pub mod meta_power_matrix_controller_svc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MetaPowerMatrixControllerSvcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MetaPowerMatrixControllerSvcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MetaPowerMatrixControllerSvcClient<T>
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
        ) -> MetaPowerMatrixControllerSvcClient<InterceptedService<T, F>>
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
            MetaPowerMatrixControllerSvcClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        pub async fn request_create_pato(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateResonse>, tonic::Status> {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestCreatePato",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "RequestCreatePato",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_pato_login(
            &mut self,
            request: impl tonic::IntoRequest<super::LoginRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestPatoLogin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "RequestPatoLogin",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_pray(
            &mut self,
            request: impl tonic::IntoRequest<super::PrayRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestPray",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "RequestPray",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_make_profession(
            &mut self,
            request: impl tonic::IntoRequest<super::MakeProfessionRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestMakeProfession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "RequestMakeProfession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_make_town_map(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyResponse>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestMakeTownMap",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "RequestMakeTownMap",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn make_plan_for_pato(
            &mut self,
            request: impl tonic::IntoRequest<super::MakePlanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MakePlanResponse>,
            tonic::Status,
        > {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/MakePlanForPato",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "MakePlanForPato",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_pato_nearby(
            &mut self,
            request: impl tonic::IntoRequest<super::NearbyRequest>,
        ) -> std::result::Result<tonic::Response<super::NearbyRespnse>, tonic::Status> {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestPatoNearby",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "RequestPatoNearby",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_place_in_map(
            &mut self,
            request: impl tonic::IntoRequest<super::PlaceRequest>,
        ) -> std::result::Result<tonic::Response<super::PlaceResonse>, tonic::Status> {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestPlaceInMap",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "RequestPlaceInMap",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn accept_donation(
            &mut self,
            request: impl tonic::IntoRequest<super::DonationRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/AcceptDonation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "AcceptDonation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn accept_stake(
            &mut self,
            request: impl tonic::IntoRequest<super::DonationRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/AcceptStake",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "AcceptStake",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn accept_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscriptionRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/AcceptSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "AcceptSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_hot_ai(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyResponse>,
        ) -> std::result::Result<tonic::Response<super::HotAiResponse>, tonic::Status> {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestHotAI",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "RequestHotAI",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_shared_knowledges(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyResponse>,
        ) -> std::result::Result<
            tonic::Response<super::SharedKnowledgesResponse>,
            tonic::Status,
        > {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestSharedKnowledges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "RequestSharedKnowledges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_hot_topics(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyResponse>,
        ) -> std::result::Result<
            tonic::Response<super::HotTopicResponse>,
            tonic::Status,
        > {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestHotTopics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "RequestHotTopics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_pay_credit_card(
            &mut self,
            request: impl tonic::IntoRequest<super::CreditCardPayRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreditCardPayResponse>,
            tonic::Status,
        > {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestPayCreditCard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "RequestPayCreditCard",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn check_pay_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckPayOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckPayOrdersResponse>,
            tonic::Status,
        > {
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/CheckPayOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_controller.MetaPowerMatrixControllerSvc",
                        "CheckPayOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod meta_power_matrix_controller_svc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MetaPowerMatrixControllerSvcServer.
    #[async_trait]
    pub trait MetaPowerMatrixControllerSvc: Send + Sync + 'static {
        async fn request_create_pato(
            &self,
            request: tonic::Request<super::CreateRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateResonse>, tonic::Status>;
        async fn request_pato_login(
            &self,
            request: tonic::Request<super::LoginRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status>;
        async fn request_pray(
            &self,
            request: tonic::Request<super::PrayRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status>;
        async fn request_make_profession(
            &self,
            request: tonic::Request<super::MakeProfessionRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status>;
        async fn request_make_town_map(
            &self,
            request: tonic::Request<super::EmptyResponse>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status>;
        async fn make_plan_for_pato(
            &self,
            request: tonic::Request<super::MakePlanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MakePlanResponse>,
            tonic::Status,
        >;
        async fn request_pato_nearby(
            &self,
            request: tonic::Request<super::NearbyRequest>,
        ) -> std::result::Result<tonic::Response<super::NearbyRespnse>, tonic::Status>;
        async fn request_place_in_map(
            &self,
            request: tonic::Request<super::PlaceRequest>,
        ) -> std::result::Result<tonic::Response<super::PlaceResonse>, tonic::Status>;
        async fn accept_donation(
            &self,
            request: tonic::Request<super::DonationRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status>;
        async fn accept_stake(
            &self,
            request: tonic::Request<super::DonationRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status>;
        async fn accept_subscription(
            &self,
            request: tonic::Request<super::SubscriptionRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyResponse>, tonic::Status>;
        async fn request_hot_ai(
            &self,
            request: tonic::Request<super::EmptyResponse>,
        ) -> std::result::Result<tonic::Response<super::HotAiResponse>, tonic::Status>;
        async fn request_shared_knowledges(
            &self,
            request: tonic::Request<super::EmptyResponse>,
        ) -> std::result::Result<
            tonic::Response<super::SharedKnowledgesResponse>,
            tonic::Status,
        >;
        async fn request_hot_topics(
            &self,
            request: tonic::Request<super::EmptyResponse>,
        ) -> std::result::Result<
            tonic::Response<super::HotTopicResponse>,
            tonic::Status,
        >;
        async fn request_pay_credit_card(
            &self,
            request: tonic::Request<super::CreditCardPayRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreditCardPayResponse>,
            tonic::Status,
        >;
        async fn check_pay_orders(
            &self,
            request: tonic::Request<super::CheckPayOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckPayOrdersResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MetaPowerMatrixControllerSvcServer<T: MetaPowerMatrixControllerSvc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MetaPowerMatrixControllerSvc> MetaPowerMatrixControllerSvcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for MetaPowerMatrixControllerSvcServer<T>
    where
        T: MetaPowerMatrixControllerSvc,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestCreatePato" => {
                    #[allow(non_camel_case_types)]
                    struct RequestCreatePatoSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::CreateRequest>
                    for RequestCreatePatoSvc<T> {
                        type Response = super::CreateResonse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::request_create_pato(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = RequestCreatePatoSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestPatoLogin" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPatoLoginSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::LoginRequest>
                    for RequestPatoLoginSvc<T> {
                        type Response = super::EmptyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LoginRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::request_pato_login(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = RequestPatoLoginSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestPray" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPraySvc<T: MetaPowerMatrixControllerSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::PrayRequest>
                    for RequestPraySvc<T> {
                        type Response = super::EmptyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PrayRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::request_pray(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = RequestPraySvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestMakeProfession" => {
                    #[allow(non_camel_case_types)]
                    struct RequestMakeProfessionSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::MakeProfessionRequest>
                    for RequestMakeProfessionSvc<T> {
                        type Response = super::EmptyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MakeProfessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::request_make_profession(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = RequestMakeProfessionSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestMakeTownMap" => {
                    #[allow(non_camel_case_types)]
                    struct RequestMakeTownMapSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::EmptyResponse>
                    for RequestMakeTownMapSvc<T> {
                        type Response = super::EmptyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyResponse>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::request_make_town_map(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = RequestMakeTownMapSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/MakePlanForPato" => {
                    #[allow(non_camel_case_types)]
                    struct MakePlanForPatoSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::MakePlanRequest>
                    for MakePlanForPatoSvc<T> {
                        type Response = super::MakePlanResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MakePlanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::make_plan_for_pato(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = MakePlanForPatoSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestPatoNearby" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPatoNearbySvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::NearbyRequest>
                    for RequestPatoNearbySvc<T> {
                        type Response = super::NearbyRespnse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NearbyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::request_pato_nearby(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = RequestPatoNearbySvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestPlaceInMap" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPlaceInMapSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::PlaceRequest>
                    for RequestPlaceInMapSvc<T> {
                        type Response = super::PlaceResonse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlaceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::request_place_in_map(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = RequestPlaceInMapSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/AcceptDonation" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptDonationSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::DonationRequest>
                    for AcceptDonationSvc<T> {
                        type Response = super::EmptyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DonationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::accept_donation(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = AcceptDonationSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/AcceptStake" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptStakeSvc<T: MetaPowerMatrixControllerSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::DonationRequest>
                    for AcceptStakeSvc<T> {
                        type Response = super::EmptyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DonationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::accept_stake(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = AcceptStakeSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/AcceptSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptSubscriptionSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::SubscriptionRequest>
                    for AcceptSubscriptionSvc<T> {
                        type Response = super::EmptyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::accept_subscription(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = AcceptSubscriptionSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestHotAI" => {
                    #[allow(non_camel_case_types)]
                    struct RequestHotAISvc<T: MetaPowerMatrixControllerSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::EmptyResponse>
                    for RequestHotAISvc<T> {
                        type Response = super::HotAiResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyResponse>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::request_hot_ai(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = RequestHotAISvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestSharedKnowledges" => {
                    #[allow(non_camel_case_types)]
                    struct RequestSharedKnowledgesSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::EmptyResponse>
                    for RequestSharedKnowledgesSvc<T> {
                        type Response = super::SharedKnowledgesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyResponse>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::request_shared_knowledges(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = RequestSharedKnowledgesSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestHotTopics" => {
                    #[allow(non_camel_case_types)]
                    struct RequestHotTopicsSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::EmptyResponse>
                    for RequestHotTopicsSvc<T> {
                        type Response = super::HotTopicResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyResponse>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::request_hot_topics(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = RequestHotTopicsSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/RequestPayCreditCard" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPayCreditCardSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::CreditCardPayRequest>
                    for RequestPayCreditCardSvc<T> {
                        type Response = super::CreditCardPayResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreditCardPayRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::request_pay_credit_card(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = RequestPayCreditCardSvc(inner);
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
                "/metapowermatrix_controller.MetaPowerMatrixControllerSvc/CheckPayOrders" => {
                    #[allow(non_camel_case_types)]
                    struct CheckPayOrdersSvc<T: MetaPowerMatrixControllerSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixControllerSvc,
                    > tonic::server::UnaryService<super::CheckPayOrdersRequest>
                    for CheckPayOrdersSvc<T> {
                        type Response = super::CheckPayOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckPayOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixControllerSvc>::check_pay_orders(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let inner = inner.0;
                        let method = CheckPayOrdersSvc(inner);
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
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: MetaPowerMatrixControllerSvc> Clone
    for MetaPowerMatrixControllerSvcServer<T> {
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
    impl<T: MetaPowerMatrixControllerSvc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MetaPowerMatrixControllerSvc> tonic::server::NamedService
    for MetaPowerMatrixControllerSvcServer<T> {
        const NAME: &'static str = "metapowermatrix_controller.MetaPowerMatrixControllerSvc";
    }
}
