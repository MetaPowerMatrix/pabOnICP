use anyhow::{anyhow, Error};
use candid::Principal;
use ic_cdk::call;
use metapower_framework::dao::http::send_http_post_request;
use metapower_framework::{
    dao::sqlite::MetapowerSqlite3, log, read_and_writeback_json_file, ChatMessage, AI_PATO_DIR,
};
use metapower_framework::{
    AirdropRequest, ChangeBalanceRequest, EmptyRequest, FollowKolRequest, InjectHumanVoiceRequest, KolRegistrationRequest,
    KolRelations, MessageRequest, NamePros, NameRequest, NameResponse, PatoInfo, PatoInfoResponse,
    PopulationRegistrationRequest, ProfessionalsResponse,
    RoomInfo, RoomListResponse, SimpleRequest, SimpleResponse, TokenRequest, TokenResponse,
    TopicChatHisResponse, TopicChatRequest, UserActiveRequest,
};
use metapower_framework::{
    BATTERY_GRPC_REST_SERVER, BATTERY_GRPC_SERVER_PORT_START, OFFICIAL_PATO,
};
use sha1::Digest;
use std::collections::HashMap;

use crate::{BATTERY, BATTERY_CALLEE};

//TO-DO-IC-Storage
fn generate_prompt(curr_input: Vec<String>, prompt: String) -> String {
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

impl Default for MetaPowerMatrixAgentService {
    fn default() -> Self {
        MetaPowerMatrixAgentService::new()
    }
}

impl MetaPowerMatrixAgentService {
    pub fn new() -> Self {
        MetaPowerMatrixAgentService {
            id: ic_cdk::id().to_string(),
        }
    }

    pub fn get_pato_name(&self, id: String) -> Option<String> {
        let mut name = None;
        BATTERY_CALLEE.with(|v| {
            if let Some(json_str) = v.borrow().get(&id) {
                if let Ok(info) = serde_json::from_str::<PatoInfo>(&json_str) {
                    name = Some(info.name);
                }
            }
        });

        name
    }

    pub async fn request_airdrop(
        &self,
        request: AirdropRequest,
    ) -> std::result::Result<SimpleResponse, Error> {
        let balance_table = "CREATE TABLE IF NOT EXISTS balance (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            id TEXT NOT NULL,
            amount REAL NOT NULL
        )";

        let id = request.id.clone();
        let amount = request.amount;
        MetapowerSqlite3::new().create_table(balance_table.to_owned())?;

        let add_balance = "INSERT INTO balance (id, amount) VALUES (?1, ?2)";
        MetapowerSqlite3::new().insert_record(add_balance, &[&id, &amount])?;

        let response = SimpleResponse {
            success: true,
            message: String::default(),
        };

        Ok(response)
    }

    pub async fn request_population_registration(
        &self,
        request: PopulationRegistrationRequest,
    ) -> std::result::Result<SimpleResponse, Error> {
        let mut response = SimpleResponse {
            success: false,
            message: "-1".to_string(),
        };

        let id_table = "CREATE TABLE IF NOT EXISTS pato (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            id TEXT NOT NULL,
            name TEXT NOT NULL
        )";
        let id = request.id.clone();
        let name = request.name.clone();
        MetapowerSqlite3::new().create_table(id_table.to_owned())?;

        let prepare_id_table = "INSERT INTO pato (id, name) VALUES (?1, ?2)";
        match MetapowerSqlite3::new().insert_record(prepare_id_table, &[&id, &name]) {
            Ok(last_sn) => {
                response.message = last_sn.to_string();
                response.success = true;
            }
            Err(e) => {
                return Err(e);
            }
        }

        Ok(response)
    }

    pub async fn request_pato_info(&self, request: SimpleRequest) -> Result<PatoInfoResponse, Error> {
        let id = request.id.clone();
        let select_id_table = format!("select * from pato where id = \"{}\"", id.clone());
        println!("select_id_table sql: {}", select_id_table);

        // let avatar_link = format!("{}/avatar/{}/avatar.png", XFILES_SERVER, id);
        let callee = BATTERY.with(|callee| *callee.borrow().as_ref().unwrap());
        let (avatar_link,): (String,) = match call(
            callee,
            "avatar_of",
            (id.clone(),),
        )
        .await
        {
            Ok(response) => response,
            Err((code, msg)) => return Err(anyhow!("{}: {}", code as u8, msg)),
        };
        let (cover,): (String,) = match call(
            callee,
            "cover_of",
            (id.clone(),),
        )
        .await
        {
            Ok(response) => response,
            Err((code, msg)) => return Err(anyhow!("{}: {}", code as u8, msg)),
        };
        let (tags,): (String,) = match call(
            callee,
            "tags_of",
            (id.clone(),),
        )
        .await
        {
            Ok(response) => response,
            Err((code, msg)) => return Err(anyhow!("{}: {}", code as u8, msg)),
        };


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
                        balance: 0.0,
                        tags: tags.split(',').map(|t| t.to_string()).collect(),
                        avatar: avatar_link,
                        cover,
                    };
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

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
                return Err(e);
            }
        }

        Ok(pato_info)
    }

    pub fn request_pato_count(&self) -> u64 {
        let mut count = 0;

        BATTERY_CALLEE.with(|v| {
            count = v.borrow().len();
        });

        count
    }
    pub fn request_for_all_patos(&self) -> std::result::Result<Vec<PatoInfo>, Error> {
        let mut response = vec![];

        BATTERY_CALLEE.with(|v| {
            let callee = v.borrow();
            for (_, json_str) in callee.iter() {
                if let Ok(info) = serde_json::from_str::<PatoInfo>(&json_str) {
                    response.push(info);
                }
            }
        });

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
    ) -> std::result::Result<(), Error> {
        let amount = request.amount;
        let id = request.id.clone();

        let add_balance = format!(
            "UPDATE balance SET amount = amount - {} WHERE id = \"{}\"",
            amount, id
        );

        MetapowerSqlite3::new().update_table(add_balance.clone())?;

        Ok(())
    }

    pub async fn request_deposit(
        &self,
        request: ChangeBalanceRequest,
    ) -> std::result::Result<(), Error> {
        let amount = request.amount * 10.0;
        let id = request.id.clone();

        let add_balance = format!(
            "UPDATE balance SET amount = amount + {} WHERE id = \"{}\"",
            amount, id
        );

        MetapowerSqlite3::new().update_table(add_balance.clone())?;

        Ok(())
    }

    //TO-DO-IC-Storage
    pub fn request_inject_human_voice(
        &self,
        request: InjectHumanVoiceRequest,
    ) -> std::result::Result<(), Error> {
        let roles = request.roles.clone();
        let session = request.session.clone();

        for role in roles {
            let active_session_path = format!("{}/{}/live/{}", AI_PATO_DIR, role, session);
            let chat_message = ChatMessage {
                created_at: 0,
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

            read_and_writeback_json_file(&message_file, &mut vec![chat_message])?;
        }

        Ok(())
    }

    pub async fn request_pato_kol_token(
        &self,
        request: SimpleRequest,
    ) -> std::result::Result<SimpleResponse, Error> {
        let auth_table = "CREATE TABLE IF NOT EXISTS kol_auth_token (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            id TEXT NOT NULL,
            name TEXT NOT NULL,
            token TEXT NOT NULL
        )";
        let id = request.id.clone();
        let name = match self.get_pato_name(id.clone()) {
            Some(name) => name.to_string(),
            None => Err(anyhow!("pato not registered"))?,
        };

        let (bytes,): (Vec<u8>,) =
            ic_cdk::api::call::call(Principal::management_canister(), "raw_rand", ())
                .await
                .unwrap_or_default();
        let uuid = bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        let mut hasher = sha1::Sha1::new();
        hasher.update(uuid.as_bytes());
        let token = format!("KT{:x}", hasher.finalize());

        MetapowerSqlite3::new().create_table(auth_table.to_owned())?;

        let prepare_token_table =
            "INSERT INTO kol_auth_token (id, name, token) VALUES (?1, ?2, ?3)";

        MetapowerSqlite3::new().insert_record(prepare_token_table, &[&id, &name, &token])?;

        Ok(SimpleResponse {
            success: true,
            message: token,
        })
    }

    pub fn query_pato_kol_token(
        &self,
        request: TokenRequest,
    ) -> std::result::Result<TokenResponse, Error> {
        let select_pro_table = format!(
            "select * from kol_auth_token where id = \"{}\" order by registered_at",
            request.token.clone()
        );
        let mut token_info = TokenResponse::default();
        
        match MetapowerSqlite3::query_db(&select_pro_table, vec!["token", "name"]) {
            Ok(records) => {
                if !records.is_empty() {
                    for record in records {
                        token_info = TokenResponse {
                            id: request.token.clone(),
                            name: record.get("name").unwrap().to_string(),
                            token: record.get("token").unwrap().to_string(),
                        };
                    }
                    Ok(token_info)
                } else {
                    Err(anyhow!("KOL auth token not registered"))
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn query_pato_by_kol_token(
        &self,
        request: TokenRequest,
    ) -> std::result::Result<TokenResponse, Error> {
        let select_pro_table = format!(
            "select * from kol_auth_token where token = \"{}\" order by registered_at",
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
                            token: request.token.clone(),
                        };
                    }
                    Ok(token_info)
                } else {
                    Err(anyhow!("KOL auth token not registered"))
                }
            }
            Err(e) => Err(e),
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
    ) -> std::result::Result<(), Error> {
        let town = request.town.clone();
        let topic = request.topic.clone();
        let id = request.initial.clone();
        let amount = 10.0;
        let add_balance = format!(
            "UPDATE balance SET amount = amount - {} WHERE id = \"{}\"",
            amount, id
        );
        MetapowerSqlite3::new().update_table(add_balance.clone())?;

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
                            curr_input
                                .push(chat_messages.last().unwrap_or(&"".to_owned()).to_owned()); //5
                            curr_input.push(name.clone()); //6
                            curr_input.push(name.clone()); //7
                            curr_input.push(name.to_owned()); //8
                            last_talked_person = name.clone();

                            let prompt = generate_prompt(curr_input, "".to_string());
                            let req = serde_json::to_string(&MessageRequest {
                                message: String::default(),
                                subject: String::default(),
                                prompt,
                            })
                            .unwrap_or_default();
                            match send_http_post_request(
                                battery_address.to_string(),
                                battery_address.to_string(),
                                "agent_smith".to_string(),
                                req,
                            )
                            .await
                            {
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
                }
            }
            Err(e) => Err(e)?,
        }

        Ok(())
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

        Ok(TopicChatHisResponse {
            history: chat_messages,
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
    ) -> std::result::Result<(), Error> {
        let pro_table = "CREATE TABLE IF NOT EXISTS KOL (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            id TEXT NOT NULL,
            follower TEXT NOT NULL
        )";

        MetapowerSqlite3::new().create_table(pro_table.to_owned())?;

        let id = request.id.clone();
        let add_kol = "INSERT INTO KOL (id, follower) VALUES (?1, ?2)";
        MetapowerSqlite3::new().insert_record(add_kol, &[&id, &OFFICIAL_PATO.to_string()])?;

        Ok(())
    }

    pub fn request_kol_list(&self) -> std::result::Result<Vec<KolRelations>, Error> {
        let mut response: Vec<KolRelations> = vec![];

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
            let name = match self.get_pato_name(key.to_string()) {
                Some(name) => name.to_string(),
                None => "".to_string(),
            };

            response.push(KolRelations {
                id: key.clone(),
                follower: value.clone(),
                name,
            });
        }

        Ok(response)
    }

    pub fn request_add_kol_follower(
        &self,
        request: FollowKolRequest,
    ) -> std::result::Result<(), Error> {
        let follower = request.follower.clone();
        let id = request.id.clone();
        let add_kol = "INSERT INTO KOL (id, follower) VALUES (?1, ?2)";
        
        MetapowerSqlite3::new().insert_record(add_kol, &[&id, &follower])?;

        Ok(())
    }
}
