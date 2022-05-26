use super::{model::volume_id::VolumeID, Client};
use anyhow::{bail, Result};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS, NON_ALPHANUMERIC};
use reqwest::{StatusCode, Url};
use std::{path::PathBuf, str::FromStr};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Response status was not {0}")]
    InvalidResponseStatus(StatusCode),
    #[error("The file could not be found. The argument `path` or `volume` may be wrong.")]
    NotFound,
}

const FILENAME_ASCIISET: &AsciiSet = &CONTROLS.add(b'+');

impl Client {
    pub async fn download<P: Into<PathBuf>>(
        &self,
        dirp: P,
        filename: &str,
        volume_id: &VolumeID,
    ) -> Result<Vec<u8>> {
        let dir: PathBuf = dirp.into();
        let filename = utf8_percent_encode(filename, FILENAME_ASCIISET);
        let url = Url::from_str(
            &format!("https://vpn.inf.shizuoka.ac.jp/dana/download/{}?url=/dana-cached/fb/smb/wfv.cgi?t=p&v={}&si=&ri=&pi=&ignoreDfs=1&dir={}&file={}",
                filename,
                volume_id.to_string().as_str(),
                utf8_percent_encode(dir.to_str().unwrap().trim_matches('/').replace('/', "\\").as_str(), NON_ALPHANUMERIC),
                filename,
            ),
        )?;

        let resp = self.http.get(url).send().await?;
        if resp.status() != StatusCode::OK {
            bail!(Error::InvalidResponseStatus(StatusCode::OK))
        }

        let is_html = resp
            .headers()
            .get("Content-Type")
            .unwrap()
            .to_str()?
            .contains("html");
        if is_html {
            const MESSAGE_NOT_FOUND: &str = "The file or folder does not exist on the server.";
            const MESSAGE_PERMISSION_DENIED: &str =
                "You do not have permission to access this file server.";
            let content = resp.bytes().await?.to_vec();
            let content_html = String::from_utf8_lossy(&content);
            if content_html.contains(MESSAGE_NOT_FOUND) {
                bail!(Error::NotFound)
            } else if content_html.contains(MESSAGE_PERMISSION_DENIED) {
                bail!(Error::PermissionDenied)
            } else {
                return Ok(content.to_vec());
            }
        }

        Ok(resp.bytes().await?.to_vec())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        api::{model::volume_id::VolumeID, Client},
        appdata::load_account_info,
    };

    #[tokio::test]
    pub async fn download() {
        let (u, p) = load_account_info().unwrap();
        let c = Client::with_token_or_login(&u, &p).await.unwrap();
        let bytes = c
            .download(
                "/class/2022記号処理",
                "symbol2022-6.pptx",
                &VolumeID::FSShare,
            )
            .await
            .unwrap();
        assert!(!bytes.is_empty());
    }
}
