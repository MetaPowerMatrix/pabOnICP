pub mod dao;
pub mod prompt;
pub mod mqtt;

pub extern crate ic_canister_log;

use std::{fs, path::Path};
use candid::CandidType;
use serde::de::DeserializeOwned;
// use chrono::{DateTime, Utc, Timelike};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write, Seek, SeekFrom};

pub const METAPOWER_BROKER:&str = "mqtt://127.0.0.1:3881";
pub const AGENT_GRPC_SERVER: &str = "0.0.0.0:9300";
pub const MATRIX_GRPC_SERVER: &str = "0.0.0.0:9400";
pub const BATTERY_GRPC_SERVER: &str = "0.0.0.0";
pub const BATTERY_GRPC_SERVER_PORT_START: i64 = 9500;
pub const AGENT_GRPC_REST_SERVER: &str = "http://127.0.0.1:9300";
pub const MATRIX_GRPC_REST_SERVER: &str = "http://127.0.0.1:9400";
pub const BATTERY_GRPC_REST_SERVER: &str = "http://127.0.0.1";
pub const LLMCHAT_GRPC_REST_SERVER: &str = "http://127.0.0.1:50051";
pub const SOLANA_MAIN_NET: &str = "https://api.mainnet-beta.solana.com";
pub const SOLANA_DEV_NET: &str = "https://api.devnet.solana.com";
pub const SOLANA_LOCALTEST_NET: &str = "http://127.0.0.1:8899";
pub const AI_PATO_DIR: &str = "/data/pato";
pub const AI_AGENT_DIR: &str = "/data/agent";
pub const AI_MATRIX_DIR: &str = "/data/matrix";
pub const TICK: u64 = 10;
pub const SNAP: u64 = 3;
pub const HAVEAREST: u64 = 2;
pub const AFTERNOONTEA: u64 = 20;
pub const SECS_PER_HOUR: u64 = 36000;
pub const XFILES_SERVER: &str = "https://xfiles.metapowermatrix.ai";
pub const XFILES_LOCAL_DIR: &str = "/data/www/xfiles";
pub const OFFICIAL_PATO: &str = "20cc7dbd-10e6-473a-bed7-626771504a9e";
pub const CREDITCARD_PAY_HOST: &str = "paas-gateway-test.imetastore.io";
pub const CREDITCARD_PAY_HOST_TEST: &str = "www.igv.com";
pub const PAY_CLIENT_ID: &str = "willispapaya@gmail.com";
pub const PAY_CLIENT_SECRET: &str = "Jc75EVBNXpgk4xW0ztMLzCt0MCoLIBp8";
pub const DEFAULT_PAY_LOGIN_PASS: &str = "123123";
pub const PAY_TENANT_ID: u64 = 3332001;
pub const SUB_BASIC: &str = "1785146807172653057";
pub const SUB_PLUS: &str = "1785147040068456450";

pub const QUESTIONER_TEMPLATE: &str = r#"You are a person who likes to learn and recently hope to gain a deeper 
    understanding of some knowledge in the {domain} field. You need to ask some constructive questions to gain 
    relevant knowledge. make a dialogue according to this, please ask only one question.
"#;
pub const ANSWERER_TEMPLATE: &str = r#""You are an expert in the {domain} field and can provide professional advice to the questioner,
    Answer the question below:"
"#;
pub const ANSWERER_TEMPLATE_RAG: &str = r#""You are an expert in the {domain} field and can provide professional advice to the questioner,
    Answer the question based only on the following context: {context}"
"#;
pub const DEFAULT_TEMPLATE: &str = r#"You are a person who likes to talk to people. 
    Talking can relieve stress and get useful information at the same time.
"#;

