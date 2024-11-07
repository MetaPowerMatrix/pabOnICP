pub mod identity;

use anyhow::{anyhow, Error};
use ic_cdk::call;
use metapower_framework::dao::http::{BSCSvcClient, LLMSvcClient, MetaPowerSvcClient};
use metapower_framework::{DataResponse, SimpleResponse, AI_PATO_DIR, MAX_SAVE_BYTES};
use metapower_framework::{
    get_now_secs, log,
    publish_battery_actions, AnswerReply,
    BecomeKolRequest, BestTalkRequest, BetterTalkRequest, EmptyRequest, ImageChatRequest, ImageChatResponse,
    ImageContextRequest, ImageContextResponse, ImageDescriptionRequest, ImageDescriptionResponse, ImagePromptRequest, InstructRequest,
    InstructResponse, JoinKolRoomRequest,
    MessageRequest, QuestionRequest,
    SomeDocs, SubmitTagsRequest, SubmitTagsResponse, SummarytResponse, SvcImageDescriptionRequest,
    SvcImageDescriptionResponse, TalkResponse, TextToSpeechRequest, TextToSpeechResponse, XFILES_LOCAL_DIR, XFILES_SERVER,
};
use stable_fs::fs::{FdStat, OpenFlags};
use std::str::from_utf8;

use crate::{AGENT_CALLEE, BATTERY_AVATAR, BATTERY_CHARACTER, BATTERY_COVER, BATTERY_TAGS, FS};

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
        summary_file: String,
        summary_content: Vec<u8>,
    ) -> Option<String> {
        let summary = FS.with(|fs|{
            let mut buffer = [0u8; MAX_SAVE_BYTES];
            let fd  = fs.borrow_mut().open_or_create(fs.borrow().root_fd(), &summary_file,
                FdStat::default(),OpenFlags::CREATE, get_now_secs()).unwrap();
            if let Ok(read_size) = fs.borrow_mut().read(fd, &mut buffer){
                return from_utf8(&buffer[..read_size as usize]).unwrap_or_default().to_string();
            }

            String::default()
        });
        if !summary.is_empty(){
            return Some(summary);
        }
        let llm_client = LLMSvcClient::default();
        let llmrequest = SomeDocs {
            doc_file: summary_content,
            doc_format: "txt".to_string(),
        };
        match llm_client
            .got_documents_summary("got_documents_summary", llmrequest)
            .await
        {
            Ok(resp) => match serde_json::from_str::<SummarytResponse>(&resp) {
                Ok(sum_resp) => {
                    let summary = sum_resp.summary.clone();
                    FS.with(|fs|{
                        let fd = fs.borrow_mut().open_or_create(fs.borrow().root_fd(), &summary_file, 
                            FdStat::default(), OpenFlags::CREATE|OpenFlags::TRUNCATE, get_now_secs()).unwrap();
                        let _ = fs.borrow_mut().write(fd, summary.as_bytes());
                    });
                
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

        None
    }
    pub async fn talk(&self, request: MessageRequest) -> std::result::Result<TalkResponse, Error> {
        let chat_content = request;
        let input = chat_content.message.clone();
        let prompt = chat_content.prompt.clone();
        let db_path = format!("{}/{}/db/knowledge_chromadb", AI_PATO_DIR, self.id);

        let llm_client = LLMSvcClient::default();
        let chat_request = BestTalkRequest {
            question: input,
            collection_name: "".to_string(),
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

    pub async fn request_instruct(
        &self,
        request: InstructRequest,
    ) -> std::result::Result<InstructResponse, Error> {
        let mut response = InstructResponse {
            answer: String::default(),
        };
        let client = LLMSvcClient::default();
        let chat_request = BetterTalkRequest {
            question: request.message.clone(),
            collection_name: vec![],
            db_path: "".to_string(),
            prompt: request.message.clone(),
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
        if let Ok(resp) = BSCSvcClient::default().bsc_proxy_get::<String, DataResponse>(&format!("/api/kol/query/staking/{}", request.from), None).await {
            if resp.code != "200" && (resp.content.parse::<u64>().unwrap_or(0) < 10) {
                return Err(anyhow!("{}: {}", resp.code, resp.content));
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
        if let Ok(resp) = BSCSvcClient::default().bsc_proxy_get::<String, DataResponse>(&format!("/api/kol/query/ticket/{}", request.from), None).await {
            if resp.code != "200" && (resp.content.parse::<u64>().unwrap_or(0) < 10) {
                return Err(anyhow!("{}: {}", resp.code, resp.content));
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

    pub async fn request_submit_tags_with_proxy(
        &self,
        request: SubmitTagsRequest,
    ) -> std::result::Result<SubmitTagsResponse, Error> {
        let sub_resp: SubmitTagsResponse;

        BATTERY_TAGS.with(|tags| {
            let mut tags = tags.borrow_mut();
            tags.insert(request.id.clone(), request.tags.join(","));
        });

        let client = MetaPowerSvcClient::default();
        match client.metapower_proxy_post::<Vec<String>, DataResponse>(
                &format!("/api/pato/proxy/submit/tags/{}", request.id),
                request.tags,
            )
            .await
        {
            Ok(answer) => {
                sub_resp = serde_json::from_str::<SubmitTagsResponse>(&answer.content)?;
                let id = request.id.clone();

                BATTERY_CHARACTER.with(|character| {
                    let mut character = character.borrow_mut();
                    character.insert(id.clone(), sub_resp.character.clone());
                });

                BATTERY_COVER.with(|avatar| {
                    let mut avatar = avatar.borrow_mut();
                    avatar.insert(id.clone(), sub_resp.cover.clone());
                });
                BATTERY_AVATAR.with(|avatar| {
                    let mut avatar = avatar.borrow_mut();
                    avatar.insert(id, sub_resp.avatar.clone());
                });
            }
            Err(e) => {
                return Err(e);
            }
        }

        Ok(sub_resp)
    }
}
