pub mod runner;
pub mod smith;

use std::cell::RefCell;
use candid::Principal;
use ic_cdk::{api::caller, id};
use smith::MetaPowerMatrixAgentService;
use metapower_framework::{
    AirdropRequest, AllPatosResponse, ChangeBalanceRequest, EmptyRequest, FollowKolRequest, InjectHumanVoiceRequest, KolListResponse, KolRegistrationRequest, NameRequest, NameResponse, PatoInfoResponse, PatoOfProResponse, PopulationRegistrationRequest, RoomCreateRequest, RoomCreateResponse, RoomListResponse, SimpleRequest, SimpleResponse, SnRequest, SnResponse, TokenRequest, TokenResponse, TopicChatHisResponse, TopicChatRequest
};

static mut POPULATION_QUANTITIES: u64 = 0;
static mut INITIALIZED: bool = false;
static mut OWNER: Principal = Principal::anonymous();

thread_local! {
    static AGENT_NAME: RefCell<String> = RefCell::new("".to_string());
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
       if !INITIALIZED {
           ic_cdk::trap("uninitialized");
       }
    }
}

#[ic_cdk::update]
fn initialize(name: String) -> Result<(), ()> {
   unsafe {
       if INITIALIZED {
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
    unsafe {
        format!("Hi, current agent is {}({}) controlled by {};registered pato is {}", AGENT_NAME.take(), id(), OWNER, POPULATION_QUANTITIES)
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
async fn query_pato_auth_token(token: String) -> Result<TokenResponse, String>{
    _must_initialized();
    let request = TokenRequest {
        token,
    };
    match MetaPowerMatrixAgentService::new().query_pato_auth_token(request){
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
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
async fn request_pato_by_name(name: String) -> Result<NameResponse, String>{
    _must_initialized();
    let request = SimpleRequest {
        id: name,
    };
    match MetaPowerMatrixAgentService::new().request_pato_by_name(request).await{
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::query]
async fn request_pato_by_ids(ids: Vec<String>) -> Result<NameResponse, String>{
    _must_initialized();
    let request = NameRequest {
        id: ids,
    };
    match MetaPowerMatrixAgentService::new().request_pato_by_ids(request).await{
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

ic_cdk::export_candid!();