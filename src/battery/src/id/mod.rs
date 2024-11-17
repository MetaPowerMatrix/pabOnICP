pub mod identity;

use anyhow::{anyhow, Error};
use ic_cdk::call;
use metapower_framework::dao::http::{BSCSvcClient, LLMSvcClient, MetaPowerSvcClient};
use metapower_framework::{DataResponse, SimpleResponse};
use metapower_framework::{
    log,
    BecomeKolRequest, ImageDescriptionRequest, ImageDescriptionResponse, JoinKolRoomRequest, SubmitTagsRequest, SvcImageDescriptionRequest,
    SvcImageDescriptionResponse,
};

use crate::reverie::memory::get_knowledge_summary;
use crate::{PlainDoc, VecQuery, AGENT_CALLEE, VECTOR_CALLEE};

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
        id: String,
        session: String,
    ) -> Vec<u8> {
        get_knowledge_summary(id, session).await.unwrap_or(vec![])
    }

    pub async fn talk(&self, query: Vec<f32>) -> std::result::Result<Vec<PlainDoc>, Error> {
        let request = VecQuery::Embeddings(query);
        let callee = VECTOR_CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());
        let (result,): (Option<Vec<PlainDoc>>,) = match call(
            callee,
            "search",
            (request,2 as usize,),
        )
        .await
        {
            Ok(result) => result,
            Err((code, msg)) => return Err(anyhow!("{}: {}", code as u8, msg)),
        };

        Ok(result.unwrap_or_default())
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

    pub async fn request_submit_tags_with_proxy(
        &self,
        request: SubmitTagsRequest,
    ) -> std::result::Result<(), Error> {

        let client = MetaPowerSvcClient::default();
        match client.metapower_proxy_post::<Vec<String>, DataResponse>(
                &format!("/api/pato/proxy/submit/tags/{}/{}", request.id, request.session),
                request.tags,
            )
            .await
        {
            Ok(_) => (),
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }
}
