pub mod identity;

use anyhow::{anyhow, Error};
use ic_cdk::api::call::reject_message;
use ic_cdk::call;
use metapower_framework::dao::crawler::download_image;
use metapower_framework::dao::http::{BSCSvcClient, LLMSvcClient};
use metapower_framework::{DataResponse, SimpleResponse, AI_PATO_DIR};
use metapower_framework::{
    ensure_directory_exists, get_event_subjects, get_now_secs, log,
    publish_battery_actions, AnswerReply, ArchiveMessageRequest,
    BecomeKolRequest, BestTalkRequest, BetterTalkRequest, CharacterGenRequest,
    CharacterGenResponse, ChatMessage, DocsRequest, DocumentSummaryRequest,
    DocumentSummaryResponse, EmptyRequest, EventTopic, GetMessageRequest, GetMessageResponse, ImageChatRequest, ImageChatResponse,
    ImageContextRequest, ImageContextResponse, ImageDescriptionRequest, ImageDescriptionResponse,
    ImageGenPromptRequest, ImageGenRequest, ImageGenResponse, ImagePromptRequest, InstructRequest,
    InstructResponse, JoinKolRoomRequest, KnowLedgeInfo,
    KnowLedgesRequest, KnowLedgesResponse, LlmEmptyResponse,
    MessageRequest, QueryEmbeddingRequest, QueryEmbeddingResponse, QueryEmbeddingsRequest,
    QueryEmbeddingsResponse, QuestionRequest, SessionMessages, ShareKnowLedgesRequest,
    SomeDocs, SubjectResponse, SubmitTagsRequest, SubmitTagsResponse, SummaryAndEmbeddingRequest,
    SummaryAndEmbeddingResponse, SummarytResponse, SvcImageDescriptionRequest,
    SvcImageDescriptionResponse, TalkResponse, TextToSpeechRequest, TextToSpeechResponse,
    AI_MATRIX_DIR, XFILES_LOCAL_DIR, XFILES_SERVER,
};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Read, Write};
use std::path::PathBuf;
use std::{fs, io};
use tempfile::NamedTempFile;

use crate::id::identity::{ask_pato_knowledges, ask_pato_name};
use crate::reverie::generate_prompt;
use crate::reverie::memory::{
    archive_session_chat_message, get_chat_his_by_session, get_chat_messages_summary, get_pato_knowledges, save_session_chat_message
};
use crate::{AGENT_CALLEE, BATTERY_AVATAR, BATTERY_CHARACTER, BATTERY_COVER, BATTERY_TAGS};

const MAX_SUBJECT_LEN: i32 = 22;

#[derive(Debug, Clone)]
pub struct MetaPowerMatrixBatteryService {
    id: String,
}

impl MetaPowerMatrixBatteryService {
    pub fn new(id: String) -> Self {
        MetaPowerMatrixBatteryService { id }
    }

