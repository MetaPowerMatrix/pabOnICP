pub mod bank;
pub mod runner;
pub mod smith;

use std::cell::RefCell;
use candid::{CandidType, Principal};
use ic_cdk::api::caller;
use serde::Deserialize;
use smith::MetaPowerMatrixAgentService;

static mut POPULATION_QUANTITIES: u64 = 0;
static mut INITIALIZED: bool = false;
static mut OWNER: Principal = Principal::anonymous();
static mut FEE_TOKEN_ID: Principal = Principal::anonymous();
//  static mut PAB_TOKEN_CANISTER: Principal = Principal::anonymous();
//  static mut PAB_NFT_CANISTER: Principal = Principal::anonymous();
const CYCLES_PER_TOKEN: u64 = 4000000000000;

thread_local! {
    static AGENT_NAME: RefCell<String> = RefCell::new("".to_string());
}

#[derive(Default)]
struct CanisterID {
   n_f_t_canister_id: String,
   p_a_b_token_canister_id: String,
   avatar_nft_canister_id: String
}

#[derive(Deserialize, CandidType)]
pub struct EmptyRequest {}

#[derive(Deserialize, CandidType)]
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

#[ic_cdk::init]
fn init() {
    unsafe {
        OWNER = caller();
    }
}

fn _only_owner() {
    unsafe {
       if OWNER != caller() {
           ic_cdk::trap("not owner");
       }
    }
}
fn _must_initialized() {
    unsafe {
       if INITIALIZED != true {
           ic_cdk::trap("uninitialized");
       }
    }
}

#[ic_cdk::update]
fn initialize(name: String) -> Result<(), ()> {
   unsafe {
       if INITIALIZED != false {
           ic_cdk::trap("initialized");
       }

       INITIALIZED = true;
       OWNER = caller();
       AGENT_NAME.with(|agent| {
        *agent.borrow_mut() = name;
       });
   }

   Ok(())
}

#[ic_cdk::query]
pub fn hi() -> String{
    _must_initialized();
    let canister = storage::get::<CanisterID>();
    unsafe {
        format!("Hi, {}; {}; {}; {}; {};", OWNER, POPULATION_QUANTITIES, canister.n_f_t_canister_id, 
                canister.p_a_b_token_canister_id, canister.avatar_nft_canister_id)
    }
}

#[ic_cdk::query]
pub fn get_population_quantities() -> u64 {
    unsafe {
        POPULATION_QUANTITIES
    }
}

#[ic_cdk::update]
async fn request_airdrop(amount: f32, id: String) -> Result<SimpleResponse, String> {
    _must_initialized();
    let request = AirdropRequest {
        id,
        amount,
    };

   match MetaPowerMatrixAgentService::new().request_airdrop(request).await {
         Ok(response) => Ok(response),
         Err(err) => Err(err.to_string()),   
   }
}

