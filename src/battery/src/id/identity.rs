use ic_cdk::call;
use crate::{reverie::memory::get_knowledge_by_session, AGENT_CALLEE};

pub async fn ask_pato_knowledges(pato_id: String, session: String) -> Vec<u8>{
    get_knowledge_by_session(pato_id, session).await.unwrap_or(vec![])
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