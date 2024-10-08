pub mod controller;
pub mod runner;

use std::cell::RefCell;
use candid::{CandidType, Principal};
use controller::MetaPowerMatrixControllerService;
use ic_cdk::{caller, id};
use metapower_framework::PatoInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, CandidType)]
pub struct EmptyResponse {}

#[derive(Deserialize, CandidType)]
pub struct CheckPayOrdersRequest {
    pub order: String,
    pub buyer_id: String,
}

#[derive(Deserialize, CandidType)]
pub struct CheckPayOrdersResponse {
    pub order: String,
    pub paid: bool,
}

#[derive(Deserialize, CandidType)]
pub struct CreditCardPayRequest {
    pub id: String,
    pub item: String,
    pub amount: i32,
}

#[derive(Deserialize, CandidType)]
pub struct CreditCardPayResponse {
    pub pay_url: String,
}

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
pub struct SubscriptionRequest {
    pub id: String,
    pub amount: f32,
    pub sub_type: String,
}

#[derive(Deserialize, CandidType)]
pub struct DonationRequest {
    pub id: String,
    pub amount: f32,
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
pub struct NearbyRequest {
    pub sn: i64,
}

#[derive(Deserialize, CandidType)]
pub struct NearbyRespnse {
    pub id: Vec<String>,
}

#[derive(Deserialize, CandidType)]
pub struct LoginRequest {
    pub id: String,
}

#[derive(Deserialize, CandidType)]
pub struct PrayRequest {
    pub id: String,
    pub message: String,
}

#[derive(Deserialize, CandidType)]
pub struct MakeProfessionRequest {
    pub id: String,
    pub message: String,
    pub knowledge: String,
    pub file_sig: String,
}

#[derive(Deserialize, CandidType)]
pub struct MakePlanRequest {
    pub id: String,
    pub message: String,
    pub refresh: bool,
}

#[derive(Deserialize, CandidType)]
pub struct MakePlanResponse {
    pub plan_file: String,
}

#[derive(Deserialize, CandidType)]
pub struct PlaceRequest {
    pub place_type: String,
}

#[derive(Deserialize, CandidType)]
pub struct PlaceResonse {
    pub sn: Vec<i64>,
}

#[derive(Deserialize, CandidType)]
pub struct BatteryInfo {
    pub sn: i64,
    pub id: String,
    pub canister: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapStatus {
    pub sn: i64,
    pub patos: Vec<String>,
}

static mut INITIALIZED: bool = false;
static mut OWNER: Principal = Principal::anonymous();

thread_local! {
    static MATRIX_NAME: RefCell<String> = RefCell::new("".to_string());
}

thread_local! {
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
async fn request_shared_knowledges() -> Result<SharedKnowledgesResponse, String>{
    _must_initialized();
    match MetaPowerMatrixControllerService::default().request_shared_knowledges().await{
        Ok(response) => Ok(response),
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

ic_cdk::export_candid!();