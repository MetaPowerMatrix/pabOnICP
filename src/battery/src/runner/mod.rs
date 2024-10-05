#![allow(clippy::too_many_arguments)]

use candid::Principal;
use ic_cdk::call;
use metapower_framework::dao::http::LLMSvcClient;
use metapower_framework::{AnswerReply, MessageRequest, SnResponse};
use metapower_framework::{
    dao::personality::Persona, ensure_directory_exists, get_event_subjects, get_now_date_str, get_now_secs, log, publish_battery_actions, read_and_writeback_json_file, ActionInfo, BetterTalkRequest, ChatMessage, EmptyRequest, KnowLedgeInfo, PatoLocation, QuestionRequest, AGENT_GRPC_REST_SERVER, AI_MATRIX_DIR, AI_PATO_DIR, BATTERY_GRPC_REST_SERVER, BATTERY_GRPC_SERVER_PORT_START, HAVEAREST, LLMCHAT_GRPC_REST_SERVER, MATRIX_GRPC_REST_SERVER, SNAP, TICK, XFILES_SERVER
};
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::{
    collections::{HashMap, HashSet},
    fs::OpenOptions,
    io::{self},
};

use crate::AGENT_CALLEE;
use crate::{
    id::identity::{ask_pato_knowledges, ask_pato_name},
    reverie::{
        generate_prompt,
        memory::{
            get_chat_his_by_session, get_kol_messages,
            get_kol_messages_summary, save_kol_chat_message,
        },
    },
};

const MAX_ROUND: u64 = 8;
const MAX_PRO_ROUND: u64 = 20;
const MAX_CHANCE_TALK_PER_DAY: i32 = 0;
const MAX_TALK_PER_PLACE: i32 = 1;

#[derive(Debug)]
pub struct BatteryRunner {
    pub version: String,
    pub id: String,
    pub sleep_mode: bool,
    pub sn: i64,
}

impl BatteryRunner {
    pub fn new(id: String, sn: i64) -> Self {
        BatteryRunner {
            version: "0.1.0".to_string(),
            id,
            sleep_mode: false,
            sn,
        }
    }

    fn get_random_topics(&self) -> String {
        get_event_subjects()[0].to_string()
    }
    async fn talk_to_pato(
        &self,
        receiver_sn: i64,
        prompt: String,
        subject: String,
        question: String,
    ) -> Option<String> {
        let battery_address = format!(
            "{}:{}",
            BATTERY_GRPC_REST_SERVER,
            receiver_sn + BATTERY_GRPC_SERVER_PORT_START
        );

        log!("talk to battery: {}", battery_address);
        let client = LLMSvcClient::default();
        let request = MessageRequest {
            message: question.clone(),
            subject,
            prompt,
        };
        match client.call_llm_proxy::<MessageRequest, AnswerReply>("talk", request).await {
            Ok(response) => {
                return Some(response.answer);
            }
            Err(e) => {
                log!("Error: {}", e);
            }
        }

        None
    }
    fn save_chat_message(
        &self,
        input: String,
        output: String,
        session: &String,
        place: String,
        sender: String,
        receiver: String,
        subject: String,
    ) {
        let mut chat_messages = vec![];
        let mut chat_messages_copy = vec![];
        let chat_message = ChatMessage {
            created_at: get_now_secs() as i64,
            session: session.clone(),
            place: place.clone(),
            sender,
            receiver: receiver.clone(),
            question: input,
            answer: output,
            subject,
            sender_role: "user".to_string(),
        };

        chat_messages.push(chat_message.clone());
        chat_messages_copy.push(chat_message);

        let chat_session_path = format!(
            "{}/{}/db/{}/{}",
            AI_PATO_DIR,
            self.id,
            get_now_date_str(),
            session,
        );
        if let Err(e) = ensure_directory_exists(&chat_session_path) {
            log!("ensure_directory_exists error: {}", e);
        }
        let message_file = chat_session_path.clone() + "/message.json";
        // log!("first write messages {:?} to file {}",chat_messages, chat_session_path);
        if let Err(e) = read_and_writeback_json_file(&message_file, &mut chat_messages) {
            log!("read_and_writeback_json_file error: {}", e);
        }

        let r_chat_session_path = format!(
            "{}/{}/db/{}/{}",
            AI_PATO_DIR,
            receiver,
            get_now_date_str(),
            session,
        );
        if let Err(e) = ensure_directory_exists(&r_chat_session_path) {
            log!("ensure_directory_exists error: {}", e);
        }
        let message_file = r_chat_session_path.clone() + "/message.json";
        // log!("second write messages {:?} to file {}",chat_messages_copy, r_chat_session_path);
        if let Err(e) = read_and_writeback_json_file(&message_file, &mut chat_messages_copy) {
            log!("read_and_writeback_json_file error for receiver: {}", e);
        }
    }

