use metapower_framework::dao::http::send_http_post_request;
use metapower_framework::{
    AirdropRequest, AllPatosResponse, ChangeBalanceRequest, EmptyRequest, FollowKolRequest, ImageGenRequest, InjectHumanVoiceRequest, KolListResponse, KolRegistrationRequest, KolRelations, MessageRequest, NamePros, NameRequest, NameResponse, PatoInfoResponse, PatoOfPro, PatoOfProResponse, PopulationRegistrationRequest, ProfessionalsResponse, RoomCreateRequest, RoomCreateResponse, RoomInfo, RoomListResponse, SimpleRequest, SimpleResponse, SnIdPaire, SnRequest, SnResponse, TokenRequest, TokenResponse, TopicChatHisResponse, TopicChatRequest, UserActiveRequest
};
use anyhow::{anyhow, Error};
use metapower_framework::dao::crawler::download_image;
use metapower_framework::{
    dao::sqlite::MetapowerSqlite3, get_now_secs, log, read_and_writeback_json_file, ChatMessage,
    AI_AGENT_DIR, AI_PATO_DIR,
};
use metapower_framework::{
    ensure_directory_exists, AI_MATRIX_DIR, BATTERY_GRPC_REST_SERVER, BATTERY_GRPC_SERVER_PORT_START, LLMCHAT_GRPC_REST_SERVER, OFFICIAL_PATO, TICK, XFILES_LOCAL_DIR, XFILES_SERVER
};
use sha1::Digest;
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::time::SystemTime;
use std::{env, io};
use std::{fs::File, io::Write};

//TO-DO-IC-Storage
fn generate_prompt(curr_input: Vec<String>, prompt_lib_file: &str) -> String {
    // Read the prompt file
    let file = File::open(prompt_lib_file).expect("Failed to open prompt file");
    let reader = BufReader::new(file);
    let prompt: String = reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n");

    // Replace the placeholders with the actual inputs
    let mut result = prompt;
    for (count, input) in curr_input.iter().enumerate() {
        result = result.replace(&format!("!<INPUT {}>!", count), input);
    }

    // Remove the comment block marker if present
    if result.contains("<commentblockmarker>###</commentblockmarker>") {
        result = result
            .split("<commentblockmarker>###</commentblockmarker>")
            .nth(1)
            .unwrap_or_default()
            .to_string();
    }

    // Return the final prompt
    result.trim().to_string()
}

#[derive(Debug)]
pub struct MetaPowerMatrixAgentService {
    id: String,
}

