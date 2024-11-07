use anyhow::Error;
use metapower_framework::{get_now_secs, ChatMessage};
use stable_fs::fs::{FdStat, OpenFlags};
use std::str::from_utf8;
use crate::FS;

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