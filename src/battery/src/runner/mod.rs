#![allow(clippy::too_many_arguments)]

use candid::Principal;
use metapower_framework::dao::http::LLMSvcClient;
use metapower_framework::{
    dao::personality::Persona, ensure_directory_exists, get_event_subjects, get_now_date_str,
    get_now_secs, log, publish_battery_actions, read_and_writeback_json_file, ActionInfo, ChatMessage, EmptyRequest, QuestionRequest, AI_MATRIX_DIR,
    AI_PATO_DIR, BATTERY_GRPC_REST_SERVER, BATTERY_GRPC_SERVER_PORT_START,
};
use metapower_framework::{AnswerReply, MessageRequest};
use std::fs::File;
use std::collections::HashMap;

use crate::reverie::{
        generate_prompt,
        memory::get_chat_his_by_session,
    };

const MAX_ROUND: u64 = 8;
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
        match client
            .call_llm_proxy::<MessageRequest, AnswerReply>("talk", request)
            .await
        {
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
        match client
            .call_llm_proxy::<QuestionRequest, AnswerReply>("talk", chat_request)
            .await
        {
            Ok(answer) => {
                if answer.answer.contains("yes") || answer.answer.contains("Yes") {
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
        match client
            .call_llm_proxy::<QuestionRequest, AnswerReply>("talk", chat_request)
            .await
        {
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
        his: Vec<ChatMessage>,
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
        curr_input.push("".to_string()); //8
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
    async fn get_name_events_subjects_for_pato(&self) -> (String, Vec<(String, String)>) {
        let client = LLMSvcClient::default();

        let name_request = EmptyRequest {};
        if let Ok(name_resp) = client
            .call_llm_proxy::<EmptyRequest, String>("request_pato_name", name_request)
            .await
        {
            (name_resp, vec![])
        } else {
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
                ActionInfo {
                    place: "cafe".to_string(),
                    action: "talk".to_string(),
                },
                ActionInfo {
                    place: "mesuem".to_string(),
                    action: "learn".to_string(),
                },
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
                    publish_battery_actions(self.id.clone(), "遇到了很多有趣的人呢".to_string());
                }
                for l in listeners {
                    if talked_listeners >= MAX_TALK_PER_PLACE {
                        break;
                    }
                    if l.0 == self.id {
                        continue;
                    }
                    let mut first_message = None;

                    let (my_name, my_events) = self.get_name_events_subjects_for_pato().await;
                    let (mut listener_name, listener_events) =
                        self.get_name_events_subjects_for_pato().await;
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
                            log!("round: {}", round);
                            let his = get_chat_his_by_session(
                                self.id.clone(),
                                l.0.clone(),
                            ).await.unwrap_or_default();
                            if round == will_talks && !his.is_empty() {
                                let last_words = his.last().unwrap();
                                // log!("last words is : {}", last_sender);
                                match last_words.question.as_str() {
                                    // should use llm to determine whether sender want byebye
                                    "see you later" | "byebye" | "bye" | "goodbye" | "再见"
                                    | "see you" | "拜拜" => {
                                        break;
                                    }
                                    _ => {
                                        will_talks += 4;
                                    }
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
                        publish_battery_actions(self.id.clone(), "好像没有什么好聊的".to_string());
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
