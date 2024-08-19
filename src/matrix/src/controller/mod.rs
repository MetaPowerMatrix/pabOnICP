use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::time::SystemTime;
use std::{env, io};
use std::path::{Path, PathBuf};
use std::vec;
use std::fs;
use anyhow::Error;

use crate::{
    CreateRequest, CreateResonse, EmptyResponse, HotAi, HotAiResponse,
    HotTopicResponse, Knowledge, LoginRequest, SharedKnowledgesResponse, 
    CALLEE,
};
use ic_cdk::api::call::call;
use metapower_framework::{
    dao::sqlite::MetapowerSqlite3, AI_MATRIX_DIR, AI_PATO_DIR,
};
use metapower_framework::{
    get_past_date_str, log, AirdropRequest, AllPatosResponse, EmptyRequest, NameResponse, PatoLocation, SimpleResponse
};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct MetaPowerMatrixControllerService {}

impl MetaPowerMatrixControllerService {
    fn create_pato_db(&self) -> Result<(), Error> {
        let message_table = "CREATE TABLE IF NOT EXISTS chat_message (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            session TEXT NOT NULL,
            place TEXT NOT NULL,
            sender TEXT NOT NULL,
            receiver TEXT NOT NULL,
            question TEXT NOT NULL,
            answer TEXT NOT NULL
        )";
        let pray_table = "CREATE TABLE IF NOT EXISTS pray_message (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sender TEXT NOT NULL,
            content TEXT NOT NULL
        )";

        MetapowerSqlite3 {
            db_name: "population_database".to_string(),
        }
        .create_table(message_table.to_owned())?
        .create_table(pray_table.to_owned())?;

        Ok(())
    }
    fn update_name_file(&self, id: String, name: String) -> Result<(), Error> {
        let namefile = format!("{}/{}/db/name.txt", AI_PATO_DIR, id);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(namefile)?;

        writeln!(file, "{}", name)?;

        Ok(())
    }
    async fn prepare_pato_db(&self, id: String, name: String) -> Result<i64, String> {
        let mut sn = 0;
        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());

        let (resp,): (SimpleResponse,) = match call(
            callee.clone(),
            "request_population_registration",
            (name, id.clone()),
        )
        .await
        {
            Ok(response) => response,
            Err((code, msg)) => return Err(format!("pato注册失败: {}: {}", code as u8, msg)),
        };
        if resp.success {
            if let Ok(last_sn) = resp.message.parse::<i64>() {
                sn = last_sn;
            }
        }

        let request = AirdropRequest { id, amount: 100.0 };
        let (_resp,): (SimpleResponse,) =
            match call(callee.clone(), "request_airdrop", (request,)).await {
                Ok(response) => response,
                Err((code, msg)) => return Err(format!("pato空投失败: {}: {}", code as u8, msg)),
            };

        Ok(sn)
    }
    fn get_pato_shared_books(&self, id: String) -> Vec<String> {
        let mut books: Vec<String> = vec![];
        let name_file = format!("{}/{}/knowledge/shared.txt", AI_PATO_DIR, id);
        if let Ok(file) = File::open(name_file) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                books.push(line.unwrap_or_default());
            }
        }
        books
    }
    fn count_pato_sessions(&self, id: String, count_days: u64) -> i64 {
        let mut session_count = 0;
        for i in 0..count_days {
            let date_string = get_past_date_str(i);
            let session_dir = format!("{}/{}/db/{}", AI_PATO_DIR, id, date_string);
            log!("searching {}", session_dir);
            let path = Path::new(&session_dir);
            if path.exists() && path.is_dir() {
                for entry in fs::read_dir(path).unwrap().flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        session_count += 1;
                    }
                }
            }
        }
        session_count
    }
    fn find_template_dirs(&self) -> Vec<PathBuf> {
        let mut db_dirs = Vec::new();
        let template_file_path = format!("{}/template/personas", AI_MATRIX_DIR);
        let path = Path::new(template_file_path.as_str());
        if path.is_dir() {
            for entry in fs::read_dir(path).unwrap().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    db_dirs.push(path);
                }
            }
        }
        db_dirs
    }
    fn find_events(&self, id: String, count_days: u64) -> Vec<String> {
        let mut lines = vec![];
        for i in 0..count_days {
            let date_string = get_past_date_str(i);
            let event_file = format!("{}/{}/db/event_{}.txt", AI_PATO_DIR, id, date_string);
            if let Ok(file) = File::open(event_file) {
                let reader = io::BufReader::new(file);
                for line in reader.lines().map_while(Result::ok) {
                    let topic = line
                        .split('#')
                        .map(|t| t.to_owned())
                        .collect::<Vec<String>>();
                    if topic.len() > 1 {
                        lines.push(topic[0].clone());
                    } else {
                        lines.push(line);
                    }
                }
            }
        }
        lines
    }
    fn create_pato_iss(&self, id: String, name: String) -> Result<(), Error> {
        let personas_file_path = self.find_template_dirs();
        // let mut rng = thread_rng();
        let persona = personas_file_path.first().unwrap();
        let orig_persona_file = persona.join("bootstrap_memory/scratch.json");
        let dest_persona_file = format!("{}/{}/db/scratch.json", AI_PATO_DIR, id);
        let orig_name = persona.file_name().unwrap().to_str().unwrap();

        match File::open(&orig_persona_file) {
            Ok(file) => {
                let reader = BufReader::new(file);

                // Read the contents of the file into a string
                let mut contents = String::new();
                for line in reader.lines() {
                    contents.push_str(&line.unwrap());
                }

                // Replace all occurrences of the old string with the new string
                let new_contents = contents.replace(orig_name, &name);

                // Open the output file for writing
                match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&dest_persona_file)
                {
                    Ok(mut file) => {
                        // Write the modified string to the output file
                        let _ = file.write_all(new_contents.as_bytes());
                    }
                    Err(e) => {
                        log!("{}文件写入失败: {}", dest_persona_file, e);
                    }
                }
            }
            Err(e) => {
                log!("{:?}文件打开失败: {}", &orig_persona_file, e);
            }
        }

        Ok(())
    }
    pub fn get_all_categories(&self) -> Vec<&str> {
        let activity_categories = self.get_event_category();
        let mut all_categories = vec![];
        for (_, v) in activity_categories.iter() {
            all_categories.extend(v.clone());
        }
        all_categories
    }
    pub fn get_event_category(&self) -> HashMap<&'static str, Vec<&'static str>> {
        HashMap::from([
            ("eat", vec!["cafe"]),
            ("work", vec!["office building", "library"]),
            ("learn", vec!["library", "school"]),
            ("commuting", vec!["airport"]),
            ("entertainment", vec!["museum"]),
            ("shopping", vec!["shopping mall"]),
        ])
    }

    pub fn request_pato_login(
        &self,
        _request: LoginRequest,
    ) -> std::result::Result<EmptyResponse, Error> {
        let response = EmptyResponse {};

        Ok(response)
    }

    pub async fn request_create_pato(
        &self,
        request: CreateRequest,
    ) -> std::result::Result<CreateResonse, Error> {
        let mut create_pato_success = true;

        let pato_id = Uuid::new_v4().to_string();

        let create_paths = [
            format!("{}/{}/bin", AI_PATO_DIR, pato_id),
            format!("{}/{}/db", AI_PATO_DIR, pato_id),
            format!("{}/{}/log", AI_PATO_DIR, pato_id),
            format!("{}/{}/knowledge", AI_PATO_DIR, pato_id),
            format!("{}/{}/live", AI_PATO_DIR, pato_id),
        ];
        create_paths.iter().for_each(|path| {
            if let Err(e) = fs::create_dir_all(path.clone()) {
                log!("{}创建失败: {}", path, e);
                create_pato_success = false;
            }
        });

        if let Err(e) = self.create_pato_db() {
            log!("pato数据库创建失败: {}", e);
            create_pato_success = false;
        }
        let mut sn = 0;
        match self
            .prepare_pato_db(pato_id.clone(), request.name.clone())
            .await
        {
            Ok(last_sn) => {
                sn = last_sn;
            }
            Err(e) => {
                log!("pato注册失败: {}", e);
                create_pato_success = false;
            }
        }
        if let Err(e) = self.update_name_file(pato_id.clone(), request.name.clone()) {
            log!("pato名称文件更新失败: {}", e);
            create_pato_success = false;
        }

        let _ = self.create_pato_iss(pato_id.clone(), request.name.clone());

        let response = if create_pato_success {
            CreateResonse {
                id: pato_id.clone(),
            }
        } else {
            CreateResonse { id: "".to_string() }
        };

        Ok(response)
    }
    pub async fn request_hot_ai(
        &self,
        _request: EmptyRequest,
    ) -> std::result::Result<HotAiResponse, String> {
        let mut all_ais: Vec<HotAi> = vec![];
        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());

        let (patos_resp,): (AllPatosResponse,) =
            match call(callee, "request_all_patos", ()).await {
                Ok(response) => response,
                Err((code, msg)) => {
                    return Err(format!("request_all_patos失败: {}: {}", code as u8, msg))
                }
            };
        let patos = patos_resp.pato_sn_id;
        let ids: Vec<String> = patos.iter().map(|p| p.id.to_owned()).collect();

        let (resp,): (NameResponse,) =
            match call(callee, "request_pato_name_and_pro", (ids,)).await {
                Ok(name_pro_resp) => name_pro_resp,
                Err((code, msg)) => {
                    return Err(format!(
                        "request_pato_name_and_pro: {}: {}",
                        code as u8, msg
                    ))
                }
            };
        for pato in patos {
            for name_pro in resp.name_pros.iter() {
                if name_pro.id == pato.id {
                    let sessions = self.count_pato_sessions(pato.id.clone(), 3);
                    let info = HotAi {
                        id: pato.id.clone(),
                        name: name_pro.name.clone(),
                        talks: sessions as i32,
                        pros: name_pro.pros.join(","),
                    };
                    all_ais.push(info);
                }
            }
        }

        all_ais.sort_by(|a, b| b.talks.cmp(&a.talks));
        log!("all sorted ais {:?}", all_ais);
        let hot_ais = if all_ais.len() > 10 {
            let _ = all_ais.split_off(5);
            all_ais
        } else {
            all_ais
        };
        log!("hot_ais {:?}", hot_ais);
        let response = HotAiResponse { sheniu: hot_ais };
        Ok(response)
    }

    pub async fn request_shared_knowledges(
        &self,
        _request: EmptyRequest,
    ) -> std::result::Result<SharedKnowledgesResponse, String> {
        let mut knowledges: Vec<Knowledge> = vec![];

        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());

        let (patos_resp,): (AllPatosResponse,) =
            match call(callee, "request_all_patos", ()).await {
                Ok(response) => response,
                Err((code, msg)) => {
                    return Err(format!("request_all_patos失败: {}: {}", code as u8, msg))
                }
            };
        let patos = patos_resp.pato_sn_id;
        for pato in patos {
            let books = self.get_pato_shared_books(pato.id.clone());
            for book in books {
                if !book.is_empty() && book.contains('#') {
                    let mut summary = String::new();
                    let pair = book
                        .split('#')
                        .map(|b| b.to_owned())
                        .collect::<Vec<String>>();
                    let summary_file_path = format!(
                        "{}/{}/knowledge/{}.summary",
                        AI_PATO_DIR,
                        pato.id.clone(),
                        pair[1].clone()
                    );
                    if let Ok(mut sig_file) = OpenOptions::new().read(true).open(summary_file_path)
                    {
                        let _ = sig_file.read_to_string(&mut summary);
                    }
                    if !summary.is_empty() {
                        let resp = Knowledge {
                            sig: pair[1].clone(),
                            title: pair[0].clone(),
                            owner: pato.id.clone(),
                            summary,
                        };
                        knowledges.push(resp);
                    }
                }
            }
        }

        let response = SharedKnowledgesResponse { books: knowledges };
        Ok(response)
    }

    pub async fn request_hot_topics(
        &self,
        _request: EmptyRequest,
    ) -> std::result::Result<HotTopicResponse, String> {
        let mut all_events = vec![];

        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());

        let (patos_resp,): (AllPatosResponse,) =
            match call(callee, "request_all_patos", ()).await {
                Ok(response) => response,
                Err((code, msg)) => {
                    return Err(format!("request_all_patos失败: {}: {}", code as u8, msg))
                }
            };

        let patos = patos_resp.pato_sn_id;
        for pato in patos {
            let events = self.find_events(pato.id, 2);
            if !events.is_empty() {
                all_events.extend(events);
            }
        }

        let mut set = HashSet::new();
        let mut result = Vec::new();
        for item in all_events {
            if set.insert(item.clone()) {
                result.push(item.clone());
            }
        }

        let response = HotTopicResponse { topics: result };
        Ok(response)
    }
}
