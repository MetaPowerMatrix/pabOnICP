pub mod id;
pub mod runner;
pub mod reverie;

use std::cell::RefCell;
use anyhow::{anyhow, Error};
use candid::Principal;
use ic_cdk::caller;
use id::MetaPowerMatrixBatteryService;
use metapower_framework::{dao::http::send_http_post_request, AnswerReply, BestTalkRequest, MessageRequest, SomeDocs};
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
    static BATTERY_ID: RefCell<String> = RefCell::new("".to_string());
    static BATTERY_SN: RefCell<u64> = RefCell::new(1);
    static MATRIX_CALLEE: RefCell<Option<Principal>> = const { RefCell::new(None) };
    static AGENT_CALLEE: RefCell<Option<Principal>> = const { RefCell::new(None) };
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
       BATTERY_ID.with(|battery| {
        *battery.borrow_mut() = name;
       });
   }

   Ok(())
}

#[ic_cdk::update]
pub fn setup_matrix_canister(canister: Principal) {
    MATRIX_CALLEE.with(|callee| {
        *callee.borrow_mut() = Some(canister);
    });
}

#[ic_cdk::update]
pub fn setup_agent_canister(canister: Principal) {
    AGENT_CALLEE.with(|callee| {
        *callee.borrow_mut() = Some(canister);
    });
}

#[ic_cdk::query]
pub fn hi() -> String{
    _must_initialized();
    unsafe {
        format!("Hi, my name is {}({}) controlled by {};registered pato is {:?}", BATTERY_ID.take(), ic_cdk::id(), OWNER, BATTERY_SN)
        }
}

#[ic_cdk::update]
pub async fn talk(message: String, subject: String, prompt: String) -> String{
    _must_initialized();

    let svc =  MetaPowerMatrixBatteryService::new(BATTERY_ID.take());

    let request = MessageRequest{
        message,
        subject,
        prompt,
    };

    let resp = svc.talk(request).await;

    String::default()
}