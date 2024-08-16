#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LlmEmptyResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImagePromptRequest {
    #[prost(string, tag = "1")]
    pub image_url: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub prompt: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub input: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageDescriptionRequest {
    #[prost(string, tag = "1")]
    pub image_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageChatRequest {
    #[prost(string, tag = "1")]
    pub image_url: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub question: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageDescriptionResponse {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XRetweetRequest {
    #[prost(string, tag = "1")]
    pub tweet_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XDirectMessageRequest {
    #[prost(string, tag = "1")]
    pub recipient_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEmbeddingsRequest {
    #[prost(string, tag = "1")]
    pub question: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub db_path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEmbeddingsResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechToTextRequest {
    #[prost(string, tag = "1")]
    pub audio_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechToTextResponse {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextToSpeechRequest {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextToSpeechResponse {
    #[prost(string, tag = "1")]
    pub audio_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageGenRequest {
    #[prost(string, tag = "1")]
    pub prompt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageGenResponse {
    #[prost(string, tag = "1")]
    pub image_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharacterGenRequest {
    #[prost(string, repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub gender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharacterGenResponse {
    #[prost(string, tag = "1")]
    pub iss: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiImagesGenRequest {
    #[prost(string, tag = "1")]
    pub prompt: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub num_images: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiImagesGenResponse {
    #[prost(string, repeated, tag = "1")]
    pub image_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BestTalkRequest {
    #[prost(string, tag = "1")]
    pub question: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub prompt: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub db_path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BetterTalkRequest {
    #[prost(string, tag = "1")]
    pub question: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub collection_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub db_path: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub prompt: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskDecompositionRequest {
    #[prost(string, tag = "1")]
    pub question: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskDecompositionResponse {
    #[prost(string, repeated, tag = "1")]
    pub plan: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatHistory {
    #[prost(string, tag = "1")]
    pub ai_message: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub human_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BetterQuestionRequest {
    #[prost(string, tag = "1")]
    pub question: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub stepback: bool,
    #[prost(string, repeated, tag = "3")]
    pub collection_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub db_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub chat_histories: ::prost::alloc::vec::Vec<ChatHistory>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BetterQuestionResponse {
    #[prost(string, tag = "1")]
    pub question_modified: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SomeDocs {
    #[prost(string, tag = "1")]
    pub doc_file: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub doc_format: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTopic {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub subjects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummarytResponse {
    #[prost(string, tag = "1")]
    pub summary: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectResponse {
    #[prost(string, tag = "1")]
    pub subject: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocsRequest {
    #[prost(string, tag = "1")]
    pub doc_file: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collection: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub db_path: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub doc_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub doc_format: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuestionRequest {
    #[prost(string, tag = "1")]
    pub question: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub persona: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnswerReply {
    #[prost(string, tag = "1")]
    pub answer: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TileClassifyRequest {
    #[prost(string, repeated, tag = "1")]
    pub name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub category: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TileTypeMap {
    #[prost(string, tag = "1")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TileClassifyResponse {
    #[prost(message, repeated, tag = "1")]
    pub classified_tiles: ::prost::alloc::vec::Vec<TileTypeMap>,
}
/// Generated client implementations.
pub mod chat_svc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ChatSvcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChatSvcClient<tonic::transport::Channel> {
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
    impl<T> ChatSvcClient<T>
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
        ) -> ChatSvcClient<InterceptedService<T, F>>
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
            ChatSvcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn talk(
            &mut self,
            request: impl tonic::IntoRequest<super::QuestionRequest>,
        ) -> std::result::Result<tonic::Response<super::AnswerReply>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/llmchat.ChatSvc/Talk");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("llmchat.ChatSvc", "Talk"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn talk_better(
            &mut self,
            request: impl tonic::IntoRequest<super::BetterTalkRequest>,
        ) -> std::result::Result<tonic::Response<super::AnswerReply>, tonic::Status> {
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
                "/llmchat.ChatSvc/TalkBetter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "TalkBetter"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn talk_best(
            &mut self,
            request: impl tonic::IntoRequest<super::BestTalkRequest>,
        ) -> std::result::Result<tonic::Response<super::AnswerReply>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/llmchat.ChatSvc/TalkBest");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("llmchat.ChatSvc", "TalkBest"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn embed_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::DocsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LlmEmptyResponse>,
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
                "/llmchat.ChatSvc/EmbedDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "EmbedDocuments"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn got_documents_summary(
            &mut self,
            request: impl tonic::IntoRequest<super::SomeDocs>,
        ) -> std::result::Result<
            tonic::Response<super::SummarytResponse>,
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
                "/llmchat.ChatSvc/GotDocumentsSummary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "GotDocumentsSummary"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn got_topic_subject(
            &mut self,
            request: impl tonic::IntoRequest<super::EventTopic>,
        ) -> std::result::Result<
            tonic::Response<super::SubjectResponse>,
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
                "/llmchat.ChatSvc/GotTopicSubject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "GotTopicSubject"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn classify_map_tile(
            &mut self,
            request: impl tonic::IntoRequest<super::TileClassifyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TileClassifyResponse>,
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
                "/llmchat.ChatSvc/ClassifyMapTile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "ClassifyMapTile"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn got_task_decomposition(
            &mut self,
            request: impl tonic::IntoRequest<super::TaskDecompositionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TaskDecompositionResponse>,
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
                "/llmchat.ChatSvc/GotTaskDecomposition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "GotTaskDecomposition"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn got_better_question(
            &mut self,
            request: impl tonic::IntoRequest<super::BetterQuestionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BetterQuestionResponse>,
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
                "/llmchat.ChatSvc/GotBetterQuestion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "GotBetterQuestion"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn gen_image_with_prompt(
            &mut self,
            request: impl tonic::IntoRequest<super::ImageGenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImageGenResponse>,
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
                "/llmchat.ChatSvc/GenImageWithPrompt",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "GenImageWithPrompt"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn text_to_speech(
            &mut self,
            request: impl tonic::IntoRequest<super::TextToSpeechRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TextToSpeechResponse>,
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
                "/llmchat.ChatSvc/TextToSpeech",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "TextToSpeech"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn speech_to_text(
            &mut self,
            request: impl tonic::IntoRequest<super::SpeechToTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SpeechToTextResponse>,
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
                "/llmchat.ChatSvc/SpeechToText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "SpeechToText"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_embbeedings(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryEmbeddingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryEmbeddingsResponse>,
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
                "/llmchat.ChatSvc/QueryEmbbeedings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "QueryEmbbeedings"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn x_retweet(
            &mut self,
            request: impl tonic::IntoRequest<super::XRetweetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LlmEmptyResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/llmchat.ChatSvc/XRetweet");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("llmchat.ChatSvc", "XRetweet"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn x_direct_message(
            &mut self,
            request: impl tonic::IntoRequest<super::XDirectMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LlmEmptyResponse>,
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
                "/llmchat.ChatSvc/XDirectMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "XDirectMessage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_image_description(
            &mut self,
            request: impl tonic::IntoRequest<super::ImageDescriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImageDescriptionResponse>,
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
                "/llmchat.ChatSvc/RequestImageDescription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "RequestImageDescription"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_image_description_with_prompt(
            &mut self,
            request: impl tonic::IntoRequest<super::ImagePromptRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImageDescriptionResponse>,
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
                "/llmchat.ChatSvc/RequestImageDescriptionWithPrompt",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "llmchat.ChatSvc",
                        "RequestImageDescriptionWithPrompt",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_image_chat(
            &mut self,
            request: impl tonic::IntoRequest<super::ImageChatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImageDescriptionResponse>,
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
                "/llmchat.ChatSvc/RequestImageChat",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "RequestImageChat"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn gen_multi_images_with_prompt(
            &mut self,
            request: impl tonic::IntoRequest<super::MultiImagesGenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MultiImagesGenResponse>,
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
                "/llmchat.ChatSvc/GenMultiImagesWithPrompt",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "GenMultiImagesWithPrompt"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn gen_character_with_prompt(
            &mut self,
            request: impl tonic::IntoRequest<super::CharacterGenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CharacterGenResponse>,
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
                "/llmchat.ChatSvc/GenCharacterWithPrompt",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("llmchat.ChatSvc", "GenCharacterWithPrompt"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod chat_svc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ChatSvcServer.
    #[async_trait]
    pub trait ChatSvc: Send + Sync + 'static {
        async fn talk(
            &self,
            request: tonic::Request<super::QuestionRequest>,
        ) -> std::result::Result<tonic::Response<super::AnswerReply>, tonic::Status>;
        async fn talk_better(
            &self,
            request: tonic::Request<super::BetterTalkRequest>,
        ) -> std::result::Result<tonic::Response<super::AnswerReply>, tonic::Status>;
        async fn talk_best(
            &self,
            request: tonic::Request<super::BestTalkRequest>,
        ) -> std::result::Result<tonic::Response<super::AnswerReply>, tonic::Status>;
        async fn embed_documents(
            &self,
            request: tonic::Request<super::DocsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LlmEmptyResponse>,
            tonic::Status,
        >;
        async fn got_documents_summary(
            &self,
            request: tonic::Request<super::SomeDocs>,
        ) -> std::result::Result<
            tonic::Response<super::SummarytResponse>,
            tonic::Status,
        >;
        async fn got_topic_subject(
            &self,
            request: tonic::Request<super::EventTopic>,
        ) -> std::result::Result<tonic::Response<super::SubjectResponse>, tonic::Status>;
        async fn classify_map_tile(
            &self,
            request: tonic::Request<super::TileClassifyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TileClassifyResponse>,
            tonic::Status,
        >;
        async fn got_task_decomposition(
            &self,
            request: tonic::Request<super::TaskDecompositionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TaskDecompositionResponse>,
            tonic::Status,
        >;
        async fn got_better_question(
            &self,
            request: tonic::Request<super::BetterQuestionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BetterQuestionResponse>,
            tonic::Status,
        >;
        async fn gen_image_with_prompt(
            &self,
            request: tonic::Request<super::ImageGenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImageGenResponse>,
            tonic::Status,
        >;
        async fn text_to_speech(
            &self,
            request: tonic::Request<super::TextToSpeechRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TextToSpeechResponse>,
            tonic::Status,
        >;
        async fn speech_to_text(
            &self,
            request: tonic::Request<super::SpeechToTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SpeechToTextResponse>,
            tonic::Status,
        >;
        async fn query_embbeedings(
            &self,
            request: tonic::Request<super::QueryEmbeddingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryEmbeddingsResponse>,
            tonic::Status,
        >;
        async fn x_retweet(
            &self,
            request: tonic::Request<super::XRetweetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LlmEmptyResponse>,
            tonic::Status,
        >;
        async fn x_direct_message(
            &self,
            request: tonic::Request<super::XDirectMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LlmEmptyResponse>,
            tonic::Status,
        >;
        async fn request_image_description(
            &self,
            request: tonic::Request<super::ImageDescriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImageDescriptionResponse>,
            tonic::Status,
        >;
        async fn request_image_description_with_prompt(
            &self,
            request: tonic::Request<super::ImagePromptRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImageDescriptionResponse>,
            tonic::Status,
        >;
        async fn request_image_chat(
            &self,
            request: tonic::Request<super::ImageChatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImageDescriptionResponse>,
            tonic::Status,
        >;
        async fn gen_multi_images_with_prompt(
            &self,
            request: tonic::Request<super::MultiImagesGenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MultiImagesGenResponse>,
            tonic::Status,
        >;
        async fn gen_character_with_prompt(
            &self,
            request: tonic::Request<super::CharacterGenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CharacterGenResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ChatSvcServer<T: ChatSvc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ChatSvc> ChatSvcServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ChatSvcServer<T>
    where
        T: ChatSvc,
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
                "/llmchat.ChatSvc/Talk" => {
                    #[allow(non_camel_case_types)]
                    struct TalkSvc<T: ChatSvc>(pub Arc<T>);
                    impl<T: ChatSvc> tonic::server::UnaryService<super::QuestionRequest>
                    for TalkSvc<T> {
                        type Response = super::AnswerReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuestionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::talk(&inner, request).await
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
                        let method = TalkSvc(inner);
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
                "/llmchat.ChatSvc/TalkBetter" => {
                    #[allow(non_camel_case_types)]
                    struct TalkBetterSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::BetterTalkRequest>
                    for TalkBetterSvc<T> {
                        type Response = super::AnswerReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BetterTalkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::talk_better(&inner, request).await
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
                        let method = TalkBetterSvc(inner);
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
                "/llmchat.ChatSvc/TalkBest" => {
                    #[allow(non_camel_case_types)]
                    struct TalkBestSvc<T: ChatSvc>(pub Arc<T>);
                    impl<T: ChatSvc> tonic::server::UnaryService<super::BestTalkRequest>
                    for TalkBestSvc<T> {
                        type Response = super::AnswerReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BestTalkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::talk_best(&inner, request).await
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
                        let method = TalkBestSvc(inner);
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
                "/llmchat.ChatSvc/EmbedDocuments" => {
                    #[allow(non_camel_case_types)]
                    struct EmbedDocumentsSvc<T: ChatSvc>(pub Arc<T>);
                    impl<T: ChatSvc> tonic::server::UnaryService<super::DocsRequest>
                    for EmbedDocumentsSvc<T> {
                        type Response = super::LlmEmptyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DocsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::embed_documents(&inner, request).await
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
                        let method = EmbedDocumentsSvc(inner);
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
                "/llmchat.ChatSvc/GotDocumentsSummary" => {
                    #[allow(non_camel_case_types)]
                    struct GotDocumentsSummarySvc<T: ChatSvc>(pub Arc<T>);
                    impl<T: ChatSvc> tonic::server::UnaryService<super::SomeDocs>
                    for GotDocumentsSummarySvc<T> {
                        type Response = super::SummarytResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SomeDocs>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::got_documents_summary(&inner, request).await
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
                        let method = GotDocumentsSummarySvc(inner);
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
                "/llmchat.ChatSvc/GotTopicSubject" => {
                    #[allow(non_camel_case_types)]
                    struct GotTopicSubjectSvc<T: ChatSvc>(pub Arc<T>);
                    impl<T: ChatSvc> tonic::server::UnaryService<super::EventTopic>
                    for GotTopicSubjectSvc<T> {
                        type Response = super::SubjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EventTopic>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::got_topic_subject(&inner, request).await
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
                        let method = GotTopicSubjectSvc(inner);
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
                "/llmchat.ChatSvc/ClassifyMapTile" => {
                    #[allow(non_camel_case_types)]
                    struct ClassifyMapTileSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::TileClassifyRequest>
                    for ClassifyMapTileSvc<T> {
                        type Response = super::TileClassifyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TileClassifyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::classify_map_tile(&inner, request).await
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
                        let method = ClassifyMapTileSvc(inner);
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
                "/llmchat.ChatSvc/GotTaskDecomposition" => {
                    #[allow(non_camel_case_types)]
                    struct GotTaskDecompositionSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::TaskDecompositionRequest>
                    for GotTaskDecompositionSvc<T> {
                        type Response = super::TaskDecompositionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TaskDecompositionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::got_task_decomposition(&inner, request)
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
                        let method = GotTaskDecompositionSvc(inner);
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
                "/llmchat.ChatSvc/GotBetterQuestion" => {
                    #[allow(non_camel_case_types)]
                    struct GotBetterQuestionSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::BetterQuestionRequest>
                    for GotBetterQuestionSvc<T> {
                        type Response = super::BetterQuestionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BetterQuestionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::got_better_question(&inner, request).await
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
                        let method = GotBetterQuestionSvc(inner);
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
                "/llmchat.ChatSvc/GenImageWithPrompt" => {
                    #[allow(non_camel_case_types)]
                    struct GenImageWithPromptSvc<T: ChatSvc>(pub Arc<T>);
                    impl<T: ChatSvc> tonic::server::UnaryService<super::ImageGenRequest>
                    for GenImageWithPromptSvc<T> {
                        type Response = super::ImageGenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImageGenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::gen_image_with_prompt(&inner, request).await
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
                        let method = GenImageWithPromptSvc(inner);
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
                "/llmchat.ChatSvc/TextToSpeech" => {
                    #[allow(non_camel_case_types)]
                    struct TextToSpeechSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::TextToSpeechRequest>
                    for TextToSpeechSvc<T> {
                        type Response = super::TextToSpeechResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TextToSpeechRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::text_to_speech(&inner, request).await
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
                        let method = TextToSpeechSvc(inner);
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
                "/llmchat.ChatSvc/SpeechToText" => {
                    #[allow(non_camel_case_types)]
                    struct SpeechToTextSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::SpeechToTextRequest>
                    for SpeechToTextSvc<T> {
                        type Response = super::SpeechToTextResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SpeechToTextRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::speech_to_text(&inner, request).await
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
                        let method = SpeechToTextSvc(inner);
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
                "/llmchat.ChatSvc/QueryEmbbeedings" => {
                    #[allow(non_camel_case_types)]
                    struct QueryEmbbeedingsSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::QueryEmbeddingsRequest>
                    for QueryEmbbeedingsSvc<T> {
                        type Response = super::QueryEmbeddingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryEmbeddingsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::query_embbeedings(&inner, request).await
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
                        let method = QueryEmbbeedingsSvc(inner);
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
                "/llmchat.ChatSvc/XRetweet" => {
                    #[allow(non_camel_case_types)]
                    struct XRetweetSvc<T: ChatSvc>(pub Arc<T>);
                    impl<T: ChatSvc> tonic::server::UnaryService<super::XRetweetRequest>
                    for XRetweetSvc<T> {
                        type Response = super::LlmEmptyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::XRetweetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::x_retweet(&inner, request).await
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
                        let method = XRetweetSvc(inner);
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
                "/llmchat.ChatSvc/XDirectMessage" => {
                    #[allow(non_camel_case_types)]
                    struct XDirectMessageSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::XDirectMessageRequest>
                    for XDirectMessageSvc<T> {
                        type Response = super::LlmEmptyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::XDirectMessageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::x_direct_message(&inner, request).await
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
                        let method = XDirectMessageSvc(inner);
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
                "/llmchat.ChatSvc/RequestImageDescription" => {
                    #[allow(non_camel_case_types)]
                    struct RequestImageDescriptionSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::ImageDescriptionRequest>
                    for RequestImageDescriptionSvc<T> {
                        type Response = super::ImageDescriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImageDescriptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::request_image_description(&inner, request)
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
                        let method = RequestImageDescriptionSvc(inner);
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
                "/llmchat.ChatSvc/RequestImageDescriptionWithPrompt" => {
                    #[allow(non_camel_case_types)]
                    struct RequestImageDescriptionWithPromptSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::ImagePromptRequest>
                    for RequestImageDescriptionWithPromptSvc<T> {
                        type Response = super::ImageDescriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImagePromptRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::request_image_description_with_prompt(
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
                        let method = RequestImageDescriptionWithPromptSvc(inner);
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
                "/llmchat.ChatSvc/RequestImageChat" => {
                    #[allow(non_camel_case_types)]
                    struct RequestImageChatSvc<T: ChatSvc>(pub Arc<T>);
                    impl<T: ChatSvc> tonic::server::UnaryService<super::ImageChatRequest>
                    for RequestImageChatSvc<T> {
                        type Response = super::ImageDescriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImageChatRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::request_image_chat(&inner, request).await
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
                        let method = RequestImageChatSvc(inner);
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
                "/llmchat.ChatSvc/GenMultiImagesWithPrompt" => {
                    #[allow(non_camel_case_types)]
                    struct GenMultiImagesWithPromptSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::MultiImagesGenRequest>
                    for GenMultiImagesWithPromptSvc<T> {
                        type Response = super::MultiImagesGenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MultiImagesGenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::gen_multi_images_with_prompt(
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
                        let method = GenMultiImagesWithPromptSvc(inner);
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
                "/llmchat.ChatSvc/GenCharacterWithPrompt" => {
                    #[allow(non_camel_case_types)]
                    struct GenCharacterWithPromptSvc<T: ChatSvc>(pub Arc<T>);
                    impl<
                        T: ChatSvc,
                    > tonic::server::UnaryService<super::CharacterGenRequest>
                    for GenCharacterWithPromptSvc<T> {
                        type Response = super::CharacterGenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CharacterGenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ChatSvc>::gen_character_with_prompt(&inner, request)
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
                        let method = GenCharacterWithPromptSvc(inner);
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
    impl<T: ChatSvc> Clone for ChatSvcServer<T> {
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
    impl<T: ChatSvc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ChatSvc> tonic::server::NamedService for ChatSvcServer<T> {
        const NAME: &'static str = "llmchat.ChatSvc";
    }
}
