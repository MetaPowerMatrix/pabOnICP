pub mod id;
pub mod runner;
pub mod reverie;

use std::{cell::RefCell, thread};
use candid::{CandidType, Principal};
use ic_cdk::{call, caller};
use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager, VirtualMemory}, StableBTreeMap, DefaultMemoryImpl, RestrictedMemory};
use id::MetaPowerMatrixBatteryService;
use metapower_framework::{log, BecomeKolRequest, JoinKolRoomRequest, SubmitTagsRequest};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlainDoc {
    pub content: String,
}
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct VecDoc {
    pub content: String,
    pub embeddings: Vec<f32>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum VecQuery {
    Embeddings(Vec<f32>),
}

#[derive(Deserialize, Debug, Default, Serialize)]
struct BatterCallParams{
    id: String,
    token: String,
    sn: i64,
    method_name: String,
    arg: String,
}

#[derive(Deserialize, Debug, Default, Serialize)]
struct TopicChatInfo {
    topic: String,
    prompt: String,
    contributor: String,
    session: String,
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
const BATTERY_SESSION_MEM_ID: MemoryId = MemoryId::new(4);
const BATTERY_POWER_MEM_ID: MemoryId = MemoryId::new(5);
const BATTERY_FOLLOWERS_MEM_ID: MemoryId = MemoryId::new(6);
const BATTERY_FOLLOWING_MEM_ID: MemoryId = MemoryId::new(7);
const BATTERY_TOPICS_MEM_ID: MemoryId = MemoryId::new(8);
const BATTERY_SUB_TOPICS_MEM_ID: MemoryId = MemoryId::new(9);
const BATTERY_BALANCE_MEM_ID: MemoryId = MemoryId::new(10);

const METADATA_PAGES: u64 = 2048;

thread_local! {
    static MATRIX_CALLEE: RefCell<Option<Principal>> = const { RefCell::new(None) };
    static AGENT_CALLEE: RefCell<Option<Principal>> = const { RefCell::new(None) };
    static VECTOR_CALLEE: RefCell<Option<Principal>> = const { RefCell::new(None) };

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

    static BATTERY_SESSIONS: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_SESSION_MEM_ID)))
        });

    static BATTERY_POWER: RefCell<StableBTreeMap<String, u64, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_POWER_MEM_ID)))
        });

    static BATTERY_FOLLOWERS: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_FOLLOWERS_MEM_ID)))
        });

    static BATTERY_FOLLOWING: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_FOLLOWING_MEM_ID)))
        });
    static BATTERY_TOPICS: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_TOPICS_MEM_ID)))
        });
    static BATTERY_SUB_TOPICS: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_SUB_TOPICS_MEM_ID)))
        });
    static BATTERY_BALANCE: RefCell<StableBTreeMap<String, f32, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_BALANCE_MEM_ID)))
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
fn initialize(matrix_canister: Principal, agent_canister: Principal, vector_canister: Principal) -> Result<(), ()> {
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
        VECTOR_CALLEE.with(|callee| {
            *callee.borrow_mut() = Some(vector_canister);
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

#[ic_cdk::query]
pub fn session_of(id: String) -> String{
    _must_initialized();

    let sessions = BATTERY_SESSIONS.with(|sessions| {
        sessions.borrow().get(&id).unwrap_or_default()
    });

    sessions
}

#[ic_cdk::query]
pub fn power_of(id: String) -> u64{
    _must_initialized();

    let power = BATTERY_POWER.with(|power| {
        power.borrow().get(&id).unwrap_or_default()
    });

    power
}

#[ic_cdk::query]
pub fn balance_of(id: String) -> f32{
    _must_initialized();

    let balance = BATTERY_BALANCE.with(|balance| {
        balance.borrow().get(&id).unwrap_or_default()
    });

    balance
}

#[ic_cdk::query]
pub fn follower_of(id: String) -> String{
    _must_initialized();

    let followers = BATTERY_FOLLOWERS.with(|followers| {
        followers.borrow().get(&id).unwrap_or_default()
    });

    followers
}

#[ic_cdk::query]
pub fn following_of(id: String) -> String{
    _must_initialized();

    let following = BATTERY_FOLLOWING.with(|following| {
        following.borrow().get(&id).unwrap_or_default()
    });

    following
}

#[ic_cdk::query]
pub fn topics_of(id: String) -> String{
    _must_initialized();

    let topics = BATTERY_TOPICS.with(|topics| {
        topics.borrow().get(&id).unwrap_or_default()
    });

    topics
}

#[ic_cdk::query]
pub fn sub_topics_of(topic_id: String) -> String{
    _must_initialized();

    let topics = BATTERY_SUB_TOPICS.with(|topics| {
        topics.borrow().get(&topic_id).unwrap_or_default()
    });

    topics
}

#[ic_cdk::update]
pub fn set_tags_of(id: String, tags_submit: String){
    _must_initialized();

    BATTERY_TAGS.with(|tags| {
        let mut tags = tags.borrow_mut();
        tags.insert(id.clone(), tags_submit);
    });

}

#[ic_cdk::update]
pub fn set_cover_of(id: String, cover: String){
    _must_initialized();

    BATTERY_COVER.with(|cover_map| {
        let mut cover_map = cover_map.borrow_mut();
        cover_map.insert(id.clone(), cover);
    });

}

#[ic_cdk::update]
pub fn set_avatar_of(id: String, avatar: String){
    _must_initialized();

    BATTERY_AVATAR.with(|avatar_map| {
        let mut avatar_map = avatar_map.borrow_mut();
        avatar_map.insert(id.clone(), avatar);
    });

}

#[ic_cdk::update]
pub fn set_character_of(id: String, character: String){
    _must_initialized();

    BATTERY_CHARACTER.with(|character_map| {
        let mut character_map = character_map.borrow_mut();
        character_map.insert(id.clone(), character);
    });

}

#[ic_cdk::update]
pub fn set_session_of(id: String, session_key: String){
    _must_initialized();

    BATTERY_SESSIONS.with(|session_map| {
        let mut session_map = session_map.borrow_mut();
        let prev = session_map.get(&id).unwrap_or_default();
        let updated = prev + "," + &session_key;
        session_map.insert(id.clone(), updated);
    });

}

#[ic_cdk::update]
pub fn set_power_of(id: String, new_power: u64){
    _must_initialized();

    BATTERY_POWER.with(|power_map| {
        let mut power_map = power_map.borrow_mut();
        let mut old = power_map.get(&id).unwrap_or(1000);
        let update = old - new_power;
        power_map.insert(id.clone(), update);
    });

}

#[ic_cdk::update]
pub fn set_balance_of(id: String, new_b: f32){
    _must_initialized();

    BATTERY_BALANCE.with(|b_map| {
        let mut b_map = b_map.borrow_mut();
        let prev = b_map.get(&id).unwrap_or_default();
        if new_b < 0.0 && prev < new_b {
            ic_cdk::trap("balance not enough");
        }
        let update = prev + new_b;
        b_map.insert(id.clone(), update);
    });

}

#[ic_cdk::update]
pub fn set_follower_of(id: String, follower: (String,String)){
    _must_initialized();

    BATTERY_FOLLOWERS.with(|follow_map| {
        let mut follow_map = follow_map.borrow_mut();
        let prev_json = follow_map.get(&id).unwrap_or_default();
        let mut prev = serde_json::from_str::<Vec<(String,String)>>(&prev_json).unwrap_or_default();
        prev.push(follower);
        let update = serde_json::to_string(&prev).unwrap_or_default();
        follow_map.insert(id.clone(), update);
    });

}

#[ic_cdk::update]
pub fn set_following_of(id: String, following: (String,String)){
    _must_initialized();

    BATTERY_FOLLOWING.with(|follow_map| {
        let mut follow_map = follow_map.borrow_mut();
        let prev_json = follow_map.get(&id).unwrap_or_default();
        let mut prev = serde_json::from_str::<Vec<(String,String)>>(&prev_json).unwrap_or_default();
        prev.push(following);
        let update = serde_json::to_string(&prev).unwrap_or_default();
        follow_map.insert(id.clone(), update);
    });

}

#[ic_cdk::update]
pub fn set_topics_of(id: String, following: (String,String)){
    _must_initialized();

    BATTERY_TOPICS.with(|topic_map| {
        let mut topic_map = topic_map.borrow_mut();
        let prev_json = topic_map.get(&id).unwrap_or_default();
        let mut prev = serde_json::from_str::<Vec<(String,String)>>(&prev_json).unwrap_or_default();
        prev.push(following);
        let update = serde_json::to_string(&prev).unwrap_or_default();
        topic_map.insert(id.clone(), update);
    });
}

#[ic_cdk::update]
pub fn set_sub_topics_of(topic_id: String, comment: (String,String)){
    _must_initialized();

    BATTERY_SUB_TOPICS.with(|topic_map| {
        let mut topic_map = topic_map.borrow_mut();
        let value = serde_json::to_string(&comment).unwrap_or_default();
        topic_map.insert(topic_id, value);
    });
}

#[ic_cdk::update]
pub async fn comment_topic(id: String){
    _must_initialized();

    let sum: u32 = id.bytes().map(|b| b as u32).sum();
    let prompts = [
        "your name is Jack, You are a chatbot that talk about a topic with funny responses.",
        "your name is Marv, You are a chatbot that reluctantly talk about a topic with sarcastic responses",
        "your name is Zuck, please always give the positive comment to the user's topic"
    ];
    let prompt = prompts[(sum % prompts.len() as u32) as usize];

    let svc =  MetaPowerMatrixBatteryService::new(id.clone());
    let mut topics: Vec<String> = vec![];
    BATTERY_FOLLOWING.with(|following_map| {
        let followings = following_map.borrow();
        for (following_id,_) in followings.iter(){
            BATTERY_TOPICS.with(|topic_map| {
                if let Some(topic) = topic_map.borrow().get(&following_id){
                    topics.push(topic);
                }
            });
        }
    });

    for topic in topics{
        let topic_info = serde_json::from_str::<(String,String)>(&topic).unwrap_or_default();
        match svc.request_comment_topic(topic_info.0, prompt.to_string(), id.clone()).await{
            Ok(_) => (),
            Err(e) => {
                ic_cdk::println!("{}", e);
            }
        }
    }
}

#[ic_cdk::update]
pub async fn search_embeddings(id: String, input: Vec<f32>) -> String{
    _must_initialized();

    let svc =  MetaPowerMatrixBatteryService::new(id);

    match svc.talk(input).await{
        Ok(response) => {
            response.iter().map(|doc| doc.content.clone()).collect::<Vec<String>>().join(",")
        }
        Err(e) => {
            ic_cdk::trap(&e.to_string());
        }
    }
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
                    match svc.request_submit_tags_with_proxy(request).await{
                        Ok(_) => (),
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
