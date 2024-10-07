pub mod runner;
pub mod smith;

use candid::Principal;
use ic_cdk::{api::caller, id};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as MM, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, RestrictedMemory, StableBTreeMap, StableLog};
use metapower_framework::{
    get_now_date_str, get_now_secs, AirdropRequest, ChangeBalanceRequest, EmptyRequest, FollowKolRequest, InjectHumanVoiceRequest, KolListResponse, KolRegistrationRequest, NameRequest, NameResponse, PatoInfo, PatoInfoResponse, PopulationRegistrationRequest, RoomCreateRequest, RoomCreateResponse, RoomListResponse, SimpleRequest, SimpleResponse, TokenRequest, TokenResponse, TopicChatHisResponse, TopicChatRequest
};
use metapower_framework::prompt::PREDEFINED_TAGS;
use smith::MetaPowerMatrixAgentService;
use std::cell::RefCell;
use std::fmt::Write;

static mut INITIALIZED: bool = false;
static mut OWNER: Principal = Principal::anonymous();

type RM = RestrictedMemory<DefaultMemoryImpl>;
type VM = VirtualMemory<RM>;

const BATTERY_MEM_ID: MemoryId = MemoryId::new(0);
const METADATA_PAGES: u64 = 256;

thread_local! {
    static AGENT_NAME: RefCell<String> = RefCell::new("".to_string());
    static MEMORY_MANAGER: RefCell<MM<RM>> = RefCell::new(
        MM::init(RM::new(DefaultMemoryImpl::default(), 16..METADATA_PAGES))
    );
    static BATTERY_CALLEE: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_MEM_ID)))
        });
    static USER_DEFINED_TAGS: RefCell<String> =  RefCell::new("".to_string());
    static BATTERY: RefCell<Option<Principal>> = const { RefCell::new(None) };
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
pub fn hi() -> String {
    _must_initialized();
    unsafe {
        format!(
            "Hi, current agent is {}({}) controlled by {};registered pato is {}",
            AGENT_NAME.take(),
            id(),
            OWNER,
            MetaPowerMatrixAgentService::default().request_pato_count()
        )
    }
}

#[ic_cdk::query]
pub fn get_population_quantities() -> u64 {
    MetaPowerMatrixAgentService::default().request_pato_count()
}

#[ic_cdk::update]
pub fn setup_battery_canister(_id: String, battery_canister: Principal) {
    BATTERY.with(|callee| {
        *callee.borrow_mut() = Some(battery_canister);
    });
}

#[ic_cdk::update]
async fn request_airdrop(amount: f32, id: String) -> SimpleResponse {
    _must_initialized();
    let request = AirdropRequest { id, amount };

    match MetaPowerMatrixAgentService::new()
        .request_airdrop(request)
        .await
    {
        Ok(response) => response,
        Err(err) => {
            ic_cdk::trap(&err.to_string());
        }
    }
}

#[ic_cdk::update]
async fn request_population_registration(name: String, id: String) -> SimpleResponse {
    _must_initialized();
    let request = PopulationRegistrationRequest {
        id: id.clone(),
        name: name.clone(),
    };
    match MetaPowerMatrixAgentService::new()
        .request_population_registration(request)
        .await
    {
        Ok(response) => {
            let (bytes,): (Vec<u8>,) =
                ic_cdk::api::call::call(Principal::management_canister(), "raw_rand", ())
                    .await
                    .unwrap_or_default();
            // let pato_id = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();
            let token = bytes.iter().fold("PT".to_string(), |mut acc, a| {
                write!(acc, "{:02x}", a).unwrap_or_default();
                acc
            });
            let info = PatoInfo {
                sn: response.message.parse::<i64>().unwrap_or(-1),
                id: id.clone(),
                name,
                registered_datetime: get_now_date_str(),
                tags: vec![],
                avatar: "".to_string(),
                token,
                token_refresh_at: get_now_secs(),
            };

            BATTERY_CALLEE.with(|v| {
                v.borrow_mut()
                    .insert(id, serde_json::to_string(&info).unwrap_or_default())
            });

            response
        }
        Err(err) => {
            ic_cdk::trap(&err.to_string());
        }
    }
}

