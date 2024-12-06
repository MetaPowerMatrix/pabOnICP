pub mod controller;
pub mod runner;

use std::cell::RefCell;
use candid::{CandidType, Principal};
use controller::MetaPowerMatrixControllerService;
use ic_cdk::{caller, id};
use metapower_framework::PatoInfo;
use serde::Deserialize;

#[derive(Deserialize, CandidType)]
pub struct Knowledge {
    pub sig: String,
    pub title: String,
    pub owner: String,
    pub summary: String,
}

#[derive(Deserialize, CandidType)]
pub struct SharedKnowledgesResponse {
    pub books: Vec<Knowledge>,
}

#[derive(Deserialize, CandidType, Debug)]
pub struct HotAi {
    pub id: String,
    pub name: String,
    pub talks: i32,
    pub pros: String,
}

#[derive(Deserialize, CandidType)]
pub struct HotAiResponse {
    pub sheniu: Vec<HotAi>,
}

#[derive(Deserialize, CandidType)]
pub struct HotTopicResponse {
    pub topics: Vec<String>,
}

#[derive(Deserialize, CandidType)]
pub struct CreateRequest {
    pub name: String,
}

#[derive(Deserialize, CandidType)]
pub struct CreateResonse {
    pub id: String,
}

#[derive(Deserialize, CandidType)]
pub struct LoginRequest {
    pub id: String,
}


static mut INITIALIZED: bool = false;
static mut OWNER: Principal = Principal::anonymous();

thread_local! {
    static MATRIX_NAME: RefCell<String> = RefCell::new("".to_string());
    static RULES: RefCell<String> = RefCell::new("".to_string());
    static GENISIS: RefCell<String> = RefCell::new("".to_string());
    static DESTINATION: RefCell<String> = RefCell::new("".to_string());
    static CALLEE: RefCell<Option<Principal>> = const { RefCell::new(None) };
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
    //    OWNER = caller();

       MATRIX_NAME.with(|matrix| {
        *matrix.borrow_mut() = name;
       });

       GENISIS.with(|v| {
        *v.borrow_mut() = "In the beginning, the world was in chaos, and the Creator God decided to open up a new era of the universe, so the first batch of silicon-based electronic life, Pato, was born.".to_string();
       });

       RULES.with(|v| {
        *v.borrow_mut() = "To collect as many tokens as possible, the speed of light is the fastest and constant".to_string();
       });

       DESTINATION.with(|v| {
        *v.borrow_mut() = "create and dead, long live universe".to_string();
       });
   }

   Ok(())
}

#[ic_cdk::query]
pub fn hi() -> String{
    _must_initialized();
    unsafe {
        format!("Hi, current matrix is {}({}) controlled by {};", MATRIX_NAME.take(), id(), OWNER)
    }
}

#[ic_cdk::query]
pub fn rules() -> String{
    _must_initialized();
    
    RULES.take()
}

#[ic_cdk::query]
pub fn destination() -> String{
    _must_initialized();
    
    DESTINATION.take()
}

#[ic_cdk::query]
pub fn genisis() -> String{
    _must_initialized();
    
    GENISIS.take()
}

#[ic_cdk::update]
pub fn setup_agent_canister(_id: String, agent_canister: Principal) {
    CALLEE.with(|callee| {
        *callee.borrow_mut() = Some(agent_canister);
    });
}

#[ic_cdk::update]
async fn request_create_pato(name: String) -> CreateResonse{
    _must_initialized();
    let request = CreateRequest {
        name,
    };
    match MetaPowerMatrixControllerService::default().request_create_pato(request).await{
        Ok(response) => response,
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

#[ic_cdk::update]
async fn request_hot_ai() -> Vec<PatoInfo>{
    _must_initialized();
    
    match MetaPowerMatrixControllerService::default().request_hot_ai().await{
        Ok(response) => response,
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

#[ic_cdk::update]
fn upload_session_assets(id: String, session_key: String, file_name: String, file_data: Vec<u8>) -> u64{
    _must_initialized();

    match MetaPowerMatrixControllerService::default().save_session_assets(id, session_key, file_name, file_data){
        Ok(sn) => sn,
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

#[ic_cdk::update]
async fn check_session_assets(id: String, session_key: String, file_name: String) -> (bool, Vec<u8>, u64) {
    _must_initialized();

    match MetaPowerMatrixControllerService::default().get_session_assets(id, session_key, file_name) {
        Ok(data) => (true, data.0, data.1),
        Err(err) => {
            (false, err.to_string().as_bytes().to_vec(), 0)
            // ic_cdk::trap(&err.to_string())
        }
    }
}

#[ic_cdk::update]
async fn query_session_files(path: String) -> Vec<String> {
    _must_initialized();

    match MetaPowerMatrixControllerService::default().list_files(path) {
        Ok(data) => data,
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

#[ic_cdk::update]
async fn query_session_assets(id: String, session_key: String, file_name: String) -> (Vec<u8>, u64) {
    _must_initialized();

    match MetaPowerMatrixControllerService::default().get_session_assets(id, session_key, file_name) {
        Ok(data) => data,
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

ic_cdk::export_candid!();