use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{self};
use anyhow::{anyhow, Error};
use crate::{AnswerReply, BestTalkRequest, SomeDocs};

const HTTP_CYCLE_COST: u128 = 1491400000;
const METAPOWER_PROXY_HOST: &str = "api.metapowermatrix.ai";
const BSC_PROXY_HOST: &str = "icp.metapowermatrix.ai";
const LLM_PROXY_HOST: &str = "llm.metapowermatrix.ai";
const LLM_REQUEST_PROTOCOL: &str = "https://";
const ICP_CLIENT: &str = "battery_icp";

// This struct is legacy code and is not really used in the code.
#[derive(Serialize, Deserialize)]
struct Context {
    bucket_start_time_index: usize,
    closing_price_index: usize,
}

pub async fn send_http_get_request<T: Serialize>(host: String, path: String, module: String, query: Option<T>) -> Result<String, String> {
    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: host.clone(),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: format!("metapowermatrix_{}_canister", module),
        },
    ];


    let context = Context {
        bucket_start_time_index: 0,
        closing_price_index: 4,
    };

    let url = if query.is_none() {
        format!("{}{}{}", LLM_REQUEST_PROTOCOL, host, path)
    } else {
        let query_string = serde_qs::to_string(&query).unwrap();
        format!("{}{}{}?{}", LLM_REQUEST_PROTOCOL, host, path, query_string)
    };

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,               //optional for request
        max_response_bytes: Some(4096), //optional for request
        // transform: None,          //optional for request
        transform: Some(TransformContext::from_name("transform".to_string(), serde_json::to_vec(&context).unwrap())),
        headers: request_headers,
    };

    match http_request(request, HTTP_CYCLE_COST).await {
        Ok((response,)) => {
            Ok(String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded."))
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            //Return the error as a string and end the method
            Err(message)
        }
    }
}

//Update method using the HTTPS outcalls feature
pub async fn send_http_post_request(host: String, path: String, module: String, json_string: String) -> Result<String, String> {
    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: host.clone(),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: format!("metapowermatrix_{}_canister", module),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
    ];

    let json_utf8: Vec<u8> = json_string.into_bytes();
    let request_body: Option<Vec<u8>> = Some(json_utf8);

    let context = Context {
        bucket_start_time_index: 0,
        closing_price_index: 4,
    };

    let url = format!("{}{}{}", LLM_REQUEST_PROTOCOL, host, path);
    let request = CanisterHttpRequestArgument {
        url,
        max_response_bytes: Some(8192), //optional for request
        method: HttpMethod::POST,
        headers: request_headers,
        body: request_body,
        transform: None,
        // transform: Some(TransformContext::from_name("transform".to_string(), serde_json::to_vec(&context).unwrap())),
    };

    match http_request(request, HTTP_CYCLE_COST).await {
        Ok((response,)) => {
            let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");
            ic_cdk::api::print(format!("{:?}", str_body));

            Ok(str_body)
        }
        Err((r, m)) => {
            let message =
                format!("RejectionCode: {r:?}, Error: {m}");

            //Return the error as a string and end the method
            Err(message)
        }
    }
}

pub fn transform(raw: TransformArgs) -> HttpResponse {

    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];
    

    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
    };

    if i32::try_from(res.status.clone().0).unwrap() == 200{
        res.body = raw.response.body;
    } else {
        ic_cdk::api::print(format!("Received an error: err = {:?}", raw));
    }
    res
}

pub struct MetaPowerSvcClient {
    pub host: String,
    pub module: String,
}

impl Default for MetaPowerSvcClient {
    fn default() -> Self {
        MetaPowerSvcClient::new(METAPOWER_PROXY_HOST.to_string(), ICP_CLIENT.to_string())
    }
}

impl MetaPowerSvcClient {
    pub fn new(host: String, module: String) -> Self {
        MetaPowerSvcClient { host, module }
    }
    pub async fn metapower_proxy_post<T: serde::Serialize, R: DeserializeOwned>(&self, url: &str, req: T) -> Result<R, Error> {
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
    pub async fn metapower_proxy_get<T: serde::Serialize, R: DeserializeOwned>(&self, url: &str, query: Option<T>) -> Result<R, Error> {
        match send_http_get_request(self.host.clone(), url.to_string(), self.module.clone(), query).await {
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
pub struct BSCSvcClient {
    pub host: String,
    pub module: String,
}

impl Default for BSCSvcClient {
    fn default() -> Self {
        BSCSvcClient::new(BSC_PROXY_HOST.to_string(), ICP_CLIENT.to_string())
    }
}

impl BSCSvcClient {
    pub fn new(host: String, module: String) -> Self {
        BSCSvcClient { host, module }
    }
    pub async fn bsc_proxy_post<T: serde::Serialize, R: DeserializeOwned>(&self, url: &str, req: T) -> Result<R, Error> {
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
    pub async fn bsc_proxy_get<T: serde::Serialize, R: DeserializeOwned>(&self, url: &str, query: Option<T>) -> Result<R, Error> {
        match send_http_get_request(self.host.clone(), url.to_string(), self.module.clone(), query).await {
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
pub struct LLMSvcClient {
    pub host: String,
    pub module: String,
}

impl Default for LLMSvcClient {
    fn default() -> Self {
        LLMSvcClient::new(LLM_PROXY_HOST.to_string(), ICP_CLIENT.to_string())
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
