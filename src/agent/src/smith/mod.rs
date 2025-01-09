use anyhow::{anyhow, Error};
use candid::Principal;
use ic_cdk::call;
use metapower_framework::{
    dao::sqlite::MetapowerSqlite3, log,
};
use metapower_framework::{
    AirdropRequest, ChangeBalanceRequest, EmptyRequest, FollowKolRequest, KolRegistrationRequest,
    KolRelations, NamePros, NameRequest, NameResponse, PatoInfo, PatoInfoResponse,
    PopulationRegistrationRequest, SimpleRequest, SimpleResponse, TokenRequest, TokenResponse,
    UserActiveRequest,
};
use metapower_framework::OFFICIAL_PATO;
use sha1::Digest;
use std::collections::HashMap;

use crate::{BATTERY, BATTERY_CALLEE};

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

    pub fn get_pato_names(&self, ids: Vec<String>) -> Vec<(String,String)> {
        let mut names: Vec<(String,String)> = vec![];
        BATTERY_CALLEE.with(|v| {
            let patos = v.borrow();
            for id in ids{
                if let Some(json_str) = patos.get(&id) {
                    if let Ok(info) = serde_json::from_str::<PatoInfo>(&json_str) {
                        names.push((info.id, info.name));
                    }
                }    
            }
        });

        names
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
        let (followers_json,): (String,) = match call(
            callee,
            "follower_of",
            (id.clone(),),
        )
        .await
        {
            Ok(response) => response,
            Err((code, msg)) => return Err(anyhow!("{}: {}", code as u8, msg)),
        };
        let followers = serde_json::from_str::<Vec<(String,String)>>(&followers_json).unwrap_or_default();
        let (following_json,): (String,) = match call(
            callee,
            "following_of",
            (id.clone(),),
        )
        .await
        {
            Ok(response) => response,
            Err((code, msg)) => return Err(anyhow!("{}: {}", code as u8, msg)),
        };
        let followings = serde_json::from_str::<Vec<(String,String)>>(&following_json).unwrap_or_default();

        let (balance,): (f32,) = match call(
            callee,
            "balance_of",
            (id.clone(),),
        )
        .await
        {
            Ok(response) => response,
            Err((code, msg)) => return Err(anyhow!("{}: {}", code as u8, msg)),
        };

        let mut pato_info: PatoInfoResponse = PatoInfoResponse::default();
        // match MetapowerSqlite3::query_db(
        //     &select_id_table,
        //     vec!["id", "name", "registered_at", "sn"],
        // ) {
        //     Ok(records) => {
        //         if !records.is_empty() {
        //             let record = records.first().unwrap();
        //             let name = record.get("name").unwrap().to_string();
        //             let registered_datetime = record.get("registered_at").unwrap().to_string();
        //             let sn = record.get("sn").unwrap().parse::<i64>().unwrap();
        //             pato_info = PatoInfoResponse {
        //                 id: id.clone(),
        //                 name,
        //                 sn,
        //                 registered_datetime,
        //                 balance,
        //                 tags: tags.split(',').map(|t| t.to_string()).collect(),
        //                 avatar: avatar_link,
        //                 cover,
        //                 followers,
        //                 followings,
        //             };
        //         }
        //     }
        //     Err(e) => {
        //         return Err(e);
        //     }
        // }

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

    pub async fn request_add_balance(
        &self,
        request: ChangeBalanceRequest,
    ) -> std::result::Result<(), Error> {

        let callee = BATTERY.with(|callee| *callee.borrow().as_ref().unwrap());
        match call(
            callee,
            "set_balance_of",
            (request.id, request.amount),
        ).await
        {
            Ok(()) => (),
            Err((_, message)) => ic_cdk::trap(&message),
        };
    
        Ok(())
    }

    pub async fn request_minus_balance(
        &self,
        request: ChangeBalanceRequest,
    ) -> std::result::Result<(), Error> {
        let callee = BATTERY.with(|callee| *callee.borrow().as_ref().unwrap());
        match call(
            callee,
            "set_balance_of",
            (request.id, -request.amount),
        ).await
        {
            Ok(()) => (),
            Err((_, message)) => ic_cdk::trap(&message),
        };

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
