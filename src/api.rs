use crate::appdata::{load_cookies, save_cookies};
use anyhow::{anyhow, bail, Result};
use reqwest::{header::HeaderMap, redirect::Policy, ClientBuilder, StatusCode};
use scraper::{Html, Selector};
use std::fmt::{self, Display, Formatter};

pub mod list;
pub mod model;

pub struct Client {
    http: reqwest::Client,
}

fn default_http_builder() -> ClientBuilder {
    reqwest::ClientBuilder::new()
        .redirect(Policy::none())
        .cookie_store(true)
}

impl Client {
    pub fn new() -> Result<Self> {
        Ok(Self {
            http: default_http_builder().build()?,
        })
    }

    pub async fn with_login(username: &str, password: &str) -> Result<Self> {
        let client = Self::new()?;
        client.login(username, password).await?;
        Ok(client)
    }

    /// make client with token.
    /// if token is invalid, make client with login.
    pub async fn with_token_or_login(username: &str, password: &str) -> Result<Self> {
        let cookies = match load_cookies() {
            Ok(cookies) => cookies,
            Err(_) => {
                dbg!("local cookies were not found");
                return Self::with_login(username, password).await;
            }
        };

        let mut header = HeaderMap::new();
        header.insert("cookie", cookies.join("; ").parse().unwrap());
        let http = default_http_builder().default_headers(header).build()?;
        let client = Self { http };
        if let Err(_e) = client.check_cookies().await {
            dbg!("cookies were invalid");
            Self::with_login(username, password).await
        } else {
            Ok(client)
        }
    }

    pub async fn check_cookies(&self) -> Result<()> {
        let resp = self
            .http
            .get("https://vpn.inf.shizuoka.ac.jp/dana/home/index.cgi")
            .send()
            .await?;
        Html::parse_document(resp.text().await?.as_str())
            .select(&Selector::parse("#xsauth_395").unwrap())
            .next()
            .ok_or_else(|| anyhow!("#xsauth_395 was not found"))?
            .value()
            .attr("value")
            .ok_or_else(|| anyhow!("xsauth not found"))?;
        Ok(())
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
        if resp.status() != StatusCode::FOUND {
            bail!("login: Response status was not 302")
        }

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
        }?;

        let cookies = resp
            .cookies()
            .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
            .collect::<Vec<_>>();

        dbg!(&cookies);
        save_cookies(&cookies)?;

        dbg!(resp.text().await?);

        Ok(())
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
                VolumeID::FSShare => "resource_1423533946.487706.3",
                VolumeID::FS(s) => s,
            }
        )
    }
}