    async fn decided_to_talk(
        &self,
        name: &String,
        l_name: &String,
        events: Vec<(String, String)>,
        l_events: &[(String, String)],
        place: &String,
        my_iss: &Persona,
        l_iss: &Persona,
    ) -> (bool, String) {
        let mut curr_input = vec![];
        let prompt_lib_file = format!("{}/template/plan/decide_to_talk_v2.txt", AI_MATRIX_DIR);

        let (event, subject) = if let Some(recent_event) = events.last() {
            (
                format!(
                    "{} which is something about {}",
                    recent_event.0,
                    recent_event.1.clone()
                ),
                recent_event.1.clone(),
            )
        } else {
            let subject = self.get_random_topics();
            let event = format!("want to talk about {}", subject);
            (event, subject)
        };
        let l_event = if let Some(recent_event) = l_events.last() {
            format!(
                "{} which is something about {}",
                recent_event.0,
                recent_event.1.clone()
            )
        } else {
            let subject = self.get_random_topics();
            format!("want to talk about {}", subject)
        };
        let context = format!(
            "{} today {}, and {} today {}, they met at {}",
            name, event, l_name, l_event, place
        );

        curr_input.push(context); //0
        curr_input.push(get_now_date_str()); //1
        curr_input.push(name.to_owned()); //2
        curr_input.push(l_name.to_owned()); //3
        curr_input.push(place.to_owned()); //4
        curr_input.push(subject.to_owned()); //5
        curr_input.push(my_iss.currently.clone()); //6
        curr_input.push(l_iss.currently.clone()); //7
        curr_input.push(name.to_owned()); //8
        curr_input.push(l_name.to_owned()); //9

        let prompt = generate_prompt(curr_input, &prompt_lib_file);
        // log!("decide_to_talk prompt: {}", prompt);
        // log!("decide_to_talk collection_name: {}", subject);

        let client = LLMSvcClient::default();
        let chat_request = QuestionRequest {
            subject: subject.clone(),
            persona: prompt,
            question: String::default(),
        };
        // println!("chat_request: {:?}", chat_request);
        match client.call_llm_proxy::<QuestionRequest, AnswerReply>("talk", chat_request).await {
            Ok(answer) => {
                if answer.answer.contains("yes")
                    || answer.answer.contains("Yes")
                {
                    return (true, subject);
                }
            }
            Err(e) => {
                log!("My AI {} is something wrong: {}", self.id, e);
            }
        }

        (false, subject)
    }
    async fn start_to_talk(
        &self,
        name: &String,
        l_name: &String,
        events: Vec<(String, String)>,
        l_events: Vec<(String, String)>,
        place: &String,
        my_iss: &Persona,
        l_iss: &Persona,
    ) -> (String, Option<String>) {
        let mut curr_input = vec![];
        let prompt_lib_file = format!("{}/template/plan/create_conversation_v2.txt", AI_MATRIX_DIR);

        let (event, _) = if let Some(recent_event) = events.last() {
            (
                format!(
                    "{} which is something about {}",
                    recent_event.0,
                    recent_event.1.clone()
                ),
                recent_event.1.clone(),
            )
        } else {
            let subject = self.get_random_topics();
            let event = format!("want to talk about {}", subject);
            (event, subject)
        };
        let l_event = if let Some(recent_event) = l_events.last() {
            format!(
                "{} which is something about {}",
                recent_event.0,
                recent_event.1.clone()
            )
        } else {
            let subject = self.get_random_topics();
            format!("want to talk about {}", subject)
        };
        // let context = format!("{} today {}, and {} today {}, they met at {}", name, event, l_name, l_event, place);

        curr_input.push(my_iss.get_str_iss()); // 0
        curr_input.push(l_iss.get_str_iss()); //1

        curr_input.push(name.to_owned()); //2
        curr_input.push(event); //init_persona's thoughts //3

        curr_input.push(l_name.to_owned()); //4
        curr_input.push(l_event); //target_persona's thoughts //5

        curr_input.push(get_now_date_str()); //6
        curr_input.push(my_iss.currently.clone()); //7
        curr_input.push(l_iss.currently.clone()); //8
        curr_input.push(name.to_owned()); //9
        curr_input.push(l_name.to_owned()); //10
        curr_input.push(place.to_owned()); //11
        curr_input.push(String::from(
            "they decided to talk with each other for a while",
        )); //12
        curr_input.push(name.to_owned()); //13
        curr_input.push(name.to_owned()); //14

        let prompt = generate_prompt(curr_input, &prompt_lib_file);
        log!("start_to_talk prompt: {}", prompt);

        let (bytes,): (Vec<u8>,) =
            ic_cdk::api::call::call(Principal::management_canister(), "raw_rand", ())
                .await
                .unwrap_or_default();
        let session = bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        let client = LLMSvcClient::default();
        let chat_request = QuestionRequest {
            question: String::default(),
            subject: String::default(),
            persona: prompt,
        };
        // println!("chat_request: {:?}", chat_request);
        match client.call_llm_proxy::<QuestionRequest, AnswerReply>("talk", chat_request).await {
            Ok(answer) => {
                let mut llm_answer = answer.answer.clone();
                let second_part = llm_answer.split(':').collect::<Vec<&str>>();
                if second_part.len() > 1 {
                    llm_answer = second_part[1].to_string();
                }

                return (session, Some(llm_answer));
            }
            Err(e) => {
                log!("StartTalk AI {} is something wrong: {}", self.id, e);
            }
        }

        (session, None)
    }
    async fn continue_to_talk_or_end(
        &self,
        name: &String,
        l_name: &String,
        events: &[(String, String)],
        l_events: &[(String, String)],
        place: &String,
        sn: i64,
        subject: &String,
        input: String,
        his: Vec<String>,
    ) -> Option<String> {
        let mut curr_input = vec![];
        let prompt_lib_file = format!("{}/template/plan/iterative_convo_v1.txt", AI_MATRIX_DIR);

        let event = if let Some(recent_event) = events.last() {
            format!(
                "{} which is something about {}",
                recent_event.0,
                recent_event.1.clone()
            )
        } else {
            let subject = self.get_random_topics();
            format!("want to talk about {}", subject)
        };
        let l_event = if let Some(recent_event) = l_events.last() {
            format!(
                "{} which is something about {}",
                recent_event.0,
                recent_event.1.clone()
            )
        } else {
            let subject = self.get_random_topics();
            format!("want to talk about {}", subject)
        };
        let context = format!(
            "{} today {}, and {} today {}, they met at {}",
            name, event, l_name, l_event, place
        );

        let my_persona = self.get_pato_iss().unwrap_or_default();
        curr_input.push(my_persona.get_str_iss()); //0
        curr_input.push(name.to_owned()); //1
        curr_input.push(String::default()); //retrieved memory //2 //todo: 获取summary分析name的情绪
        curr_input.push(context); //past context //3
        curr_input.push(place.to_owned()); //4
        curr_input.push(format!("they have talked at {} for a while.", place)); //5
        curr_input.push(name.to_owned()); //6
        curr_input.push(l_name.to_owned()); //7
        curr_input.push(his.join("\n")); //8
        curr_input.push(name.to_owned()); //9
        curr_input.push(l_name.to_owned()); //10
        curr_input.push(name.to_owned()); //11
        curr_input.push(name.to_owned()); //12
        curr_input.push(l_name.to_owned()); //13
        curr_input.push(input.clone()); //14

        let prompt = generate_prompt(curr_input, &prompt_lib_file);
        log!("continue_to_talk_or_end prompt: {}", prompt);

        self.talk_to_pato(sn, prompt, subject.to_owned(), input)
            .await
    }
    async fn get_name_events_subjects_for_pato(
        &self,
    ) -> (String, Vec<(String, String)>) {
        let client = LLMSvcClient::default();

        let name_request = EmptyRequest {};
        if let Ok(name_resp) = client.call_llm_proxy::<EmptyRequest, String>("request_pato_name", name_request).await {
            (name_resp, vec![])
        }
        else {
            (String::default(), vec![])
        }
    }

