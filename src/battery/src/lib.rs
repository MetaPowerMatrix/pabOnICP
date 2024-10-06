pub mod id;
pub mod runner;
pub mod reverie;

use std::cell::RefCell;
use anyhow::{anyhow, Error};
use candid::Principal;
use ic_cdk::{call, caller};
use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager, VirtualMemory}, StableBTreeMap, DefaultMemoryImpl, RestrictedMemory};
use id::MetaPowerMatrixBatteryService;
use metapower_framework::{dao::http::{send_http_get_request, send_http_post_request}, log, AnswerReply, BecomeKolRequest, BestTalkRequest, MessageRequest, SomeDocs, SubmitTagsRequest};
use serde::{de::DeserializeOwned, Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct DataResponse {
    pub content: String,
    pub code: String,
}

static mut INITIALIZED: bool = false;
static mut OWNER: Principal = Principal::anonymous();

type RM = RestrictedMemory<DefaultMemoryImpl>;
type VM = VirtualMemory<RM>;

const BATTERY_TAGS_MEM_ID: MemoryId = MemoryId::new(0);
const BATTERY_AVATAR_MEM_ID: MemoryId = MemoryId::new(1);
const BATTERY_CHARACTER_MEM_ID: MemoryId = MemoryId::new(2);
const BATTERY_COVER_MEM_ID: MemoryId = MemoryId::new(3);
const METADATA_PAGES: u64 = 256;

thread_local! {
    static MATRIX_CALLEE: RefCell<Option<Principal>> = const { RefCell::new(None) };
    static AGENT_CALLEE: RefCell<Option<Principal>> = const { RefCell::new(None) };

    static MEMORY_MANAGER: RefCell<MemoryManager<RM>> = RefCell::new(
        MemoryManager::init(RM::new(DefaultMemoryImpl::default(), 16..METADATA_PAGES))
    );

    static BATTERY_TAGS: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_TAGS_MEM_ID)))
        });
        
    static BATTERY_AVATAR: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_AVATAR_MEM_ID)))
        });

    static BATTERY_COVER: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_COVER_MEM_ID)))
        });

    static BATTERY_CHARACTER: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_CHARACTER_MEM_ID)))
        });
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

async fn _auth_battery(id: String, token: String, sn: i64){
    let callee = AGENT_CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());

    let (resp,): ((i64, String),) = match call(
        callee,
        "get_battery_auth",
        (id,),
    ).await{
        Ok(response) => response,
        Err((code, msg)) => {
            ic_cdk::trap(&format!("agent not available: {:?} - {}", code, msg));
        },
    };
    if resp.1 == token {
        log!("pato校验通过");
    }else {
        ic_cdk::trap(&format!("pato校验失败: {}: {}", token, sn));
    }
}

#[ic_cdk::init]
fn init() {
    unsafe {
        OWNER = caller();
    }
}

#[ic_cdk::update]
fn initialize(matrix_canister: Principal, agent_canister: Principal) -> Result<(), ()> {
   unsafe {
       if INITIALIZED {
           ic_cdk::trap("initialized");
       }

       OWNER = caller();
       MATRIX_CALLEE.with(|callee| {
            *callee.borrow_mut() = Some(matrix_canister);
        });
        AGENT_CALLEE.with(|callee| {
            *callee.borrow_mut() = Some(agent_canister);
        });    
        INITIALIZED = true;
    }

   Ok(())
}

#[ic_cdk::update]
pub fn setup_matrix_canister(canister: Principal) {
    _only_owner();
    MATRIX_CALLEE.with(|callee| {
        *callee.borrow_mut() = Some(canister);
    });
}

#[ic_cdk::update]
pub fn setup_agent_canister(canister: Principal) {
    _only_owner();
    AGENT_CALLEE.with(|callee| {
        *callee.borrow_mut() = Some(canister);
    });
}

#[ic_cdk::query]
pub fn hi(id: String) -> String{
    _must_initialized();
    unsafe {
        format!("Hi, my name is {}({}) controlled by {}", id, ic_cdk::id(), OWNER)
    }
}

#[ic_cdk::update]
pub async fn talk(id: String, token: String, message: String, subject: String, prompt: String) -> String{
    _must_initialized();

    let svc =  MetaPowerMatrixBatteryService::new(id);

    let request = MessageRequest{
        message,
        subject,
        prompt,
    };

    let resp = svc.talk(request).await;

    String::default()
}

#[ic_cdk::update]
pub async fn do_battery_service(id: String, token: String, sn: i64, method_name: String, args: String) -> String{
    _must_initialized();
    _auth_battery(id.clone(), token, sn).await;

    let mut response_string = String::default();

    match method_name.as_str() {
        "talk" => {
            let svc =  MetaPowerMatrixBatteryService::new(id);

            let request = MessageRequest{
                message: args,
                subject: "".to_string(),
                prompt: "".to_string(),
            };

            let resp = svc.talk(request).await;
        }
        "become_kol" => {
            let svc =  MetaPowerMatrixBatteryService::new(id.clone());
            match serde_json::from_str::<BecomeKolRequest>(&args){
                Ok(request) => {
                    if let Err(e) = svc.become_kol(request).await{
                        ic_cdk::trap(&e.to_string());
                    }
                }
                Err(e) => {
                    ic_cdk::trap(&format!("become_kol error: {}", e));
                }
            }
        }
        "request_submit_tags" => {
            let svc =  MetaPowerMatrixBatteryService::new(id);
            match serde_json::from_str::<SubmitTagsRequest>(&args){
                Ok(request) => {
                    match svc.request_submit_tags(request).await{
                        Ok(response) => {
                            response_string = serde_json::to_string(&response).unwrap_or_default();
                        }
                        Err(e) => {
                            ic_cdk::trap(&e.to_string());
                        }
                    }
                }
                Err(e) => {
                    ic_cdk::trap(&format!("request_submit_tags error: {}", e));
                }
            }
        }
        _ => {
            ic_cdk::trap("unknown method");
        }
    }

    response_string
}

ic_cdk::export_candid!();
