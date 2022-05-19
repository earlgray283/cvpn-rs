use super::{
    model::{
        segment::Segment,
        size::{Size, Unit},
        volume_id::VolumeID,
    },
    Client,
};
use anyhow::{bail, Result};
use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::{StatusCode, Url};
use scraper::{Html, Selector};
use std::{path::PathBuf, str::FromStr};

const SEGMENTS_CAPASITY: usize = 256;
const DATETIME_FORMAT: &str = "%a %b  %d %H:%M:%S %Y";
static DIR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"d\("(.+)","(.+)","(.+)"\);"#).unwrap());
static FILE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"f\("(.+)","(.+)","(.+)","(.+)"\);"#).unwrap());

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("cannot fetch directory '{0}': Permission denied")]
    PermissionDenied(String),
    #[error("unknown error")]
    Unknown,
}

impl Client {
    pub async fn list<P: Into<PathBuf>>(&self, p: P, volume_id: &VolumeID) -> Result<Vec<Segment>> {
        let mut u = Url::from_str("https://vpn.inf.shizuoka.ac.jp/dana/fb/smb/wfb.cgi").unwrap();
        let path: PathBuf = p.into();

        u.query_pairs_mut()
            .append_pair("t", "p")
            .append_pair("v", volume_id.to_string().as_str())
            .append_pair("si", "0")
            .append_pair("ri", "0")
            .append_pair("pi", "0")
            .append_pair("sb", "name")
            .append_pair("so", "asc")
            .append_pair("dir", path.to_str().unwrap());
        let resp = self.http.get(u).send().await?;
        if resp.status() != StatusCode::OK {
            if resp.status() == StatusCode::FOUND {
                bail!(Error::PermissionDenied(path.to_str().unwrap().to_string()))
            } else {
                bail!(Error::Unknown)
            }
        }

        let doc = Html::parse_document(resp.text().await?.as_str());
        let elem = doc
            .select(&Selector::parse("table#table_wfb_5 > tbody > script").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let lines = elem.split('\n').collect::<Vec<_>>();

        let mut segments = Vec::with_capacity(SEGMENTS_CAPASITY);
        for &line in &lines {
            if line.is_empty() {
                continue;
            }

            if let Some(tokens) = DIR_REGEX.captures_iter(line).next() {
                segments.push(Segment::from_dir(
                    tokens[1].to_string(),
                    path.join(&tokens[1]),
                    volume_id.to_string(),
                    NaiveDateTime::parse_from_str(&tokens[3], DATETIME_FORMAT)?,
                ))
            }
            if let Some(tokens) = FILE_REGEX.captures_iter(line).next() {
                let size_tokens = tokens[3].split("&nbsp;").collect::<Vec<_>>();
                segments.push(Segment::from_file(
                    tokens[1].to_string(),
                    path.join(&tokens[1]),
                    Size::new(size_tokens[0].parse()?, Unit::from_str(size_tokens[1])),
                    volume_id.to_string(),
                    NaiveDateTime::parse_from_str(&tokens[4], DATETIME_FORMAT)?,
                ))
            }
        }

        Ok(segments)
    }
}