    fn get_pato_iss(&self) -> Option<Persona> {
        let pato_persona_path = format!("{}/{}/db/scratch.json", AI_PATO_DIR, self.id,);

        if let Ok(file) = File::open(pato_persona_path.clone()) {
            match serde_json::from_reader::<File, Persona>(file) {
                Ok(persona) => {
                    return Some(persona);
                }
                Err(e) => {
                    log!("read persona from file error: {}", e);
                }
            }
        } else {
            log!("error read {:?}", pato_persona_path);
        }

        None
    }
    fn get_other_pato_iss(&self, id: String) -> Option<Persona> {
        let pato_persona_path = format!("{}/{}/db/scratch.json", AI_PATO_DIR, id,);

        if let Ok(file) = File::open(pato_persona_path.clone()) {
            match serde_json::from_reader::<File, Persona>(file) {
                Ok(persona) => {
                    return Some(persona);
                }
                Err(e) => {
                    log!("read persona from file error: {}", e);
                }
            }
        } else {
            log!("error read {:?}", pato_persona_path);
        }

        None
    }
    fn get_pato_call(&self) -> Vec<String> {
        let callfilename = format!("{}/{}/db/call.txt", AI_PATO_DIR, self.id);
        let mut callee: Vec<String> = vec![];
        let mut lines: Vec<String> = vec![];
        if let Ok(file) = File::open(callfilename.clone()) {
            let reader = io::BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                if line.is_empty() {
                    continue;
                }
                lines.push(line);
            }
        }
        // log!("get_pato_call lines: {:?}", lines);