    pub async fn get_session_messages_summary(
        &self,
        summary_file: PathBuf,
        summary_content: String,
    ) -> Option<String> {
        // log!("summary_file: {:?}", summary_file);
        if summary_file.exists() && summary_file.is_file() {
            let mut buffer = Vec::new();
            if let Ok(mut file) = File::open(summary_file) {
                match file.read_to_end(&mut buffer) {
                    Ok(_) => {
                        if let Ok(content) = String::from_utf8(buffer) {
                            return Some(content);
                        }
                    }
                    Err(e) => {
                        log!("read summary file error: {}", e);
                    }
                }
            }
        } else {
            let llm_client = LLMSvcClient::default();
            if let Ok(mut temp_file) = NamedTempFile::new() {
                if temp_file.write_all(summary_content.as_bytes()).is_ok() {
                    let _ = temp_file.flush();
                    let llmrequest = SomeDocs {
                        doc_file: temp_file.path().to_str().unwrap().to_string(),
                        doc_format: "txt".to_string(),
                    };
                    log!("llmrequest: {:?}", llmrequest);
                    match llm_client
                        .got_documents_summary("got_documents_summary", llmrequest)
                        .await
                    {
                        Ok(resp) => match serde_json::from_str::<SummarytResponse>(&resp) {
                            Ok(sum_resp) => {
                                let summary = sum_resp.summary.clone();
                                if let Ok(mut file) = OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .truncate(true)
                                    .open(summary_file.clone())
                                {
                                    let _ = write!(file, "{}", summary);
                                }
                                let _ = fs::remove_file(temp_file.path());
                                return Some(summary);
                            }
                            Err(e) => {
                                log!("got_documents_summary from LLM error: {}", e);
                            }
                        },
                        Err(e) => {
                            log!("got_documents_summary from LLM error: {}", e);
                        }
                    }
                } else {
                    log!("write temp file error");
                }
            } else {
                log!("create temp file error");
            }
        }

        None
    }
    pub async fn talk(&self, request: MessageRequest) -> std::result::Result<TalkResponse, Error> {
        let chat_content = request;
        let input = chat_content.message.clone();
        let prompt = chat_content.prompt.clone();
        let db_path = format!("{}/{}/db/knowledge_chromadb", AI_PATO_DIR, self.id);
        let knowledges = get_pato_knowledges(self.id.clone()).await;
        let collection = if knowledges.is_empty() {
            "general".to_string()
        } else {
            knowledges[0].clone()
        };

        let llm_client = LLMSvcClient::default();
        let chat_request = BestTalkRequest {
            question: input,
            collection_name: collection,
            db_path,
            prompt,
        };
        // println!("chat_request: {:?}", chat_request);
        match llm_client.talk_best("talk_best", chat_request).await {
            Ok(answer) => {
                // log!("- I({}) said: {}", self.id, answer.answer.clone());
                let response = TalkResponse {
                    speaker: self.id.clone(),
                    message: answer.answer.clone(),
                };
                return Ok(response);
            }
            Err(e) => {
                log!("My AI {} is something wrong: {}", self.id, e);
            }
        }

        Err(anyhow!("um, I didn't hear clearly"))
    }

    pub async fn get_chat_messages(
        &self,
        request: GetMessageRequest,
    ) -> std::result::Result<GetMessageResponse, Error> {
        let messages = get_chat_his_by_session(request.kol, request.id, request.session.clone())?;
        let summary = get_chat_messages_summary(serde_json::to_string(&messages).unwrap_or_default()).await?;
        // log!("summary: {:?}", summary);
        let session_message = SessionMessages {
            summary,
            messages,
            session: request.session,
        };
        let content = serde_json::to_string(&session_message).unwrap_or_default();

        let response = GetMessageResponse { content };

        Ok(response)
    }

