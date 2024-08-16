pub mod controller;
pub mod runner;

use std::{cell::RefCell, path::Path};

use candid::{CandidType, Principal};
use metapower_framework::AI_MATRIX_DIR;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MapStatus {
    pub sn: i64,
    pub patos: Vec<String>,
}

fn self_check() -> bool {
    let mut path_check = true;

    let check_paths = [
        format!("{}/bin", AI_MATRIX_DIR),
        format!("{}/db", AI_MATRIX_DIR),
        format!("{}/log", AI_MATRIX_DIR),
        format!("{}/substance", AI_MATRIX_DIR),
    ];

    check_paths.iter().for_each(|path| {
        if !Path::new(path.as_str()).exists() {
            path_check = false;
        }
    });

    path_check
}


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

#[derive(Deserialize, CandidType)]
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

thread_local! {
    static CALLEE: RefCell<Option<Principal>> = RefCell::new(None);
}

#[ic_cdk::update]
pub fn setup_callee(id: Principal) {
    CALLEE.with(|callee| {
        *callee.borrow_mut() = Some(id);
    });
}

