use ic_cdk::call;
use metapower_framework::{dao::http::LLMSvcClient, KnowLedgeInfo, KnowLedgesRequest, SnResponse};

use crate::AGENT_CALLEE;

pub async fn ask_pato_knowledges(pato_id: String) -> Vec<KnowLedgeInfo>{
    let mut knowledges: Vec<KnowLedgeInfo> = vec![];
    let mut sn: i64 = -1;

    let callee = AGENT_CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());
    let (sn_resp,): (SnResponse,) =
        match call(callee, "request_pato_sn", (vec![pato_id.clone()],)).await {
            Ok(response) => response,
            Err((code, msg)) => {
                println!("become_kol失败: {}: {}", code as u8, msg);
                (SnResponse::default(),)
            },
        };
    let resp = sn_resp.pato_sn_id;
    if !resp.is_empty(){
        sn = resp[0].sn.parse::<i64>().unwrap_or(-1);
    }else{
        println!("ask_pato_name: not found this one");
    }

    if sn >= 0 {
        let client = LLMSvcClient::default();
        let req = KnowLedgesRequest { id: pato_id };
        match client.call_llm_proxy::<KnowLedgesRequest, Vec<KnowLedgeInfo>>("request_pato_knowledges", req).await{
            Ok(answer) => {
                knowledges = answer.clone();
            }
            Err(e) => {
                println!("ask_pato_knowledges error: {:?}", e);
            }
        }
    }

    knowledges
}
pub async fn ask_pato_name(pato_id: String)-> Option<String>{
    let callee = AGENT_CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());
    let (name,): (String,) =
        match call(callee, "request_pato_name", (pato_id.clone(),)).await {
            Ok(response) => response,
            Err((code, msg)) => {
                println!("request_pato_name: {}: {}", code as u8, msg);
                return None;
            },
        };

    Some(name)
}