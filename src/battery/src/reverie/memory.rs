use anyhow::Error;
use ic_cdk::call;
use metapower_framework::ChatMessage;
use crate::MATRIX_CALLEE;

pub async fn get_chat_his_by_session(sender_id: String, session: String) -> Result<Vec<ChatMessage>, Error>{
    let callee = MATRIX_CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());
    let (his_resp,): (Vec<u8>,) =
        match call(callee, "query_session_assets", (sender_id, session, "messages.json")).await {
            Ok(response) => response,
            Err((code, msg)) => {
                println!("become_kol失败: {}: {}", code as u8, msg);
                (vec![],)
            },
        };
    
    let messages: Vec<ChatMessage> = serde_json::from_slice(&his_resp)?;

    Ok(messages)
}

pub async fn get_knowledge_by_session(sender_id: String, session: String) -> Result<Vec<u8>, Error>{
    let callee = MATRIX_CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());
    let (his_resp,): (Vec<u8>,) =
        match call(callee, "query_session_assets", (sender_id, session, "knowledge.bin")).await {
            Ok(response) => response,
            Err((code, msg)) => {
                println!("become_kol失败: {}: {}", code as u8, msg);
                (vec![],)
            },
        };
    

    Ok(his_resp)
}

pub async fn get_knowledge_summary(sender_id: String, session: String) -> Result<Vec<u8>, Error>{
    let callee = MATRIX_CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());
    let (his_resp,): (Vec<u8>,) =
        match call(callee, "query_session_assets", (sender_id, session, "summary.txt")).await {
            Ok(response) => response,
            Err((code, msg)) => {
                println!("become_kol失败: {}: {}", code as u8, msg);
                (vec![],)
            },
        };
    

    Ok(his_resp)
}

pub async fn get_knowledge_embedding(sender_id: String, session: String) -> Result<Vec<u8>, Error>{
    let callee = MATRIX_CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());
    let (his_resp,): (Vec<u8>,) =
        match call(callee, "query_session_assets", (sender_id, session, "embedding.bin")).await {
            Ok(response) => response,
            Err((code, msg)) => {
                println!("become_kol失败: {}: {}", code as u8, msg);
                (vec![],)
            },
        };
    

    Ok(his_resp)
}