#[ic_cdk::update]
pub fn setup_battery_auth(id: String, token: String) {
    _only_owner();
    BATTERY_CALLEE.with(|v| {
        if let Some(json_str) = v.borrow().get(&id) {
            if let Ok(mut info) = serde_json::from_str::<PatoInfo>(&json_str) {
                info.token = token;
                info.token_refresh_at = get_now_secs();
                v.borrow_mut()
                    .insert(id, serde_json::to_string(&info).unwrap_or_default());
            }
        }
    });
}

#[ic_cdk::update]
pub async fn refresh_battery_auth(id: String) -> String{
    let (bytes,): (Vec<u8>,) =
        ic_cdk::api::call::call(Principal::management_canister(), "raw_rand", ())
            .await
            .unwrap_or_default();
    let token = bytes.iter().fold("".to_string(), |mut acc, a| {
        write!(acc, "{:02x}", a).unwrap_or_default();
        acc
    });

    BATTERY_CALLEE.with(|v| {
        if let Some(json_str) = v.borrow().get(&id) {
            if let Ok(mut info) = serde_json::from_str::<PatoInfo>(&json_str) {
                info.token = token.clone();
                info.token_refresh_at = get_now_secs();
                v.borrow_mut()
                    .insert(id, serde_json::to_string(&info).unwrap_or_default());
            }
        }
    });

    token
}

#[ic_cdk::query]
pub async fn get_battery_auth(id: String) -> Option<String> {
    let mut token = None;
    BATTERY_CALLEE.with(|v| {
        if let Some(json_str) = v.borrow().get(&id) {
            if let Ok(info) = serde_json::from_str::<PatoInfo>(&json_str) {
                token = Some(info.token);
            }
        }
    });

    token
}

