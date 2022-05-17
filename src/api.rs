use std::fmt::{self, Display, Formatter};

use anyhow::{anyhow, bail, Result};
use reqwest::redirect::Policy;

pub mod list;
pub mod model;

pub struct Client {
    http: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Self> {
        Ok(Self {
            http: reqwest::ClientBuilder::new()
                .redirect(Policy::none())
                .build()?,
        })
    }

    pub async fn with_login(username: &str, password: &str) -> Result<Self> {
        let client = Self::new()?;
        client.login(username, password).await?;
        Ok(client)
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<()> {
        let resp = self
            .http
            .post("https://vpn.inf.shizuoka.ac.jp/dana-na/auth/url_3/login.cgi")
            .form(&[
                ("tz_offset", "540"),
                ("username", username),
                ("password", password),
                ("realm", "Student-Realm"),
                ("btnSubmit", "Sign+In"),
            ])
            .send()
            .await?;

        let location = match resp.headers().get("location") {
            Some(l) => l,
            None => {
                dbg!(resp.text().await?);
                return Err(anyhow!("location was expected"));
            }
        };

        match location.to_str()? {
            "/dana/home/index.cgi" => Ok(()),
            "/dana-na/auth/url_3/welcome.cgi?p=failed" => {
                Err(anyhow!("Invalid username or password"))
            }
            // TODO: セッションを選択させる
            _ => Err(anyhow!("Session Error")),
        }
    }
}

pub enum VolumeID {
    FSShare,
    FS(String),
}

impl VolumeID {
    pub fn from_str(name: &str) -> Result<Self> {
        if name == "fsshare" {
            Ok(Self::FSShare)
        } else if name.starts_with("fs") {
            let tokens = name.split('/').collect::<Vec<_>>();
            let fs_prefix = tokens
                .get(2)
                .ok_or_else(|| anyhow!("invalid fs volume format. example: -v fs/2020"))?;
            Ok(Self::FS("fs/".to_owned() + *fs_prefix))
        } else {
            bail!("No such volume {}", name);
        }
    }
}

impl Display for VolumeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VolumeID::FSShare => "fsshare",
                VolumeID::FS(s) => s,
            }
        )
    }
}
