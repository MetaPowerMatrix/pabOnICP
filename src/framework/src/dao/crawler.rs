use anyhow::{anyhow, Error};
use serde::Serialize;
use crate::DataResponse;
use super::http::MetaPowerSvcClient;

#[derive(Serialize)]
pub struct PathInfo {
    absolute_path: String,
}

pub async fn download_image(id: String, file_path: &str) -> Result<String, Error> {
    let req = PathInfo{
        absolute_path: file_path.to_string(),
    };

    match MetaPowerSvcClient::default().metapower_proxy_post::<PathInfo, DataResponse>(&format!("download/ai/resource/{}", id), req).await{
        Ok(resp) => {
            if resp.code == "200" {
                return Ok(resp.content);
            }
        }
        Err(e) => {
            return Err(e);
        }
    }

    Err(anyhow!("download failure!"))
}