    pub fn archive_chat_messages(
        &self,
        request: ArchiveMessageRequest,
    ) -> std::result::Result<(), Error> {
        archive_session_chat_message(request.id, request.kol, request.session, request.messages);

        Ok(())
    }
    pub async fn request_instruct(
        &self,
        request: InstructRequest,
    ) -> std::result::Result<InstructResponse, Error> {
        let mut response = InstructResponse {
            answer: String::default(),
        };
        let mut curr_input: Vec<String> = vec![];
        let kol_id = request.kol.clone();

        let kol_name = ask_pato_name(kol_id.clone()).await.unwrap_or_default();
        let my_name = ask_pato_name(self.id.clone()).await.unwrap_or_default();
        let mut session_messages: Vec<ChatMessage> =
            get_chat_his_by_session(request.reply_to.clone(), request.kol.clone(), request.session.clone())?;
        let raw_messages = session_messages
            .iter()
            .map(|m| my_name.clone() + ":" + &m.question + "\n" + &kol_name + ":" + &m.answer)
            .collect::<Vec<String>>();
        let summary_content = raw_messages.join("\n");
        let summary = get_chat_messages_summary(summary_content.clone())
            .await
            .unwrap_or_default();
        let filtered_messages = raw_messages
            .iter()
            .filter(|m| m.len() < 800)
            .map(|m| m.to_owned())
            .collect::<Vec<String>>();

        curr_input.push(my_name.clone()); //0
        curr_input.push(kol_name.clone()); //1
        curr_input.push(my_name.clone()); //2
        curr_input.push(kol_name.clone()); //3
        curr_input.push(summary); //4
        curr_input.push(filtered_messages.join("\n")); //5
        curr_input.push(kol_name.clone()); //6
        curr_input.push(my_name.clone()); //7
        curr_input.push(request.message.clone()); //8
        curr_input.push(kol_name.clone()); //9
        curr_input.push(my_name.clone()); //10
        curr_input.push(kol_name.clone()); //11
        curr_input.push(kol_name.clone()); //12

        let prompt_lib_file = format!("{}/template/plan/agent_chat_pro.txt", AI_MATRIX_DIR);
        let prompt = generate_prompt(curr_input, &prompt_lib_file);
        log!("kol_chat_prompt: {}", prompt);

        let knowledges = ask_pato_knowledges(kol_id.clone()).await;
        let filtered_knowledges = knowledges
            .iter()
            .filter(|k| k.owner == kol_id)
            .map(|k| k.to_owned())
            .collect::<Vec<KnowLedgeInfo>>();
        println!("kol_chat_knowledges: {:?}", filtered_knowledges);
        let client = LLMSvcClient::default();
        let chat_request = BetterTalkRequest {
            question: request.message.clone(),
            prompt,
            collection_name: filtered_knowledges
                .iter()
                .map(|k| "sig".to_string() + &k.sig)
                .collect::<Vec<String>>(),
            db_path: format!("{}/{}/db/knowledge_chromadb", AI_PATO_DIR, kol_id),
        };
        // println!("chat_request: {:?}", chat_request);
        match client
            .call_llm_proxy::<BetterTalkRequest, AnswerReply>("talk_better", chat_request)
            .await
        {
            Ok(answer) => {
                response.answer = answer.answer.clone();
                publish_battery_actions(
                    request.reply_to.clone() + "/instruct",
                    answer.answer.clone(),
                );

                let message = ChatMessage {
                    created_at: get_now_secs() as i64,
                    session: String::default(),
                    place: "online".to_string(),
                    sender: request.reply_to.clone(),
                    receiver: kol_id.clone(),
                    question: request.message.clone(),
                    answer: response.answer.clone(),
                    sender_role: "user".to_string(),
                    subject: "consultant".to_string(),
                };
                session_messages.push(message);
                save_session_chat_message(
                    request.reply_to.clone(),
                    kol_id.clone(),
                    request.session,
                    session_messages
                );

                let tts_request = TextToSpeechRequest {
                    text: answer.answer.clone(),
                };
                match client
                    .call_llm_proxy::<TextToSpeechRequest, TextToSpeechResponse>(
                        "text_to_speech",
                        tts_request,
                    )
                    .await
                {
                    Ok(audio_file) => {
                        let audio_url = XFILES_SERVER.to_string() + "/" + &audio_file.audio_url;
                        publish_battery_actions(
                            request.reply_to.clone() + "/instruct/voice",
                            audio_url,
                        );
                    }
                    Err(e) => {
                        log!("Instuct Text to speech is something wrong: {}", e);
                    }
                }
            }
            Err(e) => {
                log!("Instruct AI is something wrong: {}", e);
            }
        }
        Ok(response)
    }