impl MetaPowerMatrixAgentService {
    pub fn new() -> Self {
        MetaPowerMatrixAgentService {
            id: ic_cdk::id().to_string(),
        }
    }
    //TO-DO-IC-Storage
    fn get_pato_name(&self, id: String) -> Option<String> {
        let callfilename = format!("{}/{}/db/name.txt", AI_PATO_DIR, id);
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
        println!("get_pato_name lines: {:?}", lines);

        lines.last().cloned()
    }
    //TO-DO-IC-Storage
    fn generate_prompt(&self, curr_input: Vec<String>, prompt_lib_file: &str) -> String {
        // Read the prompt file
        let file = File::open(prompt_lib_file).expect("Failed to open prompt file");
        let reader = BufReader::new(file);
        let prompt: String = reader
            .lines()
            .map(|line| line.unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n");

        // Replace the placeholders with the actual inputs
        let mut result = prompt;
        for (count, input) in curr_input.iter().enumerate() {
            result = result.replace(&format!("!<INPUT {}>!", count), input);
        }

        // Remove the comment block marker if present
        if result.contains("<commentblockmarker>###</commentblockmarker>") {
            result = result
                .split("<commentblockmarker>###</commentblockmarker>")
                .nth(1)
                .unwrap_or_default()
                .to_string();
        }

        // Return the final prompt
        result.trim().to_string()
    }

    pub async fn request_airdrop(
        &self,
        request: AirdropRequest,
    ) -> std::result::Result<SimpleResponse, Error> {
        let mut success = false;
        let balance_table = "CREATE TABLE IF NOT EXISTS balance (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            id TEXT NOT NULL,
            amount REAL NOT NULL
        )";

        let id = request.id.clone();
        let amount = request.amount;
        match MetapowerSqlite3::new().create_table(balance_table.to_owned()) {
            Ok(_) => {
                success = true;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let add_balance = "INSERT INTO balance (id, amount) VALUES (?1, ?2)";

        match MetapowerSqlite3::new().insert_record(add_balance, &[&id, &amount]) {
            Ok(_) => {
                success = true;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let response = SimpleResponse {
            success,
            message: String::default(),
        };

        Ok(response)
    }

    pub async fn request_population_registration(
        &self,
        request: PopulationRegistrationRequest,
    ) -> std::result::Result<SimpleResponse, Error> {
        let mut success = false;
        let id_table = "CREATE TABLE IF NOT EXISTS pato (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            id TEXT NOT NULL,
            name TEXT NOT NULL
        )";
        let id = request.id.clone();
        let name = request.name.clone();
        match MetapowerSqlite3::new().create_table(id_table.to_owned()) {
            Ok(_) => {
                success = true;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let prepare_id_table = "INSERT INTO pato (id, name) VALUES (?1, ?2)";
        let mut sn = 0;
        match MetapowerSqlite3::new().insert_record(prepare_id_table, &[&id, &name]) {
            Ok(last_sn) => {
                sn = last_sn;
                success = true;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let response = SimpleResponse {
            success,
            message: sn.to_string(),
        };

        Ok(response)
    }

    pub fn request_pato_info(
        &self,
        request: SimpleRequest,
    ) -> std::result::Result<PatoInfoResponse, Error> {
        let id = request.id.clone();
        let select_id_table = format!("select * from pato where id = \"{}\"", id.clone());
        println!("select_id_table sql: {}", select_id_table);

        let mut avatar_link = format!("{}/avatar/{}/avatar.png", XFILES_SERVER, id);
        let avatar = format!("{}/avatar/{}/avatar.png", XFILES_LOCAL_DIR, id);
        if !Path::new(&avatar).exists() {
            avatar_link = "".to_string();
        }
        let mut tags: Vec<String> = vec![];
        if let Ok(mut file) = OpenOptions::new()
            .read(true)
            .open(format!("{}/{}/db/tags.json", AI_PATO_DIR, id))
        {
            let mut tags_json_str = String::new();
            file.read_to_string(&mut tags_json_str)?;
            tags = serde_json::from_str::<Vec<String>>(&tags_json_str).unwrap_or_default();
        }

        let mut pato_info: PatoInfoResponse = PatoInfoResponse::default();
        match MetapowerSqlite3::query_db(
            &select_id_table,
            vec!["id", "name", "registered_at", "sn"],
        ) {
            Ok(records) => {
                if !records.is_empty() {
                    let record = records.first().unwrap();
                    let name = record.get("name").unwrap().to_string();
                    let registered_datetime = record.get("registered_at").unwrap().to_string();
                    let sn = record.get("sn").unwrap().parse::<i64>().unwrap();
                    pato_info = PatoInfoResponse {
                        id: id.clone(),
                        name,
                        sn,
                        registered_datetime,
                        professionals: vec![],
                        balance: 0.0,
                        tags,
                        avatar: avatar_link,
                    };
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("pato not found"));
            }
        }

        let mut professionals = vec![];
        let select_pro_table = format!("select subject from professionals where id = \"{}\"", id);
        match MetapowerSqlite3::query_db(&select_pro_table, vec!["subject"]) {
            Ok(records) => {
                if !records.is_empty() {
                    for record in records {
                        let subject = record.get("subject").unwrap().to_string();
                        professionals.push(subject);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("professionals subject not found"));
            }
        }

        let mut set = HashSet::new();
        let mut result = Vec::new();
        for item in professionals {
            if set.insert(item.clone()) {
                result.push(item.clone());
            }
        }
        pato_info.professionals = result;

        let select_balance_table = format!("select amount from balance where id = \"{}\"", id);
        match MetapowerSqlite3::query_db(&select_balance_table, vec!["amount"]) {
            Ok(records) => {
                if !records.is_empty() {
                    for record in records {
                        pato_info.balance = record
                            .get("amount")
                            .unwrap()
                            .parse::<f32>()
                            .unwrap_or_default();
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("professionals subject not found"));
            }
        }

        Ok(pato_info)
    }

    pub fn request_professionals(
        &self,
        _request: EmptyRequest,
    ) -> std::result::Result<PatoOfProResponse, Error> {
        let mut pros: Vec<PatoOfPro> = vec![];
        let select_pro_table = "select id, subject from professionals";
        let mut pato_pro: HashMap<String, Vec<String>> = HashMap::new();

        match MetapowerSqlite3::query_db(select_pro_table, vec!["id", "subject"]) {
            Ok(records) => {
                if !records.is_empty() {
                    for record in records {
                        let id = record.get("id").unwrap().to_string();
                        let subject = record.get("subject").unwrap().to_string();
                        pato_pro
                            .entry(id)
                            .and_modify(|p| p.push(subject.clone()))
                            .or_insert(vec![]);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("professionals not found"));
            }
        }
        for pro in pato_pro.iter() {
            let mut name = String::default();
            let name_file = format!("{}/{}/db/name.txt", AI_PATO_DIR, pro.0);
            if let Ok(file) = File::open(name_file) {
                let reader = BufReader::new(file);
                if let Some(Ok(last_line)) = reader.lines().last() {
                    name = last_line;
                }
            }
            let mut set = HashSet::new();
            let mut result = Vec::new();
            for item in pro.1 {
                if !item.is_empty() && set.insert(item.clone()) {
                    result.push(item.clone());
                }
            }

            let i = PatoOfPro {
                id: pro.0.clone(),
                subjects: result,
                name,
            };
            pros.push(i);
            pros.sort_by(|a, b| b.subjects.len().cmp(&a.subjects.len()));
        }
        let hot_pros = if pros.len() > 10 {
            let _ = pros.split_off(10);
            pros
        } else {
            pros
        };

        let response = PatoOfProResponse { patos: hot_pros };
        Ok(response)
    }

    pub fn request_pato_sn(&self, request: SnRequest) -> std::result::Result<SnResponse, Error> {
        let mut response = SnResponse { pato_sn_id: vec![] };
        let ids = request.id.clone();
        let select_id_table = format!(
            "select id, sn from pato where id in (\"{}\")",
            ids.join("\",\"")
        );
        // println!("select_id_table sql: {}", select_id_table);

        match MetapowerSqlite3::query_db(&select_id_table, vec!["id", "sn"]) {
            Ok(records) => {
                for record in records {
                    let id = record.get("id").unwrap().to_string();
                    let sn = record
                        .get("sn")
                        .unwrap()
                        .parse::<i64>()
                        .unwrap()
                        .to_string();
                    let data = SnIdPaire { id, sn };
                    response.pato_sn_id.push(data);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("pato not found"));
            }
        }

        Ok(response)
    }
    pub fn request_for_all_patos(
        &self,
        _request: EmptyRequest,
    ) -> std::result::Result<AllPatosResponse, Error> {
        let mut response = AllPatosResponse { pato_sn_id: vec![] };

        let select_pro_table = "select sn, id from pato";

        match MetapowerSqlite3::query_db(select_pro_table, vec!["id", "sn"]) {
            Ok(records) => {
                if !records.is_empty() {
                    for record in records {
                        let id = record.get("id").unwrap().to_string();
                        let sn = record.get("sn").unwrap().to_string();
                        let pair = SnIdPaire { id, sn };
                        response.pato_sn_id.push(pair);
                    }
                    return Ok(response);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("none of patos"));
            }
        }

        Ok(response)
    }

    pub fn request_add_balance(
        &self,
        request: ChangeBalanceRequest,
    ) -> std::result::Result<SimpleResponse, Error> {
        let amount = request.amount;
        let id = request.id.clone();
        let mut success = false;

        let add_balance = format!(
            "UPDATE balance SET amount = amount + {} WHERE id = \"{}\"",
            amount, id
        );

        match MetapowerSqlite3::new().update_table(add_balance.clone()) {
            Ok(_) => {
                success = true;
                log!("update {} success", add_balance);
            }
            Err(e) => {
                log!("Error: {}", e);
            }
        }

        let response = SimpleResponse {
            success,
            message: String::default(),
        };

        Ok(response)
    }

    pub async fn request_minus_balance(
        &self,
        request: ChangeBalanceRequest,
    ) -> std::result::Result<SimpleResponse, Error> {
        let amount = request.amount;
        let id = request.id.clone();
        let mut success = false;

        let add_balance = format!(
            "UPDATE balance SET amount = amount - {} WHERE id = \"{}\"",
            amount, id
        );

        match MetapowerSqlite3::new().update_table(add_balance.clone()) {
            Ok(_) => {
                success = true;
                log!("update {} success", add_balance);
            }
            Err(e) => {
                log!("Error: {}", e);
            }
        }

        let response = SimpleResponse {
            success,
            message: String::default(),
        };

        Ok(response)
    }

    pub async fn request_deposit(
        &self,
        request: ChangeBalanceRequest,
    ) -> std::result::Result<SimpleResponse, Error> {
        let amount = request.amount * 10.0;
        let id = request.id.clone();
        let mut success = false;

        let add_balance = format!(
            "UPDATE balance SET amount = amount + {} WHERE id = \"{}\"",
            amount, id
        );

        match MetapowerSqlite3::new().update_table(add_balance.clone()) {
            Ok(_) => {
                success = true;
                log!("update {} success", add_balance);
            }
            Err(e) => {
                log!("Error: {}", e);
            }
        }

        let response = SimpleResponse {
            success,
            message: String::default(),
        };

        Ok(response)
    }

    //TO-DO-IC-Storage
    pub fn request_inject_human_voice(
        &self,
        request: InjectHumanVoiceRequest,
    ) -> std::result::Result<EmptyRequest, Error> {
        let roles = request.roles.clone();
        let session = request.session.clone();

        for role in roles {
            let active_session_path = format!("{}/{}/live/{}", AI_PATO_DIR, role, session);
            let chat_message = ChatMessage {
                created_at: get_now_secs() as i64,
                session: session.clone(),
                place: "live_room".to_string(),
                sender: request.id.clone(),
                receiver: "all".to_string(),
                question: request.message.clone(),
                answer: "".to_string(),
                sender_role: "human".to_string(),
                subject: "".to_string(),
            };
            let message_file = active_session_path.clone() + "/message.json";
            log!("write messages {:?} to file {}", chat_message, message_file);
            if let Err(e) = read_and_writeback_json_file(&message_file, &mut vec![chat_message]) {
                log!("read_and_writeback_json_file error for receiver: {}", e);
            }
        }
        Ok(EmptyRequest {})
    }

    pub async fn request_pato_auth_token(
        &self,
        request: SimpleRequest,
    ) -> std::result::Result<SimpleResponse, Error> {
        let auth_table = "CREATE TABLE IF NOT EXISTS auth_token (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            id TEXT NOT NULL,
            name TEXT NOT NULL,
            token TEXT NOT NULL
        )";
        let id = request.id.clone();
        let name = self.get_pato_name(id.clone()).unwrap_or_default();
        let uuid = uuid::Uuid::new_v4().to_string();
        let mut hasher = sha1::Sha1::new();
        hasher.update(uuid.as_bytes());
        let token = format!("{:x}", hasher.finalize());

        match MetapowerSqlite3::new().create_table(auth_table.to_owned()) {
            Ok(_) => {
                log!("create auth table success");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let prepare_token_table = "INSERT INTO auth_token (id, name, token) VALUES (?1, ?2, ?3)";

        match MetapowerSqlite3::new().insert_record(prepare_token_table, &[&id, &name, &token]) {
            Ok(_) => {
                log!("insert live table success");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        Ok(SimpleResponse {
            success: true,
            message: token,
        })
    }

    pub fn query_pato_auth_token(
        &self,
        request: TokenRequest,
    ) -> std::result::Result<TokenResponse, Error> {
        let select_pro_table = format!(
            "select * from auth_token where token = \"{}\" order by registered_at",
            request.token
        );
        let mut token_info = TokenResponse::default();

        match MetapowerSqlite3::query_db(&select_pro_table, vec!["id", "name"]) {
            Ok(records) => {
                if !records.is_empty() {
                    for record in records {
                        token_info = TokenResponse {
                            id: record.get("id").unwrap().to_string(),
                            name: record.get("name").unwrap().to_string(),
                        };
                    }
                    return Ok(token_info);
                } else {
                    return Err(anyhow!("professionals subject not found"));
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("professionals subject not found"));
            }
        }
    }

    pub async fn request_pato_knowledges(
        &self,
        request: SimpleRequest,
    ) -> std::result::Result<ProfessionalsResponse, Error> {
        let mut response = ProfessionalsResponse {
            professionals: vec![],
        };

        let select_pro_table = format!(
            "select knowledge from professionals where id = \"{}\"",
            request.id
        );

        match MetapowerSqlite3::query_db(&select_pro_table, vec!["knowledge"]) {
            Ok(records) => {
                if !records.is_empty() {
                    for record in records {
                        let subject = record.get("knowledge").unwrap().to_string();
                        response.professionals.push(subject);
                    }
                    return Ok(response);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("professionals subject not found"));
            }
        }

        Ok(response)
    }

    pub async fn request_topic_chat(
        &self,
        request: TopicChatRequest,
    ) -> std::result::Result<EmptyRequest, Error> {
        let town = request.town.clone();
        let topic = request.topic.clone();
        let id = request.initial.clone();
        let amount = 10.0;
        let agent_id = self.id.clone();
        let prompt_lib_file = format!("{}/template/plan/iterative_convo_topic.txt", AI_MATRIX_DIR);
        let add_balance = format!(
            "UPDATE balance SET amount = amount - {} WHERE id = \"{}\"",
            amount, id
        );
        match MetapowerSqlite3::new().update_table(add_balance.clone()) {
            Ok(_) => {
                log!("update {} success", add_balance);
            }
            Err(e) => {
                log!("Error: {}", e);
            }
        }

            let select_pro_table = "select sn, id, name from pato";
            match MetapowerSqlite3::query_db(select_pro_table, vec!["id", "sn", "name"]) {
                Ok(records) => {
                    if !records.is_empty() {
                        let mut chat_messages: Vec<String> = vec![];
                        let mut last_talked_person: String = String::default();
                        for record in records {
                            let mut curr_input: Vec<String> = vec![];
                            let name = record.get("name").unwrap().to_string();
                            log!("topic talk to {}", name);
                            // let id = record.get("id").unwrap().to_string();
                            let sn = record
                                .get("sn")
                                .unwrap()
                                .to_string()
                                .parse::<i64>()
                                .unwrap_or(-1);
                            if sn > 0 {
                                if last_talked_person.is_empty() {
                                    last_talked_person = name.clone();
                                }
                                let battery_address = format!(
                                    "{}:{}",
                                    BATTERY_GRPC_REST_SERVER,
                                    sn + BATTERY_GRPC_SERVER_PORT_START
                                );
                                curr_input.push(town.clone()); //0
                                curr_input.push(town.clone()); //1
                                curr_input.push(chat_messages.join("\n")); //2
                                curr_input.push(topic.clone()); //3
                                curr_input.push(last_talked_person.clone()); //4
                                curr_input.push(
                                    chat_messages.last().unwrap_or(&"".to_owned()).to_owned(),
                                ); //5
                                curr_input.push(name.clone()); //6
                                curr_input.push(name.clone()); //7
                                curr_input.push(name.to_owned()); //8
                                last_talked_person = name.clone();

                                let prompt = generate_prompt(curr_input, &prompt_lib_file);
                                let req = serde_json::to_string(&MessageRequest {
                                    message: String::default(),
                                    subject: String::default(),
                                    prompt,
                                }).unwrap_or_default();
                                match send_http_post_request(battery_address.to_string(), battery_address.to_string(), "agent_smith".to_string(), req).await{
                                    Ok(resp) => {
                                        chat_messages.push(name + ":" + &resp.clone());
                                    }
                                    Err(e) => {
                                        println!("topic talk error: {}", e);
                                    }
                                }
                            }
                        }
                        let mut hasher = sha1::Sha1::new();
                        hasher.update(&topic);
                        hasher.update(&town);
                        let saved_file_name = format!("{:x}", hasher.finalize());
                        let saved_topic_message_file = format!(
                            "{}/{}/db/topic/{}.json",
                            AI_AGENT_DIR, agent_id, saved_file_name
                        );
                        if let Ok(mut sig_file) = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .truncate(true)
                            .open(saved_topic_message_file)
                        {
                            let _ = sig_file.write_all(
                                serde_json::to_string(&chat_messages)
                                    .unwrap_or_default()
                                    .as_bytes(),
                            );
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }

        Ok(EmptyRequest {})
    }

    pub fn request_topic_chat_history(
        &self,
        request: TopicChatRequest,
    ) -> std::result::Result<TopicChatHisResponse, Error> {
        let town = request.town.clone();
        let topic = request.topic.clone();

        let mut chat_messages: Vec<String> = vec![];
        let mut hasher = sha1::Sha1::new();
        hasher.update(&topic);
        hasher.update(&town);
        let saved_file_name = format!("{:x}", hasher.finalize());
        let saved_topic_message_file = format!(
            "{}/{}/db/topic/{}.json",
            AI_AGENT_DIR, self.id, saved_file_name
        );
        if let Ok(mut sig_file) = OpenOptions::new().read(true).open(saved_topic_message_file) {
            let mut buf = String::new();
            let _ = sig_file.read_to_string(&mut buf);
            chat_messages = serde_json::from_str(&buf).unwrap_or_default();
        }
        Ok(TopicChatHisResponse {
            history: chat_messages,
        })
    }

    pub async fn request_create_room(
        &self,
        request: RoomCreateRequest,
    ) -> std::result::Result<RoomCreateResponse, Error> {
        let game_room_table = "CREATE TABLE IF NOT EXISTS game_room (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            owner TEXT NOT NULL,
            room_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            cover TEXT NOT NULL,
            town TEXT NOT NULL
        )";
        let owner = request.owner.clone();
        let title = request.title.clone();
        let town = request.town.clone();
        let description = request.description.clone();
        let room_id = uuid::Uuid::new_v4().to_string();
        let image_file_name = uuid::Uuid::new_v4().to_string();
        let mut xfiles_link = String::default();

        let prompt = format!(
            "draw a picture according to the description below: {}",
            description
        );
        let image_request = serde_json::to_string(&ImageGenRequest { prompt })?;
        match send_http_post_request(LLMCHAT_GRPC_REST_SERVER.to_string(), LLMCHAT_GRPC_REST_SERVER.to_string(), "agent_smith".to_string(), image_request).await{
            Ok(answer) => {
                let image_url = answer;
                let saved_local_file = format!("{}/game/{}", XFILES_LOCAL_DIR, image_file_name);
                xfiles_link = format!("{}/game/{}", XFILES_SERVER, image_file_name);
                match download_image(&image_url, &saved_local_file).await {
                    Ok(_) => {
                        let game_room_path =
                            format!("{}/{}/db/game_room/{}", AI_PATO_DIR, owner, room_id);
                        let _ = ensure_directory_exists(&game_room_path);
                        if let Ok(mut file) = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(format!("{}/scene.txt", game_room_path))
                        {
                            writeln!(file, "{}", xfiles_link)?;
                        }
                    }
                    Err(e) => {
                        log!("download image error: {}", e);
                    }
                }
            }
            Err(e) => {
                log!("image_request AI is something wrong: {}", e);
            }
        }

        match MetapowerSqlite3::new().create_table(game_room_table.to_owned()) {
            Ok(_) => {
                log!("create game room table success");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let new_game_room = "INSERT INTO game_room (owner, room_id, title, description, cover, town) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";

        match MetapowerSqlite3::new().insert_record(
            new_game_room,
            &[&owner, &room_id, &title, &description, &xfiles_link, &town],
        ) {
            Ok(_) => {
                log!("create game room success");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let game_room_path = format!(
            "{}/{}/{}/db/game_room/{}",
            AI_AGENT_DIR,
            self.id.clone(),
            owner,
            room_id
        );
        let _ = ensure_directory_exists(&game_room_path);
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("{}/name.txt", game_room_path))
        {
            let _ = file.write_all(title.as_bytes());
        }

        Ok(RoomCreateResponse {
            room_id,
            cover: xfiles_link,
        })
    }

    pub fn request_game_room_list(
        &self,
        request: SimpleRequest,
    ) -> std::result::Result<RoomListResponse, Error> {
        let mut response = RoomListResponse { rooms: vec![] };

        let select_room_table = format!(
            "select owner, room_id, title, description, cover from game_room where town = \"{}\"",
            request.id.clone()
        );

        match MetapowerSqlite3::query_db(
            &select_room_table,
            vec!["owner", "room_id", "title", "description", "cover"],
        ) {
            Ok(records) => {
                if !records.is_empty() {
                    for record in records {
                        let owner = record.get("owner").unwrap().to_string();
                        let room_id = record.get("room_id").unwrap().to_string();
                        let title = record.get("title").unwrap().to_string();
                        let description = record.get("description").unwrap().to_string();
                        let cover = record.get("cover").unwrap().to_string();
                        let room = RoomInfo {
                            owner,
                            title,
                            description,
                            town: request.id.clone(),
                            room_id,
                            cover,
                        };
                        response.rooms.push(room);
                    }
                    return Ok(response);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("game rooms list error"));
            }
        }

        Ok(response)
    }

    pub fn log_user_active(
        &self,
        request: UserActiveRequest,
    ) -> std::result::Result<EmptyRequest, Error> {
        let game_room_table = "CREATE TABLE IF NOT EXISTS user_log (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            id TEXT NOT NULL,
            page TEXT NOT NULL,
            action TEXT NOT NULL
        )";
        let id = request.id.clone();
        let page = request.page.clone();
        let action = request.action.clone();

        match MetapowerSqlite3::new().create_table(game_room_table.to_owned()) {
            Ok(_) => {
                log!("create user log table success");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let new_game_room = "INSERT INTO user_log (id, page, action) VALUES (?1, ?2, ?3)";

        match MetapowerSqlite3::new().insert_record(new_game_room, &[&id, &page, &action]) {
            Ok(_) => {
                log!("log user activity success");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        Ok(EmptyRequest {})
    }

    pub async fn request_pato_by_ids(
        &self,
        request: NameRequest,
    ) -> std::result::Result<NameResponse, Error> {
        let mut response = NameResponse { name_pros: vec![] };
        let ids = request.id.clone();
        let select_name_table = format!(
            "select id, name from pato where id in (\"{}\")",
            ids.join("\",\"")
        );
        // println!("select_name_table sql: {}", select_name_table);

        match MetapowerSqlite3::query_db(&select_name_table, vec!["id", "name"]) {
            Ok(records) => {
                for record in records {
                    let id = record.get("id").unwrap().to_string();
                    let name = record.get("name").unwrap().to_string();
                    let data = NamePros {
                        id,
                        name,
                        pros: vec![],
                    };
                    response.name_pros.push(data);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("pato not found"));
            }
        }

        Ok(response)
    }

    pub async fn request_pato_by_name(
        &self,
        request: SimpleRequest,
    ) -> std::result::Result<NameResponse, Error> {
        let name = request.id.clone();
        let select_id_table = format!("select * from pato where name = \"{}\"", name.clone());

        let mut pato_info: NameResponse = NameResponse::default();
        match MetapowerSqlite3::query_db(&select_id_table, vec!["id", "sn"]) {
            Ok(records) => {
                if !records.is_empty() {
                    for record in records {
                        let id = record.get("id").unwrap().to_string();
                        let sn = record.get("sn").unwrap().to_string();
                        let pato = NamePros {
                            id: id.clone(),
                            name: name.clone(),
                            pros: vec![sn],
                        };
                        pato_info.name_pros.push(pato);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("pato not found"));
            }
        }

        Ok(pato_info)
    }

    pub fn request_kol_registration(
        &self,
        request: KolRegistrationRequest,
    ) -> std::result::Result<EmptyRequest, Error> {
        let pro_table = "CREATE TABLE IF NOT EXISTS KOL (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            id TEXT NOT NULL,
            follower TEXT NOT NULL
        )";

        if let Err(e) = MetapowerSqlite3::new().create_table(pro_table.to_owned()) {
            println!("Error: {}", e);
        }

        let id = request.id.clone();
        let add_kol = "INSERT INTO KOL (id, follower) VALUES (?1, ?2)";
        if let Err(e) =
            MetapowerSqlite3::new().insert_record(add_kol, &[&id, &OFFICIAL_PATO.to_string()])
        {
            println!("Error: {}", e);
        }

        let response = EmptyRequest {};

        Ok(response)
    }

    pub fn request_kol_list(
        &self,
        _request: EmptyRequest,
    ) -> std::result::Result<KolListResponse, Error> {
        let mut response = KolListResponse { relations: vec![] };

        let select_room_table = "select id, follower from KOL";

        let mut relations: HashMap<String, Vec<String>> = HashMap::new();
        match MetapowerSqlite3::query_db(select_room_table, vec!["id", "follower"]) {
            Ok(records) => {
                if !records.is_empty() {
                    for record in records {
                        let kol = record.get("id").unwrap().to_string();
                        let follower = record.get("follower").unwrap().to_string();
                        relations
                            .entry(kol)
                            .and_modify(|f| f.push(follower.clone()))
                            .or_insert(vec![follower.clone()]);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("request_kol_list error"));
            }
        }
        // log!("relations: {:?}", relations);
        for (key, value) in relations.iter() {
            let name = self.get_pato_name(key.to_string()).unwrap_or_default();
            response.relations.push(KolRelations {
                id: key.clone(),
                follower: value.clone(),
                name,
            });
        }
        // log!("response: {:?}", response.relations);
        Ok(response)
    }

    pub fn request_add_kol_follower(
        &self,
        request: FollowKolRequest,
    ) -> std::result::Result<EmptyRequest, Error> {
        let follower = request.follower.clone();
        let id = request.id.clone();
        let add_kol = "INSERT INTO KOL (id, follower) VALUES (?1, ?2)";
        if let Err(e) = MetapowerSqlite3::new().insert_record(add_kol, &[&id, &follower]) {
            println!("Error: {}", e);
        }

        Ok(EmptyRequest {})
    }

    pub async fn request_marriage_registration(
        &self,
        request: KolRegistrationRequest,
    ) -> std::result::Result<EmptyRequest, Error> {
        let pro_table = "CREATE TABLE IF NOT EXISTS Marriage (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            id TEXT NOT NULL,
            follower TEXT NOT NULL
        )";

        if let Err(e) = MetapowerSqlite3::new().create_table(pro_table.to_owned()) {
            println!("Error: {}", e);
        }

        let id = request.id.clone();
        let add_kol = "INSERT INTO Marriage (id, follower) VALUES (?1, ?2)";
        if let Err(e) =
            MetapowerSqlite3::new().insert_record(add_kol, &[&id, &OFFICIAL_PATO.to_string()])
        {
            println!("Error: {}", e);
        }

        Ok(EmptyRequest {})
    }

    pub async fn request_add_marriage_follower(
        &self,
        request: FollowKolRequest,
    ) -> std::result::Result<EmptyRequest, Error> {
        let follower = request.follower.clone();
        let id = request.id.clone();
        let add_kol = "INSERT INTO Marriage (id, follower) VALUES (?1, ?2)";
        if let Err(e) = MetapowerSqlite3::new().insert_record(add_kol, &[&id, &follower]) {
            println!("Error: {}", e);
        }

        Ok(EmptyRequest {})
    }

    pub async fn request_marriage_list(
        &self,
        _request: EmptyRequest,
    ) -> std::result::Result<KolListResponse, Error> {
        let mut response = KolListResponse { relations: vec![] };

        let select_room_table = "select id, follower from Marriage";

        let mut relations: HashMap<String, Vec<String>> = HashMap::new();
        match MetapowerSqlite3::query_db(select_room_table, vec!["id", "follower"]) {
            Ok(records) => {
                if !records.is_empty() {
                    for record in records {
                        let kol = record.get("id").unwrap().to_string();
                        let follower = record.get("follower").unwrap().to_string();
                        relations
                            .entry(kol)
                            .and_modify(|f| f.push(follower.clone()))
                            .or_insert(vec![follower.clone()]);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(anyhow!("request_kol_list error"));
            }
        }
        // log!("relations: {:?}", relations);
        for (key, value) in relations.iter() {
            let name = self.get_pato_name(key.to_string()).unwrap_or_default();
            response.relations.push(KolRelations {
                id: key.clone(),
                follower: value.clone(),
                name,
            });
        }
        // log!("response: {:?}", response.relations);
        Ok(response)
    }
}
