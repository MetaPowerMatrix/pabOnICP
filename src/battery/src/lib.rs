pub mod id;
pub mod runner;
pub mod reverie;

use std::cell::RefCell;
use anyhow::{anyhow, Error};
use candid::Principal;
use ic_cdk::{call, caller};
use id::MetaPowerMatrixBatteryService;
use metapower_framework::{dao::http::send_http_post_request, log, AnswerReply, BestTalkRequest, MessageRequest, SomeDocs};
use serde::de::DeserializeOwned;

const LLM_PROXY_HOST: &str = "icp.metapowermatrix.ai";

struct LLMSvcClient {
    pub host: String,
    pub module: String,
}

impl Default for LLMSvcClient {
    fn default() -> Self {
        LLMSvcClient::new(LLM_PROXY_HOST.to_string(), "battery".to_string())
    }
}

impl LLMSvcClient {
    pub fn new(host: String, module: String) -> Self {
        LLMSvcClient { host, module }
    }
    pub async fn got_documents_summary(&self, url: &str, req: SomeDocs) -> Result<String, Error> {
        let json_string = serde_json::to_string(&req).unwrap_or_default();

        match send_http_post_request(self.host.clone(), url.to_string(), self.module.clone(), json_string).await {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!("got_documents_summary error: {}", e)),
        }
    }
    pub async fn talk_best(&self, url: &str, req: BestTalkRequest) -> Result<AnswerReply, Error> {
        let json_string = serde_json::to_string(&req).unwrap_or_default();

        match send_http_post_request(self.host.clone(), url.to_string(), self.module.clone(), json_string).await {
            Ok(response) => {
                match serde_json::from_str::<AnswerReply>(&response){
                    Ok(talk_response) => {
                        Ok(talk_response)
                    }
                    Err(e) => {
                        Err(anyhow!("talk_best error: {}", e))
                    }                    
                }
            }
            Err(e) => Err(anyhow!("talk_best error: {}", e)),
        }
    }
    pub async fn call_llm_proxy<T: serde::Serialize, R: DeserializeOwned>(&self, url: &str, req: T) -> Result<R, Error> {
        let json_string = serde_json::to_string(&req).unwrap_or_default();

        match send_http_post_request(self.host.clone(), url.to_string(), self.module.clone(), json_string).await {
            Ok(response) => {
                match serde_json::from_str::<R>(&response){
                    Ok(talk_response) => {
                        Ok(talk_response)
                    }
                    Err(e) => {
                        Err(anyhow!("{} error: {}", url, e))
                    }                    
                }
            }
            Err(e) => Err(anyhow!("{} error: {}", url, e)),
        }
    }
}

static mut INITIALIZED: bool = false;
static mut OWNER: Principal = Principal::anonymous();

thread_local! {
    static MATRIX_CALLEE: RefCell<Option<Principal>> = const { RefCell::new(None) };
    static AGENT_CALLEE: RefCell<Option<Principal>> = const { RefCell::new(None) };
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
pub async fn do_battery_service(id: String, token: String, sn: i64, method_name: String, args: String){
    _must_initialized();
    _auth_battery(id.clone(), token, sn).await;

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
        "talk_best" => {
            let svc =  MetaPowerMatrixBatteryService::new(id);
        }
        "got_documents_summary" => {
            let svc =  MetaPowerMatrixBatteryService::new(id);
        }
        _ => {
            ic_cdk::trap("unknown method");
        }
    }
}

