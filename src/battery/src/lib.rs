pub mod id;
pub mod runner;
pub mod reverie;

use std::cell::RefCell;
use candid::Principal;
use ic_cdk::{call, caller};
use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager, VirtualMemory}, StableBTreeMap, DefaultMemoryImpl, RestrictedMemory};
use id::MetaPowerMatrixBatteryService;
use metapower_framework::{log, BecomeKolRequest, JoinKolRoomRequest, MessageRequest, SubmitTagsRequest};
use serde::{Deserialize, Serialize};
use stable_fs::{fs::FileSystem, storage::stable::StableStorage};

#[derive(Deserialize, Debug, Default, Serialize)]
struct BatterCallParams{
    id: String,
    token: String,
    sn: i64,
    method_name: String,
    arg: String,
}


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
const METADATA_PAGES: u64 = 1024;

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

    static FS: RefCell<FileSystem> = {
        MEMORY_MANAGER.with(|m| {
            let memory_manager = m.borrow();
            let storage = StableStorage::new_with_memory_manager(&memory_manager, 200..210u8);
            RefCell::new(
                FileSystem::new(Box::new(storage)).unwrap()
            )
        })
    };
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

#[ic_cdk::query]
pub fn character_of(id: String) -> String{
    _must_initialized();

    let character = BATTERY_CHARACTER.with(|character| {
        character.borrow().get(&id).unwrap_or_default()
    });

    character
}

#[ic_cdk::query]
pub fn avatar_of(id: String) -> String{
    _must_initialized();

    let avatar = BATTERY_AVATAR.with(|avatar| {
        avatar.borrow().get(&id).unwrap_or_default()
    });

    avatar
}

#[ic_cdk::query]
pub fn cover_of(id: String) -> String{
    _must_initialized();

    let cover = BATTERY_COVER.with(|cover| {
        cover.borrow().get(&id).unwrap_or_default()
    });

    cover
}

#[ic_cdk::query]
pub fn tags_of(id: String) -> String{
    _must_initialized();

    let tags = BATTERY_TAGS.with(|tags| {
        tags.borrow().get(&id).unwrap_or_default()
    });

    tags
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
pub async fn do_battery_service(args: String) -> String{
    _must_initialized();

    let call_params = match serde_json::from_str::<BatterCallParams>(&args){
        Ok(params) => params,
        Err(e) => {
            ic_cdk::trap(&format!("parse call params error: {}", e));
        }
    };

    // _auth_battery(call_params.id.clone(), call_params.token.clone(), call_params.sn).await;

    let mut response_string = String::default();

    match call_params.method_name.as_str() {
        "talk" => {
            let svc =  MetaPowerMatrixBatteryService::new(call_params.id);
            match serde_json::from_str::<MessageRequest>(&call_params.arg){
                Ok(request) => {
                    match svc.talk(request).await{
                        Ok(response) => {
                            response_string = serde_json::to_string(&response).unwrap_or_default();
                        }
                        Err(e) => {
                            ic_cdk::trap(&e.to_string());
                        }
                    }
                }
                Err(e) => {
                    ic_cdk::trap(&format!("talk error: {}", e));
                }
            }
        }
        "become_kol" => {
            let svc =  MetaPowerMatrixBatteryService::new(call_params.id.clone());
            match serde_json::from_str::<BecomeKolRequest>(&call_params.arg){
                Ok(request) => {
                    match svc.become_kol(request).await{
                        Ok(response) => {
                            response_string = serde_json::to_string(&response).unwrap_or_default();
                        }
                        Err(e) => {
                            ic_cdk::trap(&e.to_string());
                        }
                    }
                }
                Err(e) => {
                    ic_cdk::trap(&format!("become_kol error: {}/{}", e, call_params.arg));
                }
            }
        }
        "request_join_kol_room" => {
            let svc =  MetaPowerMatrixBatteryService::new(call_params.id.clone());
            match serde_json::from_str::<JoinKolRoomRequest>(&call_params.arg){
                Ok(request) => {
                    if let Err(e) = svc.request_join_kol_room(request).await{
                        ic_cdk::trap(&e.to_string());
                    }
                }
                Err(e) => {
                    ic_cdk::trap(&format!("become_kol error: {}", e));
                }
            }
        }
        "request_submit_tags" => {
            let svc =  MetaPowerMatrixBatteryService::new(call_params.id);
            match serde_json::from_str::<SubmitTagsRequest>(&call_params.arg){
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