#[derive(Debug, Deserialize, Serialize)]
pub struct DataResponse {
    pub content: String,
    pub code: String,
}
pub struct TileTypeMap {
    pub category: String,
    pub name: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct  TileTypeMapRust {
    pub category: String,
    pub name: Vec<String>
}

impl From<TileTypeMap> for TileTypeMapRust {
    fn from(tile_type_map: TileTypeMap) -> Self {
        TileTypeMapRust {
            category: tile_type_map.category,
            name: tile_type_map.name
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MotionSensorMessage {
    pub accelerator: Vec<MotionSensorData>,
    pub gyroscope: Vec<MotionSensorData>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MotionSensorData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub gx: f64,
    pub gy: f64,
    pub gz: f64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SceneAction {
    pub action: String,
    pub download_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionInfo {
    pub place: String,
    pub action: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PatoInfo {
    pub sn: i64,
    pub id: String,
    pub name: String,
    pub matrix_datetime: String,
    pub registered_datetime: String,
    pub professionals: Vec<String>,
    pub balance: f32,
    pub tags: Vec<String>,
    pub avatar: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapTileInfo {
    pub sn: i64,
    pub name: String,
    pub top_corner: Coordinate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PatoLocation {
    pub id: String,
    pub tile_sn: i64,
    pub step: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionMessages {
    pub session: String,
    pub summary: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    pub created_at: i64,
    pub session: String,
    pub place: String,
    pub sender: String,
    pub receiver: String,
    pub question: String,
    pub answer: String,
    #[serde(default)]
    pub sender_role: String,
    #[serde(default)]
    pub subject: String,
}

#[derive(Deserialize, CandidType, Serialize)]
pub struct EmptyRequest {}

#[derive(Deserialize, CandidType, Serialize)]
pub struct AirdropRequest {
    pub id: String,
    pub amount: f32,
}

#[derive(Deserialize, CandidType)]
pub struct SimpleResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize, CandidType)]
pub struct PopulationRegistrationRequest {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, CandidType)]
pub struct SimpleRequest {
    pub id: String,
}

#[derive(Deserialize, CandidType, Default)]
pub struct PatoInfoResponse {
    pub id: String,
    pub name: String,
    pub sn: i64,
    pub registered_datetime: String,
    pub professionals: Vec<String>,
    pub balance: f32,
    pub tags: Vec<String>,
    pub avatar: String,
}

#[derive(Deserialize, CandidType, Debug)]
pub struct PatoOfPro {
    pub id: String,
    pub subjects: Vec<String>,
    pub name: String,
}

#[derive(Deserialize, CandidType)]
pub struct SnIdPaire {
    pub id: String,
    pub sn: String,
}

#[derive(Deserialize, CandidType)]
pub struct SnResponse {
    pub pato_sn_id: Vec<SnIdPaire>,
}

#[derive(Deserialize, CandidType)]
pub struct AllPatosResponse {
    pub pato_sn_id: Vec<SnIdPaire>,
}

#[derive(Deserialize, CandidType)]
pub struct ChangeBalanceRequest {
    pub id: String,
    pub amount: f32,
    pub key: String,
}

#[derive(Deserialize, CandidType)]
pub struct InjectHumanVoiceRequest {
    pub id: String,
    pub roles: Vec<String>,
    pub session: String,
    pub message: String,
}

#[derive(Deserialize, CandidType)]
pub struct TokenRequest {
    pub token: String,
}

#[derive(Deserialize, CandidType, Default)]
pub struct TokenResponse {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, CandidType)]
pub struct TopicChatRequest {
    pub initial: String,
    pub topic: String,
    pub town: String,
}

#[derive(Deserialize, CandidType)]
pub struct TopicChatHisResponse {
    pub history: Vec<String>,
}

#[derive(Deserialize, CandidType)]
pub struct ProfessionalsResponse {
    pub professionals: Vec<String>,
}

#[derive(Deserialize, CandidType)]
pub struct RoomCreateRequest {
    pub owner: String,
    pub title: String,
    pub description: String,
    pub town: String,
}

#[derive(Deserialize, CandidType)]
pub struct RoomCreateResponse {
    pub room_id: String,
    pub cover: String,
}

#[derive(Deserialize, CandidType)]
pub struct RoomListResponse {
    pub rooms: Vec<RoomInfo>,
}

#[derive(Deserialize, CandidType)]
pub struct RoomInfo {
    pub room_id: String,
    pub owner: String,
    pub title: String,
    pub description: String,
    pub cover: String,
    pub town: String,
}

#[derive(Deserialize, CandidType)]
pub struct NamePros {
    pub id: String,
    pub name: String,
    pub pros: Vec<String>,
}

#[derive(Deserialize, CandidType, Default)]
pub struct NameResponse {
    pub name_pros: Vec<NamePros>,
}

#[derive(Deserialize, CandidType)]
pub struct NameRequest {
    pub id: Vec<String>,
}

#[derive(Deserialize, CandidType)]
pub struct KolRegistrationRequest {
    pub id: String,
    pub key: String,
}

#[derive(Deserialize, CandidType)]
pub struct FollowKolRequest {
    pub id: String,
    pub follower: String,
    pub key: String,
}

#[derive(Deserialize, CandidType)]
pub struct KolRelations {
    pub id: String,
    pub name: String,
    pub follower: Vec<String>,
}

#[derive(Deserialize, CandidType)]
pub struct KolListResponse {
    pub relations: Vec<KolRelations>,
}

#[derive(Deserialize, CandidType)]
pub struct PatoOfProResponse {
    pub patos: Vec<PatoOfPro>,
}

#[derive(Deserialize, CandidType)]
pub struct SnRequest {
    pub id: Vec<String>,
}

#[derive(Deserialize, CandidType)]
pub struct UserActiveRequest {
    pub id: String,
    pub page: String,
    pub action: String,
}

#[derive(Deserialize, CandidType, Serialize)]
pub struct ImageGenRequest {
    pub prompt: String,
}
#[derive(Deserialize, CandidType)]
pub struct ImageGenResponse {
    pub image_url: String,
}

#[derive(Deserialize, CandidType, Serialize)]
pub struct MessageRequest {
    pub message: String,
    pub subject: String,
    pub prompt: String,
}

pub fn get_event_subjects() -> Vec<&'static str>{
    vec![
        "emotion", "work", "life", "study", "family", "relationship", "health", "hobby", "travel", "food", "sports",
        "music", "movie", "book", "game", "technology", "fashion", "art", "news", "politics", "science", "history",
        "culture", "language", "education", "religion", "philosophy", "psychology", "society", "environment", "economy",
        "business", "law", "military", "geography", "astronomy", "math", "physics", "chemistry", "biology", "medicine",
        "engineering", "computer", "internet", "AI", "blockchain", "big data", "cloud computing", "cybersecurity",
        "quantum computing", "FiveG", "IoT", "smart city", "smart factory", "smart transportation", "smart healthcare",
        "smart education", "smart tourism", "career", "job", "invest", "finance", "insurance", "tax", "real estate",
        "stock", "cryptocurrency", "forex", "commodity", "fund", "retirement", "pension", "social security", "welfare",
        "charity", "volunteer", "NGO", "nonprofit", "government", "party", "election", "democracy", "dictatorship",
        "monarchy", "republic", "federation", "parliament", "vacation", "holiday", "festival", "celebration", "party",
        "agriculture", "industry", "energy", "communication", "construction", "transportation", "space", "ocean", "forest", 
        "desert", "mountain", "river", "lake", "sea", "beach", "island", "city", "village", "country", "continent", "earth",
        "universe",
    ]
}
// pub fn get_now_secs() -> u64 {
//     SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
// }
// pub fn get_now_date_str() -> String {
//     let system_time = SystemTime::now();
//     let datetime: DateTime<Utc> = system_time.into();

//     datetime.format("%Y-%m-%d").to_string()
// }
// pub fn get_tomorrow_date_str() -> String {
//     let system_time = SystemTime::now().checked_add(std::time::Duration::from_secs(86400)).unwrap_or(SystemTime::now());
//     let datetime: DateTime<Utc> = system_time.into();

//     datetime.format("%Y-%m-%d").to_string()
// }
// pub fn get_past_date_str(day: u64) -> String {
//     let system_time = SystemTime::now().checked_sub(std::time::Duration::from_secs(86400*day)).unwrap_or(SystemTime::now());
//     let datetime: DateTime<Utc> = system_time.into();

//     datetime.format("%Y-%m-%d").to_string()
// }
// pub fn get_now_secs_str() -> String {
//     let system_time = SystemTime::now();
//     let datetime: DateTime<Utc> = system_time.into();

//     datetime.format("%d-%m-%YTZ%H:%M:%S").to_string()
// }
// pub fn get_now_secs_str_zh() -> String {
//     let system_time = SystemTime::now();
//     let datetime: DateTime<Utc> = system_time.into();

//     datetime.format("%Y-%m-%d %H:%M").to_string()
// }
// pub fn get_now_mils() -> u128 {
//     SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
// }
pub fn ensure_directory_exists(dir_path: &str) -> std::io::Result<()> {
    let path = Path::new(dir_path);
    if !path.exists() {
        // The directory does not exist, attempt to create it
        fs::create_dir_all(path)?;
        log!("Directory created: {}", dir_path);
    } else {
        println!("Directory already exists: {}", dir_path);
    }
    Ok(())
}

pub fn read_and_writeback_json_file<T: DeserializeOwned + Serialize>(file_path: &str, content_apped: &mut Vec<T>) -> std::io::Result<()> {

    // Open the file for reading and writing
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(file_path)?;

    // Read the existing content of the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the JSON content into a Vec<Item>
    let mut items: Vec<T> = serde_json::from_str(&contents).unwrap_or_else(|_| vec![]);

    // Add a new item to the array
    items.append(content_apped);

    // Serialize the modified Vec<Item> back to a JSON string
    let modified_contents = serde_json::to_string(&items).unwrap();

    // Seek to the start of the file before writing back
    file.seek(SeekFrom::Start(0))?;

    // Optionally truncate the file to ensure no leftover data
    file.set_len(modified_contents.len() as u64)?;

    // Write the modified JSON string back to the file
    file.write_all(modified_contents.as_bytes())?;

    Ok(())
}


#[macro_export]
macro_rules! log {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        {
            $crate::ic_canister_log::declare_log_buffer!(name = LOG, capacity = 100);

            let system_time = ic_cdk::api::time();
            let formatted_msg = format!("[{:?}][{}]{}", system_time, ic_cdk::id(), format_args!($fmt $(, $($arg)+)?));
    
            $crate::ic_canister_log::log!(LOG, "{}", formatted_msg);    
        }
    };
}
