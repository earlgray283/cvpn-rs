use crate::appdata::{load_cookies, save_cookies};
use anyhow::{anyhow, bail, Result};
use reqwest::{header::HeaderMap, redirect::Policy, ClientBuilder, StatusCode};
use scraper::{Html, Selector};
use thiserror::Error;

pub mod download;
pub mod list;
pub mod model;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid username or password")]
    InvalidUsernameOrPassword,
    #[error("{0} was not found")]
    AttrValueNotFound(String),
    #[error("Response status was not {0}")]
    InvalidResponseStatus(StatusCode),
    #[error("unknown error")]
    Unknown,
}

pub struct Client {
    http: reqwest::Client,
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
                return Self::with_login(username, password).await;
            }
        };

        let mut header = HeaderMap::new();
        header.insert("cookie", cookies.join("; ").parse().unwrap());
        let http = default_http_builder().default_headers(header).build()?;
        let client = Self { http };
        if let Err(_e) = client.check_cookies().await {
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
            .ok_or_else(|| Error::AttrValueNotFound("xsauth".to_string()))?
            .value()
            .attr("value")
            .ok_or_else(|| Error::AttrValueNotFound("xsauth".to_string()))?;
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
            bail!(Error::InvalidResponseStatus(StatusCode::FOUND))
        }

        match resp.headers().get("location").unwrap().to_str()? {
            "/dana/home/index.cgi" => {
                let cookies = resp
                    .cookies()
                    .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
                    .collect::<Vec<_>>();
                save_cookies(&cookies)?;
                Ok(())
            }
            "/dana-na/auth/url_3/welcome.cgi?p=failed" => {
                Err(anyhow!(Error::InvalidUsernameOrPassword))
            }
            _ => {
                let html = Html::parse_document(resp.text().await?.as_str());
                let form_data_str = html
                    .select(&Selector::parse("#DSIDFormDataStr").unwrap())
                    .next()
                    .ok_or_else(|| Error::AttrValueNotFound("DSIDFormDataStr".to_string()))?
                    .value()
                    .attr("value")
                    .ok_or_else(|| Error::AttrValueNotFound("DSIDFormDataStr".to_string()))?;
                self.continue_current_session(form_data_str).await
            }
        }
    }

    pub async fn continue_current_session(&self, form_data_str: &str) -> Result<()> {
        let resp = self
            .http
            .post("https://vpn.inf.shizuoka.ac.jp/dana-na/auth/url_3/login.cgi")
            .form(&[
                ("btnContinue", "セッションを続行します"),
                ("FormDataStr", form_data_str),
            ])
            .send()
            .await?;
        if resp.status() != StatusCode::FOUND {
            bail!(Error::InvalidResponseStatus(StatusCode::FOUND))
        }

        match resp.headers().get("location").unwrap().to_str()? {
            "/dana/home/index.cgi" => {
                let cookies = resp
                    .cookies()
                    .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
                    .collect::<Vec<_>>();
                save_cookies(&cookies)?;
                Ok(())
            }
            _ => Err(anyhow!(Error::Unknown)),
        }
    }
}

fn default_http_builder() -> ClientBuilder {
    reqwest::ClientBuilder::new()
        .redirect(Policy::none())
        .cookie_store(true)
}