        if !lines.is_empty() {
            for line in lines.iter_mut() {
                let l = line.clone();
                let st = l.split('#').collect::<Vec<&str>>();
                if st.len() > 2 && st[2] == "waiting" {
                    *line = format!("{}#{}#done", st[0], st[1]);
                    callee.push(st[0].to_string() + "#" + st[1]);
                }
            }
        }
        // log!("get_pato_call callee: {:?}", lines);

        let mut set = HashSet::new();
        let mut result = Vec::new();
        for item in callee {
            if set.insert(item.clone()) {
                result.push(item.clone());
            }
        }
        callee = result;

        match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(callfilename)
        {
            Ok(mut file) => {
                let _ = writeln!(file, "{}", lines.join("\n"));
            }
            Err(e) => {
                log!("get_pato_call write back to file error: {}", e);
            }
        }

        callee
    }

    async fn kol_follower_conversation(
        &self,
        kol_id: String,
        follower_id: String,
        last_message: String,
        is_ask_kol: bool,
    ) -> String {
        let mut curr_input: Vec<String> = vec![];
        let mut reply = String::default();
        let kol_name = ask_pato_name(kol_id.clone()).await.unwrap_or_default();
        let my_name = ask_pato_name(follower_id.clone()).await.unwrap_or_default();
        let session_messages: Vec<ChatMessage> =
            get_kol_messages(follower_id.clone(), kol_id.clone());
        let raw_messages = session_messages
            .iter()
            .map(|m| my_name.clone() + ":" + &m.question + "\n" + &kol_name + ":" + &m.answer)
            .collect::<Vec<String>>();
        let summary_content = raw_messages.join("\n");
        let summary = get_kol_messages_summary(summary_content.clone())
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
        curr_input.push(last_message.clone()); //8
        curr_input.push(kol_name.clone()); //9
        curr_input.push(my_name.clone()); //10
        curr_input.push(kol_name.clone()); //11
        curr_input.push(kol_name.clone()); //12

        let prompt_lib_file = if is_ask_kol {
            format!("{}/template/plan/agent_chat_pro.txt", AI_MATRIX_DIR)
        } else {
            curr_input.push(my_name.clone()); //13
            curr_input.push(my_name.clone()); //14
            format!("{}/template/plan/agent_chat_follower.txt", AI_MATRIX_DIR)
        };
        let prompt = generate_prompt(curr_input, &prompt_lib_file);
        // log!("kol_follower_chat_prompt: {}", prompt);

        let db_path = if is_ask_kol {
            format!("{}/{}/db/knowledge_chromadb", AI_PATO_DIR, kol_id)
        } else {
            format!("{}/{}/db/knowledge_chromadb", AI_PATO_DIR, follower_id)
        };
        let knowledges = if is_ask_kol {
            let ks = ask_pato_knowledges(kol_id.clone()).await;
            let filtered_knowledges = ks
                .iter()
                .filter(|k| k.owner == kol_id)
                .map(|k| k.to_owned())
                .collect::<Vec<KnowLedgeInfo>>();
            filtered_knowledges
        } else {
            let ks = ask_pato_knowledges(follower_id.clone()).await;
            let filtered_knowledges = ks
                .iter()
                .filter(|k| k.owner == follower_id)
                .map(|k| k.to_owned())
                .collect::<Vec<KnowLedgeInfo>>();
            filtered_knowledges
        };

        let client = LLMSvcClient::default();
        let chat_request = BetterTalkRequest {
            question: last_message.clone(),
            prompt,
            collection_name: knowledges
                .iter()
                .map(|k| "sig".to_string() + &k.sig)
                .collect::<Vec<String>>(),
            db_path,
        };
        // println!("chat_request: {:?}", chat_request);
        match client.call_llm_proxy::<BetterTalkRequest, AnswerReply>("talk_better", chat_request).await {
            Ok(answer) => {
                reply = answer.answer.clone();
            }
            Err(e) => {
                log!("Call KOL AI is something wrong: {}", e);
            }
        }
        reply
    }
    async fn call_pato(&self, want_calls: Vec<String>) {
        let mut listeners: Vec<(String, i64)> = vec![];
        let mut callees: Vec<String> = vec![];
        let mut topics: HashMap<String, String> = HashMap::new();

        for call in want_calls {
            let info = call.split('#').collect::<Vec<&str>>();
            topics
                .entry(info[0].to_string())
                .and_modify(|t| *t = info[1].to_string())
                .or_insert(info[1].to_string());
            callees.push(info[0].to_string());
        }
        
        let callee = AGENT_CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());
        let (sn_resp,): (SnResponse,) =
            match call(callee, "request_pato_sn", (callees,)).await {
                Ok(response) => response,
                Err((code, msg)) => {
                    println!("become_kol失败: {}: {}", code as u8, msg);
                    (SnResponse::default(),)
                },
            };
    
        let resp = sn_resp.pato_sn_id;
        for pato in resp {
            listeners.push((pato.id.clone(), pato.sn.parse::<i64>().unwrap_or(0)));
        }

        log!("call listeners: {:?}", listeners);

        for l in listeners.iter() {
            if l.0 == self.id {
                continue;
            }
            let mut last_message = topics.get(&l.0).unwrap_or(&"".to_string()).clone();

            let kol_id = l.0.clone();
            for round in 0..MAX_PRO_ROUND {
                if last_message.is_empty() {
                    last_message = "你好".to_string() + ",我们继续聊吧";
                }
                if round % 2 == 0 {
                    let reply = self
                        .kol_follower_conversation(
                            kol_id.clone(),
                            self.id.clone(),
                            last_message.clone(),
                            true,
                        )
                        .await;
                    let message = ChatMessage {
                        created_at: get_now_secs() as i64,
                        session: String::default(),
                        place: "online".to_string(),
                        sender: self.id.clone(),
                        receiver: kol_id.clone(),
                        question: last_message.clone(),
                        answer: reply.clone(),
                        sender_role: "follower".to_string(),
                        subject: "kol".to_string(),
                    };
                    save_kol_chat_message(
                        self.id.clone(),
                        kol_id.clone(),
                        &mut vec![message],
                        true,
                    );
                    last_message = reply;
                } else {
                    last_message = self
                        .kol_follower_conversation(
                            kol_id.clone(),
                            self.id.clone(),
                            last_message.clone(),
                            false,
                        )
                        .await;
                }
            }

            publish_battery_actions(
                self.id.clone(),
                "和专家的聊天愉快地结束了。真是受益匪浅呢".to_string(),
            );
        }
    }

    pub async fn run_loop(&mut self) {
        log!("battery runner");
        let mut idle = 0;
        publish_battery_actions(self.id.clone(), "新的一天开始了!".to_string());
        let mut talk_records = HashMap::<String, i32>::new();
        let mut today = get_now_date_str();
        loop {
            publish_battery_actions(self.id.clone(), "在小镇里转一转!".to_string());
            if today != get_now_date_str() {
                talk_records.clear();
                today = get_now_date_str();
            }
            if self.sleep_mode {
                publish_battery_actions(self.id.clone(), "休息，休息一会儿!".to_string());
                idle += 1;
                if idle > 100 {
                    self.sleep_mode = false;
                    idle = 0;
                }
                continue;
            }

            let want_call = self.get_pato_call();
            if !want_call.is_empty() {
                publish_battery_actions(self.id.clone(), "联系专家聊一聊吧!".to_string());
                self.call_pato(want_call.clone()).await;
                continue;
            }

            if *talk_records.get(&get_now_date_str()).unwrap_or(&0) >= MAX_CHANCE_TALK_PER_DAY {
                publish_battery_actions(
                    self.id.clone(),
                    format!(
                        "今天{}次聊谈机会已经用完了，休息一下吧",
                        MAX_CHANCE_TALK_PER_DAY
                    ),
                );
                // log!("今天{}次聊谈机会已经用完了，休息一下吧！- {}", MAX_CHANCE_TALK_PER_DAY, idle);
                continue;
            }

            let actions = vec![
                ActionInfo{ place: "cafe".to_string(), action: "talk".to_string() },
                ActionInfo{ place: "mesuem".to_string(), action: "learn".to_string() }
            ];

            publish_battery_actions(self.id.clone(), "继续在小镇里转一转吧!".to_string());
            for action in actions {
                publish_battery_actions(
                    self.id.clone(),
                    format!("去{}看看吧", action.place.clone()),
                );
                publish_battery_actions(
                    self.id.clone(),
                    format!("交通真是头疼的事儿，终于到{}咯", action.place.clone()),
                );

                let listeners: Vec<(String, i64)> = vec![];
                // let listeners = self.pick_patos_to_talk(new_location).await;
                log!("met listeners: {:?}", listeners);

                let place = action.place.clone();
                let mut talked_listeners = 0;
                if !listeners.is_empty() {
                    publish_battery_actions(
                        self.id.clone(),
                        "遇到了很多有趣的人呢".to_string(),
                    );
                }
                for l in listeners {
                    if talked_listeners >= MAX_TALK_PER_PLACE {
                        break;
                    }
                    if l.0 == self.id {
                        continue;
                    }
                    let mut first_message = None;

                    let (my_name, my_events) = self
                        .get_name_events_subjects_for_pato()
                        .await;
                    let (mut listener_name, listener_events) = self
                        .get_name_events_subjects_for_pato()
                        .await;
                    let my_iss = self.get_pato_iss().unwrap_or_default();
                    let l_iss = self.get_other_pato_iss(l.0.clone()).unwrap_or_default();

                    if my_name == listener_name {
                        listener_name.push_str("#2");
                    }
                    let (want_talk, subject) = self
                        .decided_to_talk(
                            &my_name,
                            &listener_name,
                            my_events.clone(),
                            &listener_events,
                            &place.clone(),
                            &my_iss,
                            &l_iss,
                        )
                        .await;

                    publish_battery_actions(
                        self.id.clone(),
                        format!("{}现在好像有空的样子", listener_name.clone()),
                    );
                    if want_talk {
                        log!("decided to talk");
                        publish_battery_actions(
                            self.id.clone(),
                            format!("和{}聊一会儿吧", listener_name.clone()),
                        );
                        talked_listeners += 1;
                        // insert new on to talk_records or update existing one
                        let _ = talk_records
                            .entry(get_now_date_str())
                            .and_modify(|t| *t += 1)
                            .or_insert(1);
                        let mut session = String::new();
                        let mut will_talks = MAX_ROUND;
                        let mut round = 0;
                        while round <= will_talks {
                            publish_battery_actions(
                                l.0.clone(),
                                format!("{}正在和你聊天", my_name.clone()),
                            );
                            log!("round: {}", round);
                            let (mut his, last_sender) = get_chat_his_by_session(
                                session.to_string(),
                                self.id.clone(),
                                my_name.to_owned(),
                                listener_name.to_owned(),
                            );
                            if round == will_talks && !his.is_empty() {
                                // log!("last sender is : {}", last_sender);
                                if last_sender == self.id {
                                    let last_words = his.last().unwrap();
                                    // log!("last words is : {}", last_sender);
                                    match last_words.as_str() {
                                        // should use llm to determine whether sender want byebye
                                        "see you later" | "byebye" | "bye" | "goodbye" | "再见"
                                        | "see you" | "拜拜" => {
                                            break;
                                        }
                                        _ => {
                                            will_talks += 4;
                                            his = his[..his.len() - 1].to_vec();
                                        }
                                    }
                                } else {
                                    log!("not continue");
                                    break;
                                }
                            }
                            if round == 0 {
                                (session, first_message) = self
                                    .start_to_talk(
                                        &my_name,
                                        &listener_name,
                                        my_events.clone(),
                                        listener_events.clone(),
                                        &action.place,
                                        &my_iss,
                                        &l_iss,
                                    )
                                    .await;
                            } else if round % 2 == 1 {
                                let listener_input = first_message.clone();
                                let saved_input = first_message.clone();
                                let reply = self
                                    .continue_to_talk_or_end(
                                        &listener_name,
                                        &my_name,
                                        &listener_events,
                                        &my_events,
                                        &place,
                                        l.1,
                                        &subject,
                                        listener_input.unwrap_or_default(),
                                        his,
                                    )
                                    .await;
                                first_message = reply.clone();

                                self.save_chat_message(
                                    saved_input.unwrap_or_default(),
                                    reply.unwrap_or_default(),
                                    &session,
                                    place.clone(),
                                    self.id.clone(),
                                    l.0.clone(),
                                    subject.clone(),
                                );

                                if let Some(message) = first_message.clone() {
                                    if message.contains("bye")
                                        || message.contains("goodbye")
                                        || message.contains("再见")
                                    {
                                        break;
                                    }
                                }
                                publish_battery_actions(
                                    self.id.clone() + "/refresh",
                                    session.clone(),
                                );
                                if round == will_talks - 1 {
                                    publish_battery_actions(
                                        self.id.clone() + "/continue",
                                        session.clone(),
                                    );
                                    publish_battery_actions(
                                        self.id.clone(),
                                        format!(
                                            "聊了一会儿了，{}问你是否继续聊",
                                            listener_name.clone()
                                        ),
                                    );
                                }
                            } else {
                                let my_input = first_message.clone();
                                let reply = self
                                    .continue_to_talk_or_end(
                                        &my_name,
                                        &listener_name,
                                        &my_events,
                                        &listener_events,
                                        &place,
                                        self.sn,
                                        &subject,
                                        my_input.unwrap_or_default(),
                                        his,
                                    )
                                    .await;
                                first_message = reply.clone();
                            }
                            round += 1;
                        }
                        publish_battery_actions(
                            self.id.clone(),
                            format!("和{}聊了很久了，找别人聊聊吧", listener_name.clone()),
                        );
                    } else {
                        publish_battery_actions(
                            self.id.clone(),
                            "好像没有什么好聊的".to_string(),
                        );
                    }
                }
                publish_battery_actions(
                    self.id.clone(),
                    format!("{}没人，去别的地方看看吧", place),
                );
            }
        }
    }
}
