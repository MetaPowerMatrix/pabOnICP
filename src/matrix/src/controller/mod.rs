use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, io};
use std::collections::{HashMap, HashSet};
use std::fs::{write, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::process::CommandExt;

use std::path::{Path, PathBuf};
use std::vec;
use std::{fs, process::Command};

use anyhow::{anyhow, Error};

use ic_cdk::api::call::call;
use metapower_framework::{get_event_subjects, get_now_date_str, get_now_secs, get_past_date_str, get_tomorrow_date_str, log, ActionInfo, MapTileInfo, PatoLocation, TileTypeMapRust, AGENT_GRPC_REST_SERVER, DEFAULT_PAY_LOGIN_PASS, LLMCHAT_GRPC_REST_SERVER, PAY_CLIENT_ID, PAY_CLIENT_SECRET};
use metapower_framework::{
    dao::sqlite::MetapowerSqlite3,
    AI_AGENT_DIR, AI_MATRIX_DIR, AI_PATO_DIR,
};
use rand::seq::IteratorRandom;
use rand::prelude::SliceRandom;
use uuid::Uuid;
use libc::{self};
use rand::{thread_rng, Rng};
use crate::{CreateRequest, CreateResonse, DonationRequest, EmptyResponse, HotAi, HotAiResponse, HotTopicResponse, Knowledge, LoginRequest, MakePlanRequest, MakePlanResponse, MakeProfessionRequest, MapStatus, NearbyRequest, NearbyRespnse, PlaceRequest, PlaceResonse, PrayRequest, SharedKnowledgesResponse, SubscriptionRequest, CALLEE};

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
    async fn prepare_pato_db(&self, id: String, name: String) -> Result<i64, Error> {
        let mut sn = 0;
        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());        
        let request = PopulationRegistrationRequest {
            id: id.clone(),
            name,
        };

        match call(callee.clone(), "request_population_registration", request).await {
            Ok(response) => {
                if response.success {
                    if let Ok(last_sn) = response.message.parse::<i64>() {
                        sn = last_sn;
                    }
                } else {
                    return Err(anyhow!("pato注册失败"));
                }
            }
            Err(e) => {
                return Err(anyhow!(e.to_string() + "pato注册失败"));
            }
        }
        let request = AirdropRequest { id, amount: 100.0 };
        match call(callee.clone(), "request_airdrop", request).await {
            Ok(response) => {
                if !response.success {
                    return Err(anyhow!("pato空投失败"));
                }
            }
            Err(e) => {
                return Err(anyhow!(e.to_string() + "pato空投失败"));
            }
        }

        Ok(sn)
    }
    fn write_message_to_pray_db(
        &self,
        db_name: String,
        from: String,
        content: String,
    ) -> Result<(), Error> {
        let pray_table = "CREATE TABLE IF NOT EXISTS pray_message (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sender TEXT NOT NULL,
            message TEXT NOT NULL
        )";
        let _ = MetapowerSqlite3 {
                db_name: "population_database".to_string(),
            }.create_table(pray_table.to_owned());

        let write_message_table = "INSERT INTO pray_message (sender, message) VALUES (?1, ?2)";

        MetapowerSqlite3 {
                db_name: "population_database".to_string(),
            }.insert_record(write_message_table, &[&from, &content])?;

        Ok(())
    }
    fn run_battey(&self, id: String, sn: u64) -> std::io::Result<()> {
        let battery_life = "/data/bin/metapower_battery";
        //         -i {} -s {}", id, sn);
        unsafe {
            Command::new(battery_life)
                .arg("-i")
                .arg(id)
                .arg("-s")
                .arg(sn.to_string())
                .pre_exec(|| {
                    // This makes the child process the leader of a new session
                    libc::setsid();
                    Ok(())
                })
                .spawn()?;
        }

        log!("Started process in detached mode.");
        Ok(())
    }
    fn get_pato_shared_books(&self, id: String) -> Vec<String> {
        let mut books: Vec<String> = vec![];
        let name_file = format!("{}/{}/knowledge/shared.txt", AI_PATO_DIR, id);
        if let Ok(file) = File::open(name_file){
            let reader = BufReader::new(file);
            for line in reader.lines(){
                books.push(line.unwrap_or_default());
            }
        }
        books
    }

    pub fn request_init_matrix() {
        let mut init_matrix_success = true;

        let create_paths = [
            format!("{}/smith/bin", AI_AGENT_DIR),
            format!("{}/smith/db", AI_AGENT_DIR),
            format!("{}/db", AI_MATRIX_DIR),
            format!("{}/smith/log", AI_AGENT_DIR),
            format!("{}/log", AI_MATRIX_DIR),
        ];

        create_paths.iter().for_each(|path| {
            if let Err(e) = fs::create_dir_all(path.clone()) {
                log!("{}创建失败: {}", path, e);
                init_matrix_success = false;
            }
        });

        if !init_matrix_success {
            panic!("matrix init failed");
        }
    }
    fn count_pato_sessions(&self, id: String, count_days: u64) -> i64 {
        let mut session_count = 0;
        for i in 0..count_days{
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
        for i in 0..count_days{
            let date_string = get_past_date_str(i);
            let event_file = format!("{}/{}/db/event_{}.txt", AI_PATO_DIR, id, date_string);
            if let Ok(file) = File::open(event_file){
                let reader = io::BufReader::new(file);    
                for line in reader.lines().map_while(Result::ok) {
                    let topic = line.split('#').map(|t| t.to_owned()).collect::<Vec<String>>();
                    if topic.len() > 1 {
                        lines.push(topic[0].clone());
                    }else{
                        lines.push(line);
                    }
                }    
            }
        }
        lines
    }
    fn create_pato_iss(&self, id: String, name: String) -> Result<(), Error> {
        let personas_file_path = self.find_template_dirs();
        let mut rng = thread_rng();
        let persona = personas_file_path.choose(&mut rng).unwrap();
        let orig_persona_file = persona.join("bootstrap_memory/scratch.json");
        let dest_persona_file = format!("{}/{}/db/scratch.json", AI_PATO_DIR, id);
        let orig_name = persona.file_name().unwrap().to_str().unwrap();

        match File::open(&orig_persona_file){
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
                match OpenOptions::new().write(true).create(true).truncate(true).open(&dest_persona_file){
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
            ("eat", vec![
                "cafe", 
            ]),
            (
                "work",
                vec![
                    "office building",
                    "library",
                ],
            ),
            (
                "learn",
                vec![
                    "library",
                    "school",
                ],
            ),
            (
                "commuting",
                vec![
                    "airport", 
                ],
            ),
            ("entertainment", vec![
                "museum"
            ]),
            ("shopping", vec![
                "shopping mall", 
            ]),
        ])
    }

    fn request_pato_login(
        &self,
        _request: LoginRequest,
    ) -> std::result::Result<EmptyResponse, Error> {
        let response = EmptyResponse {};

        Ok(response)
    }

    async fn request_create_pato(
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

        let dbfilename = format!("{}/{}/db/{}.sqlite3", AI_PATO_DIR, pato_id, pato_id);
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
        if let Err(e) = self.run_battey(pato_id.clone(), sn as u64) {
            log!("pato启动失败: {}", e);
            create_pato_success = false;
        }

        let _ = self.create_pato_iss(pato_id.clone(), request.name.clone());

        let location_file_path =
            PathBuf::from(format!("{}/{}/db/location.json", AI_PATO_DIR, pato_id));
        if let Ok(file) = fs::File::create(location_file_path.clone()) {
            let mut rng = rand::thread_rng();
            let first_tile = rng.gen_range(0..49); // Generate a random direction
            let first_location = PatoLocation {
                id: pato_id.clone(),
                step: 0,
                tile_sn: first_tile,
            };
            if let Err(e) = serde_json::to_writer::<File, PatoLocation>(file, &first_location) {
                log!("pato位置文件写入失败: {}", e);
                // create_pato_success = false;
            }
        }

        let response = if create_pato_success {
            CreateResonse {
                id: pato_id.clone(),
            }
        } else {
            CreateResonse { id: "".to_string() }
        };

        Ok(response)
    }

    fn request_pray(
        &self,
        request: PrayRequest,
    ) -> std::result::Result<EmptyResponse, Error> {
        let dbfilename = format!("{}/db/matrix.sqlite3", AI_MATRIX_DIR);
        let from = request.id.clone();
        let content = request.message.clone();

        match self.write_message_to_pray_db(dbfilename, from, content) {
            Ok(_) => {
                let response = EmptyResponse {};
                Ok(response)
            }
            Err(e) => Err(anyhow!( e.to_string())),
        }
    }

    async fn request_make_profession(
        &self,
        request: MakeProfessionRequest,
    ) -> std::result::Result<EmptyResponse, Error> {
        let response = EmptyResponse {};
        let subjects = get_event_subjects();
        let mut subject = String::from("general");
        let knowledge_file = request.knowledge.clone();
        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());        

        let mut file_format = String::from("txt");
        if let Ok(mut file) = File::open(knowledge_file){
            let mut buf = vec![0; 4096]; // Read more bytes to improve accuracy
            if file.read(&mut buf).is_ok(){
                match infer::get(&buf) {
                    Some(kind) => {
                        println!("File type: {:?}", kind.mime_type());
                        file_format = kind.mime_type().to_string();
                    }
                    None => println!("Could not determine file type"),
                }    
            }
        }

        // store embeddings to vectorstrorage
        if let Ok(mut llm_client) = ChatSvcClient::connect(LLMCHAT_GRPC_REST_SERVER).await {
            let llmrequest = SomeDocs {
                doc_file: request.knowledge.clone(),
                doc_format: file_format.clone(),
            };
            log!("llmrequest: {:?}", llmrequest);
            let sum_resp = llm_client.got_documents_summary(llmrequest).await?;            
            let summary = sum_resp.summary.clone();
            
            let topic_subject_request = EventTopic{
                topic: summary,
                subjects: subjects.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            };
            if let Ok(response) = llm_client.got_topic_subject (topic_subject_request).await{
                subject = response.subject.clone();
            }

            let embed_request = DocsRequest {
                doc_file: request.knowledge.clone(),
                collection: subject.clone(),
                db_path: format!(
                    "{}/{}/db/knowledge_chromadb",
                    AI_PATO_DIR,
                    request.id.clone()
                ),
                doc_id: request.file_sig.clone(),
                doc_format: file_format,
            };

            if let Err(e) = llm_client.embed_documents(embed_request).await {
                return Err(anyhow!( e.to_string()));
            }
        }

        let agent_request = ProfessionalRegistrationRequest {
            id: request.id.clone(),
            message: request.message.clone(),
            knowledge: request.knowledge.clone(),
            subject
        };
        log!("agent_request: {:?}", agent_request);
        match call(callee.clone(), "request_professional_registration", agent_request).await {
            Ok(response) => {
                if !response.success {
                    return Err(anyhow!("professional注册失败"));
                }
            }
            Err(e) => {
                return Err(anyhow!(e.to_string() + "professional注册失败"));
            }
        }

        Ok(response)
    }

    async fn request_make_town_map(
        &self,
        _request: EmptyResponse,
    ) -> std::result::Result<EmptyResponse, Error> {
        let response = EmptyResponse {};
        return Ok(response);
    }

    fn make_plan_for_pato(
        &self,
        request: MakePlanRequest,
    ) -> std::result::Result<MakePlanResponse, Error> {
        let use_saved_plan = !request.refresh;

        let pato_id = request.id.clone();
        log!("create plan for pato: {}", pato_id);
        let str_plan_file = format!(
            "{}/{}/db/plan_{}.json",
            AI_PATO_DIR,
            pato_id,
            get_now_date_str()
        );
        let plan_file = PathBuf::from(str_plan_file.clone());
        if use_saved_plan && plan_file.exists() && plan_file.is_file() {
            return Ok(MakePlanResponse {
                plan_file: str_plan_file.clone(),
            });
        }

        let actions_place_map: HashMap<&str, Vec<&str>> = self.get_event_category();
        let mut rng = thread_rng();
        let random_keys = actions_place_map
            .keys()
            .choose_multiple(&mut rng, 3)
            .iter()
            .map(|&x| *x)
            .collect::<Vec<&str>>();

        // To get both keys and their corresponding values
        let random_items = random_keys
            .iter()
            .map(|key| (*key, actions_place_map[*key].clone()))
            .collect::<Vec<(&str, Vec<&str>)>>();

        let plan: Vec<ActionInfo> = random_items
            .iter()
            .map(|(key, value)| {
                let action = (*key).to_owned();
                let place = value.choose(&mut rng).unwrap().to_string();
                ActionInfo { action, place }
            })
            .collect::<Vec<ActionInfo>>();
        
        // println!("{:?}", plan);

        write(
            plan_file.clone(),
            serde_json::to_string(&plan).unwrap_or_default(),
        )?;
        let response = MakePlanResponse {
            plan_file: str_plan_file.clone(),
        };
        return Ok(response);
    }

    fn request_pato_nearby(
        &self,
        request: NearbyRequest,
    ) -> std::result::Result<NearbyRespnse, Error> {
        let map_st_path = format!("{}/db/map_status.json", AI_MATRIX_DIR);
        let mut pato_nearby: Vec<String> = vec![];
        if let Ok(file) = fs::File::open(map_st_path) {
            if let Ok(map_status) = serde_json::from_reader::<File, Vec<MapStatus>>(file) {
                for map in map_status {
                    if map.sn == request.sn {
                        pato_nearby = map.patos;
                        break;
                    }
                }
            }
        }
        let response = NearbyRespnse { id: pato_nearby };
        return Ok(response);
    }
    
    async fn request_place_in_map(&self,request: PlaceRequest,) -> std::result::Result<PlaceResonse, Error> {
        let mut map = vec![];
        let mut map_classified = vec![];
        let mut is_classified = false;

        let map_filepath = format!("{}/db/map.json", AI_MATRIX_DIR);
        if let Ok(file) = fs::File::open(map_filepath) {
            match serde_json::from_reader::<File, Vec<MapTileInfo>>(file) {
                Ok(mapdata) => {
                    // log!("all map tiles: {:?}", mapdata);
                    map = mapdata;
                }
                Err(e) => {
                    log!("map.json read error: {}", e);
                }
            }
        }

        let map_classified_filepath = PathBuf::from(format!("{}/db/map_classified.json", AI_MATRIX_DIR));
        if map_classified_filepath.exists() && map_classified_filepath.is_file() {
            if let Ok(file) = fs::File::open(map_classified_filepath.clone()) {
                match serde_json::from_reader::<File, Vec<TileTypeMapRust>>(file) {
                    Ok(mapdata) => {
                        // log!("all classified tils: {:?}", mapdata);
                        map_classified = mapdata;
                        is_classified = true;
                    }
                    Err(e) => {
                        log!("map_classified.json read error: {}", e);
                    }
                }
            }
        }

        if !is_classified {
            let all_names = map.iter().map(|tile| tile.name.clone()).collect::<Vec<String>>();
            let all_categories_set: HashSet<_> = self.get_all_categories().into_iter().collect();
            let all_categories: Vec<&str> = all_categories_set.into_iter().collect();
            log!("all category in map: {:?}", all_categories);
            // log!("all places in map: {:?}", all_names);
            if let Ok(mut client) = ChatSvcClient::connect(LLMCHAT_GRPC_REST_SERVER).await {
                let tile_request = TileClassifyRequest {
                    name: all_names,
                    category: all_categories.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
                };
                match client.classify_map_tile(tile_request).await {
                    Ok(response) => {
                        let classified_tiles = response.classified_tiles.clone();
                        // log!("classified_tiles: {:?}", classified_tiles);
                        let stored_map = classified_tiles.iter().map(|t| TileTypeMapRust::from(t.to_owned())).collect::<Vec<TileTypeMapRust>>();
                        map_classified = stored_map.clone();
                        log!("classified_tiles: {:?}", map_classified);
        
                        match OpenOptions::new().create(true).write(true).truncate(true).open(map_classified_filepath.clone()){
                            Ok(file) => {
                                let _ = serde_json::to_writer::<File, Vec<TileTypeMapRust>>(file, &stored_map);
                            }
                            Err(e) => {
                                log!("write classify_map file Error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        log!("classify_map_tile Error: {}", e);
                    }
                }
            }
        }


        let mut sns = vec![];
        let tiles = map_classified.iter().filter(|t| t.category == request.place_type);
        for tile in tiles {
            for name in &tile.name{
                for t in map.iter(){
                    if t.name == *name{
                        sns.push(t.sn);
                    }
                }
            }
        }
        log!("places choices: {:?}", sns);

        let response = PlaceResonse { sn: sns };
        return Ok(response);
    }
    
    fn accept_donation(&self,request: DonationRequest,) -> std::result::Result<EmptyResponse, Error> {
        let from = request.id.clone();
        let amount = request.amount;

        let donation_table = "CREATE TABLE IF NOT EXISTS donation (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            donator TEXT NOT NULL,
            amount DOUBLE NOT NULL
        )";
        let _ = MetapowerSqlite3::new().create_table(donation_table.to_owned());

        let write_donation_table = "INSERT INTO donation (donator, amount) VALUES (?1, ?2)";

        match  MetapowerSqlite3::new().insert_record(write_donation_table, &[&from, &amount]){
            Ok(_) => {
                let response = EmptyResponse {};
                Ok(response)
            }
            Err(e) => Err(anyhow!(e.to_string())),   
        }
    }
    
    async fn accept_stake(&self,request:DonationRequest,) -> std::result::Result<EmptyResponse, Error> {
        let from = request.id.clone();
        let amount = request.amount;

        let donation_table = "CREATE TABLE IF NOT EXISTS stake (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            investor TEXT NOT NULL,
            amount DOUBLE NOT NULL
        )";
        let _ = MetapowerSqlite3 {
                db_name: "population_database".to_string(),
            }.create_table(donation_table.to_owned());

        let write_donation_table = "INSERT INTO stake (investor, amount) VALUES (?1, ?2)";

        match  MetapowerSqlite3::new().insert_record(write_donation_table, &[&from, &amount]){
            Ok(_) => {
                let response = EmptyResponse {};
                Ok(response)
            }
            Err(e) => Err(anyhow!( e.to_string())),   
        }
    }
    
    async fn accept_subscription(&self,request: SubscriptionRequest,) -> std::result::Result<EmptyResponse, Error> {
        let id = request.id.clone();
        let mut amount = request.amount;
        let sub_type = request.sub_type.clone();

        let sub_table = "CREATE TABLE IF NOT EXISTS subscription (
            sn INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            id TEXT NOT NULL,
            amount DOUBLE NOT NULL,
            sub_type TEXT NOT NULL
        )";
        let _ = MetapowerSqlite3 {
                db_name: "population_database".to_string(),
            }.create_table(sub_table.to_owned());
        if amount <= 0.0 {
            amount  = 10000.0;
        }
        if sub_type == "pro" {
            amount = 120000.0;
        }else if sub_type == "plus"{
            amount = 1500000.0;
        }

        let add_req = ChangeBalanceRequest {
            id: id.clone(),
            amount,
            key: String::default(),
        };

        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());        
        call(callee.clone(), "request_add_balance", add_req).await;

        let write_sub_table = "INSERT INTO subscription (id, amount, sub_type) VALUES (?1, ?2, ?3)";

        match  MetapowerSqlite3::new().insert_record(write_sub_table, &[&id, &amount, &sub_type]){
            Ok(_) => {
                let response = EmptyResponse {};
                Ok(response)
            }
            Err(e) => Err(anyhow!( e.to_string())),   
        }
    }
    
    async fn request_hot_ai(&self, _request:EmptyResponse,) -> std::result::Result<HotAiResponse, Error> {
        let mut all_ais: Vec<HotAi> = vec![];
        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());        

        let request = EmptyRequest{};
        if let Ok(patos_resp) = call(callee.clone(), "request_for_all_patos", request).await{
            let patos = patos_resp.pato_sn_id.clone();
            let name_pro_request = NameRequest { id: patos.iter().map(|p| p.id.to_owned()).collect() };
            match call(callee.clone(), "request_pato_name_and_pro", name_pro_request).await {
                Ok(name_pro_resp) => {
                    let resp = name_pro_resp.name_pros.clone();
                    for pato in patos {
                        for name_pro in resp.iter(){
                            if name_pro.id == pato.id{
                                let sessions = self.count_pato_sessions(pato.id.clone(), 3);
                                let info = HotAi{
                                    id: pato.id.clone(),
                                    name: name_pro.name.clone(),
                                    talks: sessions as i32,
                                    pros: name_pro.pros.join(","),
                                };
                                all_ais.push(info);
                            }
                        }
                    }
                }
                Err(e) => {
                    log!("request_pato_name_and_pro error: {}", e);
                }
            }
        }

        all_ais.sort_by(|a, b| b.talks.cmp(&a.talks));
        log!("all sorted ais {:?}", all_ais);
        let hot_ais = if all_ais.len() > 10{
            let _ = all_ais.split_off(5);
            all_ais
        }else{
            all_ais
        };
        log!("hot_ais {:?}", hot_ais);
        let response = HotAiResponse { sheniu: hot_ais};
        Ok(response)
    }
    
    async fn request_shared_knowledges(&self, _request:EmptyResponse,) -> std::result::Result<SharedKnowledgesResponse, Error> {
        let mut knowledges: Vec<Knowledge> = vec![];

        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());        

        let request = EmptyRequest{};
        if let Ok(patos_resp) = call(callee.clone(), "request_for_all_patos", request).await{
            let patos = patos_resp.pato_sn_id.clone();
            for pato in patos {
                let books  = self.get_pato_shared_books(pato.id.clone());
                for book in books{
                    if !book.is_empty() && book.contains('#'){
                        let mut summary = String::new();
                        let pair = book.split('#').map(|b| b.to_owned()).collect::<Vec<String>>();
                        let summary_file_path = format!("{}/{}/knowledge/{}.summary", AI_PATO_DIR, pato.id.clone(), pair[1].clone());
                        if let Ok(mut sig_file) = OpenOptions::new().read(true).open(summary_file_path){
                            let _ = sig_file.read_to_string(&mut summary);
                        }
                        if !summary.is_empty(){
                            let resp = Knowledge{
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
        }
        let response = SharedKnowledgesResponse { books: knowledges };
        Ok(response)
    }
    
    async fn request_hot_topics(&self, _request:EmptyResponse,) -> std::result::Result<HotTopicResponse, Error> {
        let mut all_events = vec![];

        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());        

        let request = EmptyRequest{};
        if let Ok(patos_resp) = call(callee.clone(), "request_for_all_patos", request).await{
            let patos = patos_resp.pato_sn_id.clone();
            for pato in patos {
                let events = self.find_events(pato.id, 2);
                if !events.is_empty(){
                    all_events.extend(events);
                }
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
