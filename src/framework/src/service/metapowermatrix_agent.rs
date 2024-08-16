#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserActiveRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub page: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub action: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveRoomInfo {
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub town: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub role1: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub role2: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveRoomListResponse {
    #[prost(message, repeated, tag = "1")]
    pub rooms: ::prost::alloc::vec::Vec<LiveRoomInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomInfo {
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub town: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomListResponse {
    #[prost(message, repeated, tag = "1")]
    pub rooms: ::prost::alloc::vec::Vec<RoomInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomCreateRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub town: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomCreateResponse {
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicChatHisResponse {
    #[prost(string, repeated, tag = "1")]
    pub history: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicChatRequest {
    #[prost(string, tag = "1")]
    pub initial: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub topic: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub town: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenRequest {
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatoOfPro {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub subjects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveChatSessionResponse {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub role_1: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub role_2: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub topic: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectHumanVoiceRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub session: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveRegRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub role1: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub role2: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub topic: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub room_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeBalanceRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(float, tag = "2")]
    pub amount: f32,
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopulationRegistrationRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AirdropRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(float, tag = "2")]
    pub amount: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatoInfoResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub sn: i64,
    #[prost(string, tag = "4")]
    pub registered_datetime: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub professionals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(float, tag = "6")]
    pub balance: f32,
    #[prost(string, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "8")]
    pub avatar: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatoOfProResponse {
    #[prost(message, repeated, tag = "1")]
    pub patos: ::prost::alloc::vec::Vec<PatoOfPro>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfessionalsResponse {
    #[prost(string, repeated, tag = "1")]
    pub professionals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfessionalRegistrationRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub knowledge: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub subject: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KolRegistrationRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowKolRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub follower: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KolRelations {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub follower: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KolListResponse {
    #[prost(message, repeated, tag = "1")]
    pub relations: ::prost::alloc::vec::Vec<KolRelations>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnRequest {
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameRequest {
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnIdPaire {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sn: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnResponse {
    #[prost(message, repeated, tag = "1")]
    pub pato_sn_id: ::prost::alloc::vec::Vec<SnIdPaire>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamePros {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub pros: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameResponse {
    #[prost(message, repeated, tag = "1")]
    pub name_pros: ::prost::alloc::vec::Vec<NamePros>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllPatosResponse {
    #[prost(message, repeated, tag = "1")]
    pub pato_sn_id: ::prost::alloc::vec::Vec<SnIdPaire>,
}
/// Generated client implementations.
pub mod meta_power_matrix_agent_svc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MetaPowerMatrixAgentSvcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MetaPowerMatrixAgentSvcClient<tonic::transport::Channel> {
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
    impl<T> MetaPowerMatrixAgentSvcClient<T>
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
        ) -> MetaPowerMatrixAgentSvcClient<InterceptedService<T, F>>
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
            MetaPowerMatrixAgentSvcClient::new(
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
        pub async fn request_airdrop(
            &mut self,
            request: impl tonic::IntoRequest<super::AirdropRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestAirdrop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestAirdrop",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_population_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::PopulationRegistrationRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPopulationRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestPopulationRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_pato_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatoInfoResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestPatoInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_pato_professionals(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProfessionalsResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoProfessionals",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestPatoProfessionals",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_for_professionals(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatoOfProResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestForProfessionals",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestForProfessionals",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_professional_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::ProfessionalRegistrationRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestProfessionalRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestProfessionalRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_pato_sn(
            &mut self,
            request: impl tonic::IntoRequest<super::SnRequest>,
        ) -> std::result::Result<tonic::Response<super::SnResponse>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoSn",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestPatoSn",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_for_all_patos(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllPatosResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestForAllPatos",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestForAllPatos",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_pato_name_and_pro(
            &mut self,
            request: impl tonic::IntoRequest<super::NameRequest>,
        ) -> std::result::Result<tonic::Response<super::NameResponse>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoNameAndPro",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestPatoNameAndPro",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_add_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeBalanceRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestAddBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestAddBalance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_minus_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeBalanceRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestMinusBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestMinusBalance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_deposit(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeBalanceRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestDeposit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestDeposit",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_live_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::LiveRegRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestLiveRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestLiveRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_inject_human_voice(
            &mut self,
            request: impl tonic::IntoRequest<super::InjectHumanVoiceRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestInjectHumanVoice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestInjectHumanVoice",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_live_chat_session_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LiveChatSessionResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestLiveChatSessionInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestLiveChatSessionInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_pato_auth_token(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoAuthToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestPatoAuthToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_pato_auth_token(
            &mut self,
            request: impl tonic::IntoRequest<super::TokenRequest>,
        ) -> std::result::Result<tonic::Response<super::TokenResponse>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/QueryPatoAuthToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "QueryPatoAuthToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_pato_knowledges(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProfessionalsResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoKnowledges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestPatoKnowledges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_topic_chat(
            &mut self,
            request: impl tonic::IntoRequest<super::TopicChatRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestTopicChat",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestTopicChat",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_topic_chat_history(
            &mut self,
            request: impl tonic::IntoRequest<super::TopicChatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TopicChatHisResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestTopicChatHistory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestTopicChatHistory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_create_room(
            &mut self,
            request: impl tonic::IntoRequest<super::RoomCreateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RoomCreateResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestCreateRoom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestCreateRoom",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_game_room_list(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RoomListResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestGameRoomList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestGameRoomList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn log_user_active(
            &mut self,
            request: impl tonic::IntoRequest<super::UserActiveRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/LogUserActive",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "LogUserActive",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_pato_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleRequest>,
        ) -> std::result::Result<tonic::Response<super::NameResponse>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestPatoByName",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_live_room_list(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LiveRoomListResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestLiveRoomList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestLiveRoomList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_kol_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::KolRegistrationRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestKolRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestKolRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_kol_list(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KolListResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestKolList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestKolList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_add_kol_follower(
            &mut self,
            request: impl tonic::IntoRequest<super::FollowKolRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestAddKolFollower",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestAddKolFollower",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_marriage_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::KolRegistrationRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestMarriageRegistration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestMarriageRegistration",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_add_marriage_follower(
            &mut self,
            request: impl tonic::IntoRequest<super::FollowKolRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status> {
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestAddMarriageFollower",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestAddMarriageFollower",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_marriage_list(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KolListResponse>,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestMarriageList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "metapowermatrix_agent.MetaPowerMatrixAgentSvc",
                        "RequestMarriageList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod meta_power_matrix_agent_svc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MetaPowerMatrixAgentSvcServer.
    #[async_trait]
    pub trait MetaPowerMatrixAgentSvc: Send + Sync + 'static {
        async fn request_airdrop(
            &self,
            request: tonic::Request<super::AirdropRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status>;
        async fn request_population_registration(
            &self,
            request: tonic::Request<super::PopulationRegistrationRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status>;
        async fn request_pato_info(
            &self,
            request: tonic::Request<super::SimpleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatoInfoResponse>,
            tonic::Status,
        >;
        async fn request_pato_professionals(
            &self,
            request: tonic::Request<super::SimpleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProfessionalsResponse>,
            tonic::Status,
        >;
        async fn request_for_professionals(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatoOfProResponse>,
            tonic::Status,
        >;
        async fn request_professional_registration(
            &self,
            request: tonic::Request<super::ProfessionalRegistrationRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status>;
        async fn request_pato_sn(
            &self,
            request: tonic::Request<super::SnRequest>,
        ) -> std::result::Result<tonic::Response<super::SnResponse>, tonic::Status>;
        async fn request_for_all_patos(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllPatosResponse>,
            tonic::Status,
        >;
        async fn request_pato_name_and_pro(
            &self,
            request: tonic::Request<super::NameRequest>,
        ) -> std::result::Result<tonic::Response<super::NameResponse>, tonic::Status>;
        async fn request_add_balance(
            &self,
            request: tonic::Request<super::ChangeBalanceRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status>;
        async fn request_minus_balance(
            &self,
            request: tonic::Request<super::ChangeBalanceRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status>;
        async fn request_deposit(
            &self,
            request: tonic::Request<super::ChangeBalanceRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status>;
        async fn request_live_registration(
            &self,
            request: tonic::Request<super::LiveRegRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status>;
        async fn request_inject_human_voice(
            &self,
            request: tonic::Request<super::InjectHumanVoiceRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status>;
        async fn request_live_chat_session_info(
            &self,
            request: tonic::Request<super::SimpleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LiveChatSessionResponse>,
            tonic::Status,
        >;
        async fn request_pato_auth_token(
            &self,
            request: tonic::Request<super::SimpleRequest>,
        ) -> std::result::Result<tonic::Response<super::SimpleResponse>, tonic::Status>;
        async fn query_pato_auth_token(
            &self,
            request: tonic::Request<super::TokenRequest>,
        ) -> std::result::Result<tonic::Response<super::TokenResponse>, tonic::Status>;
        async fn request_pato_knowledges(
            &self,
            request: tonic::Request<super::SimpleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProfessionalsResponse>,
            tonic::Status,
        >;
        async fn request_topic_chat(
            &self,
            request: tonic::Request<super::TopicChatRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status>;
        async fn request_topic_chat_history(
            &self,
            request: tonic::Request<super::TopicChatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TopicChatHisResponse>,
            tonic::Status,
        >;
        async fn request_create_room(
            &self,
            request: tonic::Request<super::RoomCreateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RoomCreateResponse>,
            tonic::Status,
        >;
        async fn request_game_room_list(
            &self,
            request: tonic::Request<super::SimpleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RoomListResponse>,
            tonic::Status,
        >;
        async fn log_user_active(
            &self,
            request: tonic::Request<super::UserActiveRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status>;
        async fn request_pato_by_name(
            &self,
            request: tonic::Request<super::SimpleRequest>,
        ) -> std::result::Result<tonic::Response<super::NameResponse>, tonic::Status>;
        async fn request_live_room_list(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LiveRoomListResponse>,
            tonic::Status,
        >;
        async fn request_kol_registration(
            &self,
            request: tonic::Request<super::KolRegistrationRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status>;
        async fn request_kol_list(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> std::result::Result<tonic::Response<super::KolListResponse>, tonic::Status>;
        async fn request_add_kol_follower(
            &self,
            request: tonic::Request<super::FollowKolRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status>;
        async fn request_marriage_registration(
            &self,
            request: tonic::Request<super::KolRegistrationRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status>;
        async fn request_add_marriage_follower(
            &self,
            request: tonic::Request<super::FollowKolRequest>,
        ) -> std::result::Result<tonic::Response<super::EmptyRequest>, tonic::Status>;
        async fn request_marriage_list(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> std::result::Result<tonic::Response<super::KolListResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MetaPowerMatrixAgentSvcServer<T: MetaPowerMatrixAgentSvc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MetaPowerMatrixAgentSvc> MetaPowerMatrixAgentSvcServer<T> {
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
    for MetaPowerMatrixAgentSvcServer<T>
    where
        T: MetaPowerMatrixAgentSvc,
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestAirdrop" => {
                    #[allow(non_camel_case_types)]
                    struct RequestAirdropSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::AirdropRequest>
                    for RequestAirdropSvc<T> {
                        type Response = super::SimpleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AirdropRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_airdrop(
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
                        let method = RequestAirdropSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPopulationRegistration" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPopulationRegistrationSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::PopulationRegistrationRequest>
                    for RequestPopulationRegistrationSvc<T> {
                        type Response = super::SimpleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PopulationRegistrationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_population_registration(
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
                        let method = RequestPopulationRegistrationSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoInfo" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPatoInfoSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::SimpleRequest>
                    for RequestPatoInfoSvc<T> {
                        type Response = super::PatoInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_pato_info(
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
                        let method = RequestPatoInfoSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoProfessionals" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPatoProfessionalsSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::SimpleRequest>
                    for RequestPatoProfessionalsSvc<T> {
                        type Response = super::ProfessionalsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_pato_professionals(
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
                        let method = RequestPatoProfessionalsSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestForProfessionals" => {
                    #[allow(non_camel_case_types)]
                    struct RequestForProfessionalsSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::EmptyRequest>
                    for RequestForProfessionalsSvc<T> {
                        type Response = super::PatoOfProResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_for_professionals(
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
                        let method = RequestForProfessionalsSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestProfessionalRegistration" => {
                    #[allow(non_camel_case_types)]
                    struct RequestProfessionalRegistrationSvc<
                        T: MetaPowerMatrixAgentSvc,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::ProfessionalRegistrationRequest>
                    for RequestProfessionalRegistrationSvc<T> {
                        type Response = super::EmptyRequest;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ProfessionalRegistrationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_professional_registration(
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
                        let method = RequestProfessionalRegistrationSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoSn" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPatoSnSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::SnRequest>
                    for RequestPatoSnSvc<T> {
                        type Response = super::SnResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SnRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_pato_sn(
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
                        let method = RequestPatoSnSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestForAllPatos" => {
                    #[allow(non_camel_case_types)]
                    struct RequestForAllPatosSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::EmptyRequest>
                    for RequestForAllPatosSvc<T> {
                        type Response = super::AllPatosResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_for_all_patos(
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
                        let method = RequestForAllPatosSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoNameAndPro" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPatoNameAndProSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::NameRequest>
                    for RequestPatoNameAndProSvc<T> {
                        type Response = super::NameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NameRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_pato_name_and_pro(
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
                        let method = RequestPatoNameAndProSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestAddBalance" => {
                    #[allow(non_camel_case_types)]
                    struct RequestAddBalanceSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::ChangeBalanceRequest>
                    for RequestAddBalanceSvc<T> {
                        type Response = super::SimpleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeBalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_add_balance(
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
                        let method = RequestAddBalanceSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestMinusBalance" => {
                    #[allow(non_camel_case_types)]
                    struct RequestMinusBalanceSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::ChangeBalanceRequest>
                    for RequestMinusBalanceSvc<T> {
                        type Response = super::SimpleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeBalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_minus_balance(
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
                        let method = RequestMinusBalanceSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestDeposit" => {
                    #[allow(non_camel_case_types)]
                    struct RequestDepositSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::ChangeBalanceRequest>
                    for RequestDepositSvc<T> {
                        type Response = super::SimpleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeBalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_deposit(
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
                        let method = RequestDepositSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestLiveRegistration" => {
                    #[allow(non_camel_case_types)]
                    struct RequestLiveRegistrationSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::LiveRegRequest>
                    for RequestLiveRegistrationSvc<T> {
                        type Response = super::EmptyRequest;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LiveRegRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_live_registration(
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
                        let method = RequestLiveRegistrationSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestInjectHumanVoice" => {
                    #[allow(non_camel_case_types)]
                    struct RequestInjectHumanVoiceSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::InjectHumanVoiceRequest>
                    for RequestInjectHumanVoiceSvc<T> {
                        type Response = super::EmptyRequest;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InjectHumanVoiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_inject_human_voice(
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
                        let method = RequestInjectHumanVoiceSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestLiveChatSessionInfo" => {
                    #[allow(non_camel_case_types)]
                    struct RequestLiveChatSessionInfoSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::SimpleRequest>
                    for RequestLiveChatSessionInfoSvc<T> {
                        type Response = super::LiveChatSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_live_chat_session_info(
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
                        let method = RequestLiveChatSessionInfoSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoAuthToken" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPatoAuthTokenSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::SimpleRequest>
                    for RequestPatoAuthTokenSvc<T> {
                        type Response = super::SimpleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_pato_auth_token(
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
                        let method = RequestPatoAuthTokenSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/QueryPatoAuthToken" => {
                    #[allow(non_camel_case_types)]
                    struct QueryPatoAuthTokenSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::TokenRequest>
                    for QueryPatoAuthTokenSvc<T> {
                        type Response = super::TokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TokenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::query_pato_auth_token(
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
                        let method = QueryPatoAuthTokenSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoKnowledges" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPatoKnowledgesSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::SimpleRequest>
                    for RequestPatoKnowledgesSvc<T> {
                        type Response = super::ProfessionalsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_pato_knowledges(
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
                        let method = RequestPatoKnowledgesSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestTopicChat" => {
                    #[allow(non_camel_case_types)]
                    struct RequestTopicChatSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::TopicChatRequest>
                    for RequestTopicChatSvc<T> {
                        type Response = super::EmptyRequest;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TopicChatRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_topic_chat(
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
                        let method = RequestTopicChatSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestTopicChatHistory" => {
                    #[allow(non_camel_case_types)]
                    struct RequestTopicChatHistorySvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::TopicChatRequest>
                    for RequestTopicChatHistorySvc<T> {
                        type Response = super::TopicChatHisResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TopicChatRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_topic_chat_history(
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
                        let method = RequestTopicChatHistorySvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestCreateRoom" => {
                    #[allow(non_camel_case_types)]
                    struct RequestCreateRoomSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::RoomCreateRequest>
                    for RequestCreateRoomSvc<T> {
                        type Response = super::RoomCreateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RoomCreateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_create_room(
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
                        let method = RequestCreateRoomSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestGameRoomList" => {
                    #[allow(non_camel_case_types)]
                    struct RequestGameRoomListSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::SimpleRequest>
                    for RequestGameRoomListSvc<T> {
                        type Response = super::RoomListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_game_room_list(
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
                        let method = RequestGameRoomListSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/LogUserActive" => {
                    #[allow(non_camel_case_types)]
                    struct LogUserActiveSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::UserActiveRequest>
                    for LogUserActiveSvc<T> {
                        type Response = super::EmptyRequest;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserActiveRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::log_user_active(
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
                        let method = LogUserActiveSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestPatoByName" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPatoByNameSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::SimpleRequest>
                    for RequestPatoByNameSvc<T> {
                        type Response = super::NameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_pato_by_name(
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
                        let method = RequestPatoByNameSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestLiveRoomList" => {
                    #[allow(non_camel_case_types)]
                    struct RequestLiveRoomListSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::EmptyRequest>
                    for RequestLiveRoomListSvc<T> {
                        type Response = super::LiveRoomListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_live_room_list(
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
                        let method = RequestLiveRoomListSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestKolRegistration" => {
                    #[allow(non_camel_case_types)]
                    struct RequestKolRegistrationSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::KolRegistrationRequest>
                    for RequestKolRegistrationSvc<T> {
                        type Response = super::EmptyRequest;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KolRegistrationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_kol_registration(
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
                        let method = RequestKolRegistrationSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestKolList" => {
                    #[allow(non_camel_case_types)]
                    struct RequestKolListSvc<T: MetaPowerMatrixAgentSvc>(pub Arc<T>);
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::EmptyRequest>
                    for RequestKolListSvc<T> {
                        type Response = super::KolListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_kol_list(
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
                        let method = RequestKolListSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestAddKolFollower" => {
                    #[allow(non_camel_case_types)]
                    struct RequestAddKolFollowerSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::FollowKolRequest>
                    for RequestAddKolFollowerSvc<T> {
                        type Response = super::EmptyRequest;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FollowKolRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_add_kol_follower(
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
                        let method = RequestAddKolFollowerSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestMarriageRegistration" => {
                    #[allow(non_camel_case_types)]
                    struct RequestMarriageRegistrationSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::KolRegistrationRequest>
                    for RequestMarriageRegistrationSvc<T> {
                        type Response = super::EmptyRequest;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KolRegistrationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_marriage_registration(
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
                        let method = RequestMarriageRegistrationSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestAddMarriageFollower" => {
                    #[allow(non_camel_case_types)]
                    struct RequestAddMarriageFollowerSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::FollowKolRequest>
                    for RequestAddMarriageFollowerSvc<T> {
                        type Response = super::EmptyRequest;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FollowKolRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_add_marriage_follower(
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
                        let method = RequestAddMarriageFollowerSvc(inner);
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
                "/metapowermatrix_agent.MetaPowerMatrixAgentSvc/RequestMarriageList" => {
                    #[allow(non_camel_case_types)]
                    struct RequestMarriageListSvc<T: MetaPowerMatrixAgentSvc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MetaPowerMatrixAgentSvc,
                    > tonic::server::UnaryService<super::EmptyRequest>
                    for RequestMarriageListSvc<T> {
                        type Response = super::KolListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MetaPowerMatrixAgentSvc>::request_marriage_list(
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
                        let method = RequestMarriageListSvc(inner);
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
    impl<T: MetaPowerMatrixAgentSvc> Clone for MetaPowerMatrixAgentSvcServer<T> {
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
    impl<T: MetaPowerMatrixAgentSvc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MetaPowerMatrixAgentSvc> tonic::server::NamedService
    for MetaPowerMatrixAgentSvcServer<T> {
        const NAME: &'static str = "metapowermatrix_agent.MetaPowerMatrixAgentSvc";
    }
}
