use anyhow::{anyhow, Error};
use crate::DataResponse;
use super::http::BSCSvcClient;

pub async fn download_image(id: String, file_path: &str) -> Result<String, Error> {

    match BSCSvcClient::default().bsc_proxy_post::<String, DataResponse>(&format!("download/image/{}", id), file_path.to_string()).await{
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