#[ic_cdk::update]
async fn request_population_registration(name: String, id: String) -> Result<SimpleResponse, String> {
    _must_initialized();
    let request = PopulationRegistrationRequest {
        id,
        name,
    };
    match MetaPowerMatrixAgentService::new().request_population_registration(request).await
    {
        Ok(response) => {
            unsafe {
                POPULATION_QUANTITIES += 1;
            }
                Ok(response)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::query]
fn request_pato_info(id: String) -> Result<PatoInfoResponse, String> {
    _must_initialized();
    let request = SimpleRequest {
        id,
    };
    match MetaPowerMatrixAgentService::new().request_pato_info(request){
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::query]
fn request_pato_of_pro() -> Result<PatoOfProResponse, String> {
    _must_initialized();
    let request = EmptyRequest {};
    match MetaPowerMatrixAgentService::new().request_professionals(request)
    {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::query]
fn request_sn(id: String) -> Result<SnResponse, String>{
    _must_initialized();
    let request = SnRequest {
        id: vec![id],
    };
    match MetaPowerMatrixAgentService::new().request_pato_sn(request){
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::query]
fn request_all_patos() -> Result<AllPatosResponse, String>{
    _must_initialized();
    let request = EmptyRequest {};
    match MetaPowerMatrixAgentService::new().request_for_all_patos(request){
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::update]
fn request_plus_balance(id: String, amount: f32) -> Result<SimpleResponse, String>{
    _must_initialized();
    let request = ChangeBalanceRequest {
        id,
        amount,
        key: "key".to_string(),
    };
    match MetaPowerMatrixAgentService::new().request_add_balance(request){
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::update]
fn request_minus_balance(id: String, amount: f32){
    _must_initialized();
    let request = ChangeBalanceRequest {
        id,
        amount,
        key: "key".to_string(),
    };
    MetaPowerMatrixAgentService::new().request_minus_balance(request);
}

#[ic_cdk::update]
fn request_inject_human_voice(id: String, roles: Vec<String>, session: String, message: String){
    _must_initialized();
    let request = InjectHumanVoiceRequest {
        id,
        roles,
        session,
        message,
    };
    MetaPowerMatrixAgentService::new().request_inject_human_voice(request);
}

#[ic_cdk::query]
async fn request_pato_auth_token(id: String) -> Result<SimpleResponse, String>{
    _must_initialized();
    let request = SimpleRequest {
        id,
    };
    match MetaPowerMatrixAgentService::new().request_pato_auth_token(request).await{
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::update]
async fn request_topic_chat(initial: String, topic: String, town: String) -> Result<EmptyRequest, String>{
    _must_initialized();
    let request = TopicChatRequest {
        initial,
        topic,
        town,
    };
    match MetaPowerMatrixAgentService::new().request_topic_chat(request).await{
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::query]
fn request_topic_chat_his(initial: String, topic: String, town: String) -> Result<TopicChatHisResponse, String>{
    _must_initialized();
    let request = TopicChatRequest {
        initial,
        topic,
        town,
    };
    match MetaPowerMatrixAgentService::new().request_topic_chat_history(request){
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::update]
async fn request_room_create(owner: String, title: String, description: String, town: String) -> Result<RoomCreateResponse, String>{
    _must_initialized();
    let request = RoomCreateRequest {
        owner,
        title,
        description,
        town,
    };
    match MetaPowerMatrixAgentService::new().request_create_room(request).await{
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::query]
fn request_room_list(id: String) -> Result<RoomListResponse, String>{
    _must_initialized();
    let request = SimpleRequest {
        id,
    };
    match MetaPowerMatrixAgentService::new().request_game_room_list(request){
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::query]
async fn request_pato_name_and_pro(id: String) -> Result<NameResponse, String>{
    _must_initialized();
    let request = NameRequest {
        id: vec![id],
    };
    match MetaPowerMatrixAgentService::new().request_pato_name_and_pro(request).await{
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::update]
fn request_kol_registration(id: String){
    _must_initialized();
    let request = KolRegistrationRequest {
        id,
        key: "key".to_string(),
    };

    MetaPowerMatrixAgentService::new().request_kol_registration(request);
}

#[ic_cdk::update]
fn request_follow_kol( id: String, follower: String, key: String){
    _must_initialized();
    let request = FollowKolRequest {
        id,
        follower,
        key,
    };

    MetaPowerMatrixAgentService::new().request_add_kol_follower(request);
}

#[ic_cdk::query]
fn request_kol_list() -> Result<KolListResponse, String>{
    _must_initialized();
    let request = EmptyRequest {};

    match MetaPowerMatrixAgentService::new().request_kol_list(request){
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::update]
async fn request_deposit( id: String, amount: f32, key: String){
    _must_initialized();
    let request = ChangeBalanceRequest {
        id,
        amount,
        key,
    };
    MetaPowerMatrixAgentService::new().request_deposit(request).await;
}
