use anyhow::{anyhow, Error};
use serde::Serialize;
use crate::DataResponse;
use super::http::MetaPowerSvcClient;

#[derive(Serialize)]
pub struct PathInfo {
    absolute_path: String,
    saved_name: String,
}

pub async fn download_image(id: String, file_path: &str, saved_name: String) -> Result<String, Error> {
    let req = PathInfo{
        absolute_path: file_path.to_string(),
        saved_name,
    };

    match MetaPowerSvcClient::default().metapower_proxy_post::<PathInfo, DataResponse>(&format!("/api/download/ai/resource/{}", id), req).await{
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