#[ic_cdk::query]
async fn request_pato_info(id: String) -> PatoInfoResponse {
    _must_initialized();
    let request = SimpleRequest { id };
    match MetaPowerMatrixAgentService::new().request_pato_info(request).await {
        Ok(response) => response,
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

#[ic_cdk::query]
fn request_all_patos() -> Vec<PatoInfo> {
    _must_initialized();

    match MetaPowerMatrixAgentService::new().request_for_all_patos() {
        Ok(response) => response,
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

#[ic_cdk::query]
fn request_predefined_tags() -> String{
    _must_initialized();

    unsafe {
        let tags = USER_DEFINED_TAGS.with(|v| v.borrow().clone());
        if tags.is_empty() {
            PREDEFINED_TAGS.to_string()
        }else{
            tags
        }
    }
}

#[ic_cdk::update]
fn set_predefined_tags(tags: String) {
    _must_initialized();

    USER_DEFINED_TAGS.with(move |v|{
        let data = tags;
        *v.borrow_mut() = data;
    });
}

#[ic_cdk::update]
fn request_plus_balance(id: String, amount: f32) -> Result<SimpleResponse, String> {
    _must_initialized();
    let request = ChangeBalanceRequest {
        id,
        amount,
        key: "key".to_string(),
    };
    match MetaPowerMatrixAgentService::new().request_add_balance(request) {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::update]
async fn request_minus_balance(id: String, amount: f32) {
    _must_initialized();
    let request = ChangeBalanceRequest {
        id,
        amount,
        key: "key".to_string(),
    };
    if let Err(e) = MetaPowerMatrixAgentService::new()
        .request_minus_balance(request)
        .await
    {
        ic_cdk::trap(&e.to_string())
    }
}

#[ic_cdk::update]
fn request_inject_human_voice(id: String, roles: Vec<String>, session: String, message: String) {
    _must_initialized();
    let request = InjectHumanVoiceRequest {
        id,
        roles,
        session,
        message,
    };
    if let Err(e) = MetaPowerMatrixAgentService::new().request_inject_human_voice(request) {
        ic_cdk::trap(&e.to_string())
    }
}

#[ic_cdk::query]
async fn query_pato_kol_token(token: String) -> TokenResponse {
    _must_initialized();
    let request = TokenRequest { token };
    match MetaPowerMatrixAgentService::new().query_pato_kol_token(request) {
        Ok(response) => response,
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

#[ic_cdk::query]
async fn query_pato_by_kol_token(token: String) -> TokenResponse {
    _must_initialized();
    let request = TokenRequest { token };
    match MetaPowerMatrixAgentService::new().query_pato_by_kol_token(request) {
        Ok(response) => response,
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

#[ic_cdk::update]
async fn request_pato_kol_token(id: String) -> SimpleResponse {
    _must_initialized();
    let request = SimpleRequest { id };
    match MetaPowerMatrixAgentService::new()
        .request_pato_kol_token(request)
        .await
    {
        Ok(response) => response,
        Err(err) => ic_cdk::trap(&err.to_string()),
    }
}

#[ic_cdk::update]
async fn request_topic_chat(initial: String, topic: String, town: String) {
    _must_initialized();
    let request = TopicChatRequest {
        initial,
        topic,
        town,
    };
    if let Err(e) = MetaPowerMatrixAgentService::new()
        .request_topic_chat(request)
        .await
    {
        ic_cdk::trap(&e.to_string())
    }
}

#[ic_cdk::query]
fn request_topic_chat_his(
    initial: String,
    topic: String,
    town: String,
) -> Result<TopicChatHisResponse, String> {
    _must_initialized();
    let request = TopicChatRequest {
        initial,
        topic,
        town,
    };
    match MetaPowerMatrixAgentService::new().request_topic_chat_history(request) {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::update]
async fn request_room_create(
    owner: String,
    title: String,
    description: String,
    town: String,
) -> Result<RoomCreateResponse, String> {
    _must_initialized();
    let request = RoomCreateRequest {
        owner,
        title,
        description,
        town,
    };
    match MetaPowerMatrixAgentService::new()
        .request_create_room(request)
        .await
    {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::query]
fn request_room_list(id: String) -> Result<RoomListResponse, String> {
    _must_initialized();
    let request = SimpleRequest { id };
    match MetaPowerMatrixAgentService::new().request_game_room_list(request) {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::query]
async fn request_pato_by_name(name: String) -> Result<NameResponse, String> {
    _must_initialized();
    let request = SimpleRequest { id: name };
    match MetaPowerMatrixAgentService::new()
        .request_pato_by_name(request)
        .await
    {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::query]
async fn request_pato_name(id: String) -> String {
    _must_initialized();

    match MetaPowerMatrixAgentService::new()
        .get_pato_name(id)
    {
        Some(response) => response,
        None => ic_cdk::trap("pato name not registered"),
    }
}

#[ic_cdk::query]
async fn request_pato_by_ids(ids: Vec<String>) -> Result<NameResponse, String> {
    _must_initialized();
    let request = NameRequest { id: ids };
    match MetaPowerMatrixAgentService::new()
        .request_pato_by_ids(request)
        .await
    {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::update]
fn request_kol_registration(id: String) {
    _must_initialized();
    let request = KolRegistrationRequest {
        id,
        key: "".to_string(),
    };

    if let Err(e) = MetaPowerMatrixAgentService::new().request_kol_registration(request) {
        ic_cdk::trap(&e.to_string())
    };
}

#[ic_cdk::update]
fn request_follow_kol(id: String, follower: String, key: String) {
    _must_initialized();
    let request = FollowKolRequest { id, follower, key };

    if let Err(e) = MetaPowerMatrixAgentService::new().request_add_kol_follower(request) {
        ic_cdk::trap(&e.to_string());
    }
}

#[ic_cdk::query]
fn request_kol_list() -> Result<KolListResponse, String> {
    _must_initialized();
    let request = EmptyRequest {};

    match MetaPowerMatrixAgentService::new().request_kol_list(request) {
        Ok(response) => Ok(response),
        Err(err) => Err(err.to_string()),
    }
}

#[ic_cdk::update]
async fn request_deposit(id: String, amount: f32, key: String) {
    _must_initialized();
    let request = ChangeBalanceRequest { id, amount, key };
    if let Err(e) = MetaPowerMatrixAgentService::new()
        .request_deposit(request)
        .await
    {
        ic_cdk::trap(&e.to_string());
    }
}

ic_cdk::export_candid!();
