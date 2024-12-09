pub mod runner;
pub mod smith;

use candid::Principal;
use ic_cdk::call;
use ic_cdk::{api::caller, id};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as MM, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, RestrictedMemory, StableBTreeMap};
use metapower_framework::prompt::PREDEFINED_TAGS;
use metapower_framework::{
    get_now_date_str, get_now_secs, ChangeBalanceRequest, FollowKolRequest, KolRegistrationRequest, KolRelations, NameRequest, NameResponse,
    PatoInfo, PatoInfoResponse, PopulationRegistrationRequest, SimpleRequest, SimpleResponse, TokenRequest,
    TokenResponse,
};
use smith::MetaPowerMatrixAgentService;
use std::cell::RefCell;
use std::fmt::Write;
use std::thread;

static mut INITIALIZED: bool = false;
static mut OWNER: Principal = Principal::anonymous();

type RM = RestrictedMemory<DefaultMemoryImpl>;
type VM = VirtualMemory<RM>;

const BATTERY_MEM_ID: MemoryId = MemoryId::new(0);
const BILLBOARD_MEM_ID: MemoryId = MemoryId::new(1);
const METADATA_PAGES: u64 = 300;

thread_local! {
    static AGENT_NAME: RefCell<String> = RefCell::new("".to_string());
    static MEMORY_MANAGER: RefCell<MM<RM>> = RefCell::new(
        MM::init(RM::new(DefaultMemoryImpl::default(), 16..METADATA_PAGES))
    );
    static BATTERY_CALLEE: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BATTERY_MEM_ID)))
        });
    static BILLBOARD: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(BILLBOARD_MEM_ID)))
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
pub fn billboard() -> String {
    _must_initialized();

    BILLBOARD.with(|billboard| {
        let mut result = "".to_string();
        for (k, v) in billboard.borrow().iter() {
            result.push_str(&format!("{}: {}\n", k, v));
        }
        result
    })
}

#[ic_cdk::query]
pub fn get_population_quantities() -> u64 {
    MetaPowerMatrixAgentService::default().request_pato_count()
}

#[ic_cdk::update]
pub fn setup_battery_canister(battery_canister: Principal) {
    BATTERY.with(|callee| {
        *callee.borrow_mut() = Some(battery_canister);
    });
}

#[ic_cdk::update]
async fn request_airdrop(amount: f32, id: String) {
    _must_initialized();

    let request = ChangeBalanceRequest {
        id,
        amount,
        key: "key".to_string(),
    };

    if let Err(e) = MetaPowerMatrixAgentService::new().request_add_balance(request).await {
        ic_cdk::trap(&e.to_string())
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
pub async fn refresh_battery_auth(id: String) -> String {
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

#[ic_cdk::update]
async fn request_pato_info(id: String) -> PatoInfoResponse {
    _must_initialized();
    let request = SimpleRequest { id };
    match MetaPowerMatrixAgentService::new()
        .request_pato_info(request)
        .await
    {
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
fn request_predefined_tags() -> String {
    _must_initialized();

    let tags = USER_DEFINED_TAGS.with(|v| v.borrow().clone());
    if tags.is_empty() {
        PREDEFINED_TAGS.to_string()
    } else {
        tags
    }
}

#[ic_cdk::update]
fn set_predefined_tags(tags: String) {
    _must_initialized();

    USER_DEFINED_TAGS.with(move |v| {
        let data = tags;
        *v.borrow_mut() = data;
    });
}

#[ic_cdk::update]
async fn request_plus_balance(id: String, amount: f32) -> Result<(), String> {
    _must_initialized();
    let request = ChangeBalanceRequest {
        id,
        amount,
        key: "key".to_string(),
    };
    if let Err(e) = MetaPowerMatrixAgentService::new().request_add_balance(request).await {
        ic_cdk::trap(&e.to_string())
    }

    Ok(())
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

    match MetaPowerMatrixAgentService::new().get_pato_name(id) {
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
async fn descrease_battery_power(amount: u64) {
    _must_initialized();

    let mut all_id: Vec<String> = vec![];
    let callee = BATTERY.with(|callee| *callee.borrow().as_ref().unwrap());
    BATTERY_CALLEE.with(|v| {
        for pato in v.borrow().iter() {
            all_id.push(pato.0);
        }
    });
    ic_cdk::spawn( async move {
        for id in all_id {
            match call(
                callee,
                "set_power_of",
                (id.clone(), amount),
            ).await{
                Ok(()) => (),
                Err((_, message)) => ic_cdk::println!("descrease_battery_power error: {}", message),
            }
            match call(
                callee,
                "comment_topic",
                (id.clone(),),
            ).await{
                Ok(()) => (),
                Err((_, message)) => ic_cdk::println!("comment_topic error: {}", message),
            }
            thread::sleep(std::time::Duration::from_secs(5));
        }    
    });
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
fn request_kol_list() -> Vec<KolRelations> {
    _must_initialized();
    match MetaPowerMatrixAgentService::new().request_kol_list() {
        Ok(response) => response,
        Err(e) => ic_cdk::trap(&e.to_string()),
    }
}

#[ic_cdk::update]
async fn request_deposit(id: String, amount: f32) {
    _must_initialized();
    let callee = BATTERY.with(|callee| *callee.borrow().as_ref().unwrap());
    match call(
        callee,
        "set_balance_of",
        (id.clone(), amount),
    ).await
    {
        Ok(()) => (),
        Err((_, message)) => ic_cdk::trap(&message),
    };
}

ic_cdk::export_candid!();