    pub async fn request_summary_and_embedding(
        &self,
        request: SummaryAndEmbeddingRequest,
    ) -> std::result::Result<SummaryAndEmbeddingResponse, Error> {
        let link = request.link.clone();
        let knowledge_file = request.knowledge_file.clone();
        let knowledge_file_fullpath = format!(
            "{}/{}/knowledge/{}",
            AI_PATO_DIR, self.id, request.knowledge_file
        );
        let transcript_file = request.transcript_file.clone();
        let collection_prefix = "sig".to_string();
        let subjects = get_event_subjects();
        let mut my_subjects: Vec<String> = vec![];

        let mut file_format = String::from("txt");
        if let Ok(mut file) = File::open(knowledge_file_fullpath.clone()) {
            let mut buf = vec![0; 4096]; // Read more bytes to improve accuracy
            if file.read(&mut buf).is_ok() {
                match infer::get(&buf) {
                    Some(kind) => {
                        println!("File type: {:?}", kind.mime_type());
                        file_format = kind.mime_type().to_string();
                    }
                    None => println!("Could not determine file type"),
                }
            }
        }
        let llm_client = LLMSvcClient::default();
        if !knowledge_file.is_empty() {
            let llmrequest = SomeDocs {
                doc_file: knowledge_file_fullpath.clone(),
                doc_format: file_format.clone(),
            };
            log!("file llmrequest: {:?}", llmrequest);
            match llm_client
                .call_llm_proxy::<SomeDocs, SummarytResponse>("got_documents_summary", llmrequest)
                .await
            {
                Ok(sum_resp) => {
                    let summary = sum_resp.summary.clone();
                    let summary_file_path = format!(
                        "{}/{}/knowledge/{}.summary",
                        AI_PATO_DIR,
                        self.id,
                        request.knowledge_file_sig.clone()
                    );
                    match OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(summary_file_path)
                    {
                        Ok(mut sig_file) => {
                            let _ = sig_file.write_all(summary.as_bytes());
                        }
                        Err(e) => {
                            log!("write summary file error: {}", e);
                        }
                    }
                    let topic_subject_request = EventTopic {
                        topic: summary,
                        subjects: subjects
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>(),
                    };
                    if let Ok(response) = llm_client
                        .call_llm_proxy::<EventTopic, SubjectResponse>(
                            "got_topic_subject",
                            topic_subject_request,
                        )
                        .await
                    {
                        if response.subject.clone().len() < MAX_SUBJECT_LEN as usize {
                            my_subjects.push(response.subject.clone());
                        }
                    }
                }
                Err(e) => {
                    log!("got_documents_summary error: {}", e);
                }
            };
            let embed_request = DocsRequest {
                doc_file: knowledge_file_fullpath,
                collection: collection_prefix.clone() + &request.knowledge_file_sig.clone(),
                db_path: format!("{}/{}/db/knowledge_chromadb", AI_PATO_DIR, self.id.clone()),
                doc_id: request.knowledge_file_sig.clone(),
                doc_format: file_format,
            };

            if let Err(e) = llm_client
                .call_llm_proxy::<DocsRequest, LlmEmptyResponse>("embed_documents", embed_request)
                .await
            {
                log!("embed_documents error: {}", e);
            }
        }
        if !transcript_file.is_empty() {
            // process transcript file
            let file_format = String::from("txt");
            let llmrequest = SomeDocs {
                doc_file: transcript_file.clone(),
                doc_format: file_format.clone(),
            };
            log!("record llmrequest: {:?}", llmrequest);
            let sum_resp = llm_client
                .call_llm_proxy::<SomeDocs, SummarytResponse>("got_documents_summary", llmrequest)
                .await?;
            let summary = sum_resp.summary.clone();
            let summary_file_path = format!(
                "{}/{}/knowledge/{}.summary",
                AI_PATO_DIR,
                self.id,
                request.transcript_file_sig.clone()
            );
            if let Ok(mut sig_file) = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&summary_file_path)
            {
                let _ = sig_file.write_all(summary.as_bytes());
            }
            let topic_subject_request = EventTopic {
                topic: summary,
                subjects: subjects
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            };
            if let Ok(response) = llm_client
                .call_llm_proxy::<EventTopic, SubjectResponse>(
                    "got_topic_subject",
                    topic_subject_request,
                )
                .await
            {
                my_subjects.push(response.subject.clone());
            }
            let embed_request = DocsRequest {
                doc_file: transcript_file,
                collection: collection_prefix.clone() + &request.transcript_file_sig.clone(),
                db_path: format!("{}/{}/db/knowledge_chromadb", AI_PATO_DIR, self.id.clone()),
                doc_id: request.transcript_file_sig.clone(),
                doc_format: file_format,
            };

            if let Err(e) = llm_client
                .call_llm_proxy::<DocsRequest, LlmEmptyResponse>("embed_documents", embed_request)
                .await
            {
                log!("record embed_documents error: {}", e);
            }
        }
        if !link.is_empty() {
            // process web link
            let file_format = String::from("link");
            let llmrequest = SomeDocs {
                doc_file: link.clone(),
                doc_format: file_format.clone(),
            };
            log!("link llmrequest: {:?}", llmrequest);
            let sum_resp = llm_client
                .call_llm_proxy::<SomeDocs, SummarytResponse>("got_documents_summary", llmrequest)
                .await?;
            let summary = sum_resp.summary.clone();
            let summary_file_path = format!(
                "{}/{}/knowledge/{}.summary",
                AI_PATO_DIR,
                self.id,
                request.link_sig.clone()
            );
            if let Ok(mut sig_file) = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&summary_file_path)
            {
                let _ = sig_file.write_all(summary.as_bytes());
            }
            let topic_subject_request = EventTopic {
                topic: summary,
                subjects: subjects
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            };
            if let Ok(response) = llm_client
                .call_llm_proxy::<EventTopic, SubjectResponse>(
                    "got_topic_subject",
                    topic_subject_request,
                ).await
            {
                my_subjects.push(response.subject.clone());
            }
            let embed_request = DocsRequest {
                doc_file: link.clone(),
                collection: collection_prefix + &request.link_sig.clone(),
                db_path: format!("{}/{}/db/knowledge_chromadb", AI_PATO_DIR, self.id.clone()),
                doc_id: request.link_sig.clone(),
                doc_format: file_format,
            };

