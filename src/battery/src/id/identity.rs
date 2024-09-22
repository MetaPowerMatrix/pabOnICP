use ic_cdk::call;
use metapower_framework::{EmptyRequest, KnowLedgeInfo, KnowLedgesRequest, SnResponse};

use crate::{LLMSvcClient, AGENT_CALLEE};

pub fn get_pato_name(id: String)-> Option<String>{
    let name = String::default();
    Some(name)
}

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
        let req = EmptyRequest {};
        match client.call_llm_proxy::<EmptyRequest, String>("request_pato_name", req).await{
            Ok(answer) => {
                return Some(answer);
            }
            Err(e) => {
                println!("request_pato_name error: {:?}", e);
            }
        }
    }

    None
}