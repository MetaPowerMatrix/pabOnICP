use anyhow::Error;
use ic_cdk::call;
use metapower_framework::dao::http::LLMSvcClient;
use metapower_framework::{get_now_secs, ChatMessage, ProfessionalsResponse, SomeDocs, SummarytResponse};
use stable_fs::fs::{FdStat, OpenFlags};
use std::str::from_utf8;

use crate::{AGENT_CALLEE, FS};

pub fn save_session_chat_message(initial: String, kol: String, session: String, chat_messages: Vec<ChatMessage>) 
{
    let chat_session_message_file = format!(
        "messages/{}/{}/{}/message.json",
        initial,
        kol,
        session
    );
    FS.with(|fs|{
        let fd = fs.borrow_mut().open_or_create(fs.borrow().root_fd(), &chat_session_message_file, 
            FdStat::default(), OpenFlags::CREATE|OpenFlags::TRUNCATE, get_now_secs()).unwrap();
        fs.borrow_mut().write(fd, serde_json::to_string(&chat_messages).unwrap().as_bytes());
    });
}
pub fn archive_session_chat_message(initial: String, kol: String, session: String, chat_messages: String) 
{
    let chat_session_message_file = format!(
        "messages/archive/{}/{}/{}/message.json",
        initial,
        kol,
        session
    );
    FS.with(|fs|{
        let fd = fs.borrow_mut().open_or_create(fs.borrow().root_fd(), &chat_session_message_file, 
            FdStat::default(), OpenFlags::CREATE|OpenFlags::TRUNCATE, get_now_secs()).unwrap();
        fs.borrow_mut().write(fd, chat_messages.as_bytes());
    });
}

pub fn get_chat_his_by_session(kol: String, sender_id: String, session: String) -> Result<Vec<ChatMessage>, Error>{
    let chat_session_message_file = format!(
        "messages/{}/{}/{}/message.json",
        sender_id,
        kol,
        session
    );
    let mut messages = vec![];

    FS.with(|fs|{
        let mut buffer = [0u8; 1024*1024*5];
        let fd  = fs.borrow_mut().open_or_create(fs.borrow().root_fd(), &chat_session_message_file,
            FdStat::default(),OpenFlags::CREATE, get_now_secs()).unwrap();
        if let Ok(read_size) = fs.borrow_mut().read(fd, &mut buffer){
            messages = serde_json::from_str::<Vec<ChatMessage>>(from_utf8(&buffer[..read_size as usize]).unwrap_or_default()).unwrap_or_default();
        }
    });

    Ok(messages)
}

pub async fn get_pato_knowledges(id: String) -> Vec<String> {
    let mut professionals:Vec<String> = vec![];

    let callee = AGENT_CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());
    let (pro_resp,): (ProfessionalsResponse,) =
        match call(callee, "request_pato_knowledges", (id,)).await {
            Ok(response) => response,
            Err((code, msg)) => {
                println!("become_kol失败: {}: {}", code as u8, msg);
                (ProfessionalsResponse::default(),)
            },
        };

    for pro in pro_resp.professionals.iter(){
        let knowledges = pro.split(',').map(|k| k.to_owned()) .collect::<Vec<String>>();
        professionals.extend(knowledges);
    }

    professionals
}
pub async fn get_chat_messages_summary(summary_content: String) -> Result<String, Error> {
    let llm_client = LLMSvcClient::default();
    let llmrequest = SomeDocs {
        doc_file: summary_content,
        doc_format: "txt".to_string(),
    };
    match llm_client.call_llm_proxy::<SomeDocs, SummarytResponse>("got_documents_summary", llmrequest).await{
        Ok(sum_resp) => {
            let summary = sum_resp.summary.clone();
            Ok(summary)
        }
        Err(e) => {
            Err(e)
        }
    }
}