            if let Err(e) = llm_client
                .call_llm_proxy::<DocsRequest, LlmEmptyResponse>("embed_documents", embed_request)
                .await
            {
                log!("link embed_documents error: {}", e);
            }
        }

        let response = SummaryAndEmbeddingResponse {
            knowledge_file_sig: request.knowledge_file_sig.clone(),
            transcript_file_sig: request.transcript_file_sig.clone(),
        };

        Ok(response)
    }

    pub async fn request_query_embedding(
        &self,
        request: QueryEmbeddingRequest,
    ) -> std::result::Result<QueryEmbeddingResponse, Error> {
        let db_path = format!("{}/{}/db/knowledge_chromadb", AI_PATO_DIR, self.id.clone());
        let collection_prefix = "sig".to_string();
        let llm_client = LLMSvcClient::default();
        let llmrequest = QueryEmbeddingsRequest {
            collection_name: collection_prefix + &request.collection_name.clone(),
            question: request.query.clone(),
            db_path,
        };
        log!("query request: {:?}", llmrequest);
        let query_resp = llm_client
            .call_llm_proxy::<QueryEmbeddingsRequest, QueryEmbeddingsResponse>(
                "query_embbeedings",
                llmrequest,
            )
            .await?;
        let result = query_resp.result.clone();

        let response = QueryEmbeddingResponse { result };

        Ok(response)
    }

    pub fn request_document_summary(
        &self,
        request: DocumentSummaryRequest,
    ) -> std::result::Result<DocumentSummaryResponse, Error> {
        let mut summary = String::new();
        let summary_file_path = format!(
            "{}/{}/knowledge/{}.summary",
            AI_PATO_DIR, self.id, request.document
        );
        if let Ok(mut sig_file) = OpenOptions::new().read(true).open(summary_file_path) {
            let _ = sig_file.read_to_string(&mut summary);
        }

        let response = DocumentSummaryResponse { summary };

        Ok(response)
    }

    pub fn request_pato_knowledges(
        &self,
        _request: KnowLedgesRequest,
    ) -> std::result::Result<KnowLedgesResponse, Error> {
        let saved_file_sig = format!("{}/{}/knowledge/knowledge.sig", AI_PATO_DIR, self.id);
        let mut knowledges: Vec<KnowLedgeInfo> = vec![];

        let mut my_knowledges: Vec<String> = vec![];
        let file = OpenOptions::new().read(true).open(saved_file_sig)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            my_knowledges.push(line);
        }
        for knowledge in my_knowledges.iter() {
            let mut summary = String::new();
            let line = knowledge
                .split('#')
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            let title = if !line.is_empty() {
                line[0].clone()
            } else {
                String::default()
            };
            let sig = if line.len() > 1 {
                line[1].clone()
            } else {
                String::default()
            };
            let owner = if line.len() > 2 {
                line[2].clone()
            } else {
                self.id.clone()
            };
            let summary_file_path =
                format!("{}/{}/knowledge/{}.summary", AI_PATO_DIR, self.id, sig);
            if let Ok(mut sig_file) = OpenOptions::new().read(true).open(summary_file_path) {
                let _ = sig_file.read_to_string(&mut summary);
            }
            let info = KnowLedgeInfo {
                title: title.to_string(),
                sig: sig.to_string(),
                summary,
                owner,
            };
            knowledges.push(info);
        }
        let mut set = HashSet::new();
        let mut result = Vec::new();
        for item in knowledges {
            if set.insert(item.sig.clone()) {
                result.push(item.clone());
            }
        }

        let response = KnowLedgesResponse {
            knowledge_info: result,
        };

        Ok(response)
    }

    pub fn request_share_knowledge(
        &self,
        request: ShareKnowLedgesRequest,
    ) -> std::result::Result<EmptyRequest, Error> {
        let share_file = format!("{}/{}/knowledge/shared.txt", AI_PATO_DIR, self.id.clone());
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(share_file)
        {
            Ok(mut file) => {
                writeln!(file, "{}#{}\n", request.title.clone(), request.sig.clone())?;
            }
            Err(e) => {
                log!("share_knowledge write file error: {}", e);
            }
        }

        let response = EmptyRequest {};

        Ok(response)
    }

    pub async fn add_shared_knowledge(
        &self,
        request: ShareKnowLedgesRequest,
    ) -> std::result::Result<EmptyRequest, Error> {
        let saved_file_sig = format!("{}/{}/knowledge/knowledge.sig", AI_PATO_DIR, self.id);
        if let Ok(mut sig_file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(saved_file_sig)
        {
            let _ = sig_file.write_all(
                format!(
                    "{}#{}#{}\n",
                    request.title,
                    request.sig,
                    request.owner.clone()
                )
                .as_bytes(),
            );
        }

        let response = EmptyRequest {};

        Ok(response)
    }

    pub async fn request_image_description(
        &self,
        request: SvcImageDescriptionRequest,
    ) -> std::result::Result<SvcImageDescriptionResponse, Error> {
        let sample_image = request.image_url.clone();
        let mut resp = SvcImageDescriptionResponse {
            description: String::default(),
        };

        let client = LLMSvcClient::default();
        log!("sample image file url: {}", sample_image);
        let image_description_request = ImageDescriptionRequest {
            image_url: sample_image,
        };
        match client
            .call_llm_proxy::<ImageDescriptionRequest, ImageDescriptionResponse>(
                "request_image_description",
                image_description_request,
            )
            .await
        {
            Ok(answer) => {
                let description = answer.description.clone();
                resp.description = description;
            }
            Err(e) => {
                log!("image_description_request AI is something wrong: {}", e);
            }
        }
        Ok(resp)
    }

    pub async fn request_chat_with_image(
        &self,
        request: ImageChatRequest,
    ) -> std::result::Result<ImageChatResponse, Error> {
        let mut response = ImageChatResponse {
            answer: String::default(),
            answer_voice: String::default(),
        };
        let local_xfile = request.image_url.split('/').last().unwrap_or_default();
        let local_file = format!("{}/game/{}", XFILES_LOCAL_DIR, local_xfile);
        log!("local_file: {}", local_file);

        let client = LLMSvcClient::default();
        let chat_request = ImageChatRequest {
            image_url: request.image_url.clone(),
            reply_to: String::default(),
            message: String::default(),
            room_id: String::default(),
            level: 1,
        };
        println!("chat_image_request: {:?}", chat_request);
        match client
            .call_llm_proxy::<ImageChatRequest, ImageDescriptionResponse>(
                "request_image_chat",
                chat_request,
            )
            .await
        {
            Ok(answer) => {
                response.answer = answer.description.clone();
                let tts_request = TextToSpeechRequest {
                    text: answer.description.clone(),
                };
                match client
                    .call_llm_proxy::<TextToSpeechRequest, TextToSpeechResponse>(
                        "text_to_speech",
                        tts_request,
                    )
                    .await
                {
                    Ok(audio_file) => {
                        let audio_url = XFILES_SERVER.to_string() + "/" + &audio_file.audio_url;
                        response.answer_voice = audio_url;
                    }
                    Err(e) => {
                        log!(
                            "request_chat_with_image Text to speech is something wrong: {}",
                            e
                        );
                    }
                }
            }
            Err(e) => {
                log!("request_chat_with_image AI is something wrong: {}", e);
            }
        }
        Ok(response)
    }

    pub async fn become_kol(
        &self,
        request: BecomeKolRequest,
    ) -> std::result::Result<SimpleResponse, Error> {
        match BSCSvcClient::default().bsc_proxy_get::<String, DataResponse>(&format!("/api/kol/query/staking/{}", request.from), None).await{
            Ok(resp) => {
                if resp.code != "200" && (resp.content.parse::<u64>().unwrap_or(0) < 10) {
                    return Err(anyhow!("{}: {}", resp.code, resp.content));
                }
            }
            Err(e) => {
                // return Err(e);
            }
        }

        let callee = AGENT_CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());
        let (_,): ((),) = match call(
            callee,
            "request_kol_registration",
            (request.id.clone(),),
        )
        .await
        {
            Ok(response) => response,
            Err((code, msg)) => return Err(anyhow!("{}: {}", code as u8, msg)),
        };

        let (token_resp,): (SimpleResponse,) = match call(
            callee,
            "request_pato_kol_token",
            (request.id,),
        ).await
        {
            Ok(response) => response,
            Err((code, msg)) => return Err(anyhow!("{}: {}", code as u8, msg)),
        };

        Ok(token_resp)
    }

    pub async fn request_join_kol_room(
        &self,
        request: JoinKolRoomRequest,
    ) -> std::result::Result<(), Error> {
        match BSCSvcClient::default().bsc_proxy_get::<String, DataResponse>(&format!("/api/kol/query/ticket/{}", request.from), None).await{
            Ok(resp) => {
                if resp.code != "200" && (resp.content.parse::<u64>().unwrap_or(0) < 10) {
                    return Err(anyhow!("{}: {}", resp.code, resp.content));
                }
            }
            Err(e) => {
                // return Err(e);
            }
        }


        let callee = AGENT_CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());
        let (_,):((),) = match call(
            callee,
            "request_follow_kol",
            (
                request.kol,
                request.follower,
                request.key,
            ),
        ).await
        {
            Ok(response) => response,
            Err((code, msg)) => return Err(anyhow!("request_follow_kol: {}: {}", code as u8, msg)),
        };

        Ok(())
    }

    pub async fn request_image_context(
        &self,
        request: ImageContextRequest,
    ) -> std::result::Result<ImageContextResponse, Error> {
        let image_url = request.image_url.clone();
        let input = request.input.clone();
        let prompt = request.prompt.clone();

        let mut context = String::default();
        let client = LLMSvcClient::default();
        let chat_request = ImagePromptRequest {
            image_url,
            prompt,
            input,
        };
        // println!("chat_request: {:?}", chat_request);
        match client
            .call_llm_proxy::<ImagePromptRequest, ImageDescriptionResponse>(
                "request_image_description_with_prompt",
                chat_request,
            )
            .await
        {
            Ok(answer) => {
                context = answer.description.clone();
            }
            Err(e) => {
                log!("request_image_context AI is something wrong: {}", e);
            }
        }
        let response = ImageContextResponse { context };

        Ok(response)
    }

    pub async fn request_image_gen_prompt(
        &self,
        request: ImageGenPromptRequest,
    ) -> std::result::Result<ImageContextResponse, Error> {
        let mut curr_input: Vec<String> = vec![];
        let prompt_lib_file = format!("{}/template/plan/agent_chat_maker.txt", AI_MATRIX_DIR);
        let description = request.description.clone();
        let his = request.historical.clone();
        let cul = request.architectural.clone();
        let mut image_prompt = String::default();

        curr_input.push(description); //0
        curr_input.push(his); //1
        curr_input.push(cul); //2
        let maker_prompt = generate_prompt(curr_input, &prompt_lib_file);
        log!("maker_prompt: {}", maker_prompt);

        let client = LLMSvcClient::default();
        let chat_request = QuestionRequest {
            subject: String::default(),
            persona: maker_prompt,
            question: String::default(),
        };
        match client
            .call_llm_proxy::<QuestionRequest, AnswerReply>("talk", chat_request)
            .await
        {
            Ok(answer) => {
                image_prompt = answer.answer.clone();
            }
            Err(e) => {
                log!("Maker AI {} is something wrong: {}", self.id, e);
            }
        }

        let response = ImageContextResponse {
            context: image_prompt,
        };

        Ok(response)
    }

    pub async fn request_self_talk_for_today_plan(
        &self,
        _request: EmptyRequest,
    ) -> std::result::Result<EmptyRequest, Error> {
        let send_to = self.id.clone();
                let client = LLMSvcClient::default();
                let chat_request = QuestionRequest {
                    subject: String::default(),
                    persona: "I want to do something today".to_string(),
                    question: String::default(),
                };
                match client
                    .call_llm_proxy::<QuestionRequest, AnswerReply>("talk", chat_request)
                    .await
                {
                    Ok(answer) => {
                        publish_battery_actions(send_to.clone(), answer.answer.clone());
                    }
                    Err(e) => {
                        log!(
                            "request_self_talk_for_today_plan AI is something wrong: {}",
                            e
                        );
                    }
                }

        Ok(EmptyRequest {})
    }

    pub async fn request_submit_tags(
        &self,
        request: SubmitTagsRequest,
    ) -> std::result::Result<SubmitTagsResponse, Error> {
        let mut resp = SubmitTagsResponse::default();

        BATTERY_TAGS.with(|tags| {
            let mut tags = tags.borrow_mut();
            tags.insert(request.id.clone(), request.tags.join(","));
        });

        let client = LLMSvcClient::default();
        let tag_request = CharacterGenRequest {
            tags: request.tags.clone(),
            name: ask_pato_name(self.id.clone()).await.unwrap_or_default(),
            gender: "Unknown".to_string(),
        };
        match client
            .call_llm_proxy::<CharacterGenRequest, CharacterGenResponse>(
                "/api/gen/character",
                tag_request,
            )
            .await
        {
            Ok(answer) => {
                let id = request.id.clone();
                let iss = answer.iss.clone();

                BATTERY_CHARACTER.with(|character| {
                    let mut character = character.borrow_mut();
                    character.insert(id.clone(), iss.clone());
                });


                let mut xfiles_link_mock = format!("{}/ai/{}/{}", XFILES_SERVER, id, "cover.png");
                let image_request = ImageGenRequest {
                    prompt: answer.iss.clone(),
                };
                match client
                    .call_llm_proxy::<ImageGenRequest, ImageGenResponse>(
                        "/api/gen/image",
                        image_request,
                    )
                    .await
                {
                    Ok(answer) => {
                        resp.avatar = answer.image_url.clone();
                        match download_image(id.clone(), &resp.avatar, "cover.png".to_string()).await {
                            Ok(xfiles_link) => {
                                BATTERY_COVER.with(|avatar| {
                                    let mut avatar = avatar.borrow_mut();
                                    avatar.insert(id.clone(), xfiles_link_mock);
                                });
                            }
                            Err(e) => {
                                BATTERY_COVER.with(|avatar| {
                                    let mut avatar = avatar.borrow_mut();
                                    avatar.insert(id.clone(), xfiles_link_mock);
                                });
                                // return Err(e);
                            }
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
                xfiles_link_mock = format!("{}/ai/{}/{}", XFILES_SERVER, id, "avatar.png");
                let avatar_prompt = format!("Design an avatar that represents a fictional character or persona for storytelling or role-playing purposes. Provide details about the character's appearance, personality traits, and backstory to create a visually compelling and immersive avatar: {}", iss);
                let avatar_request = ImageGenRequest {
                    prompt: avatar_prompt,
                };
                match client
                    .call_llm_proxy::<ImageGenRequest, ImageGenResponse>(
                        "/api/gen/avatar",
                        avatar_request,
                    )
                    .await
                {
                    Ok(answer) => {
                        resp.avatar = answer.image_url.clone();
                        match download_image(id.clone(), &resp.avatar, "avatar.png".to_string()).await {
                            Ok(xfiles_link) => {
                                BATTERY_AVATAR.with(|avatar| {
                                    let mut avatar = avatar.borrow_mut();
                                    avatar.insert(id, xfiles_link_mock);
                                });
                            }
                            Err(e) => {
                                BATTERY_AVATAR.with(|avatar| {
                                    let mut avatar = avatar.borrow_mut();
                                    avatar.insert(id, xfiles_link_mock);
                                });
                                // return Err(e);
                            }
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        Ok(resp)
    }
}
