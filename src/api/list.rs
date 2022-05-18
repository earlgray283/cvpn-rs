use super::{
    model::segment::{Segment, Size, Unit},
    Client,
};
use anyhow::{bail, Result};
use chrono::NaiveDateTime;
use reqwest::{StatusCode, Url};
use scraper::{Html, Selector};
use std::{path::PathBuf, str::FromStr};

impl Client {
    pub async fn list<P: Into<PathBuf>>(&self, p: P, volume_id: &str) -> Result<Vec<Segment>> {
        let mut u = Url::from_str("https://vpn.inf.shizuoka.ac.jp/dana/fb/smb/wfb.cgi").unwrap();
        let path: PathBuf = p.into();

        u.query_pairs_mut()
            .append_pair("t", "p")
            .append_pair("v", volume_id)
            .append_pair("si", "0")
            .append_pair("ri", "0")
            .append_pair("pi", "0")
            .append_pair("sb", "name")
            .append_pair("so", "asc")
            .append_pair("dir", path.to_str().unwrap());
        dbg!(u.as_str());
        let resp = self.http.get(u).send().await?;
        if resp.status() != StatusCode::OK {
            if resp.status() == StatusCode::FOUND {
                bail!("Permission denied");
            } else {
                bail!("Undefined error");
            }
        }

        let doc = Html::parse_document(resp.text().await?.as_str());
        let elem = doc
            .select(&Selector::parse("table#table_wfb_5 > tbody > script").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let lines = elem.split(';').collect::<Vec<_>>();

        let mut segments = Vec::new();
        for &line in &lines {
            if line.is_empty() {
                break;
            }
            let tokens = line[2..=line.len() - 2]
                .split(',')
                .map(|s| s.trim_matches('\"'))
                .collect::<Vec<_>>();

            if tokens.len() == 3 {
                segments.push(Segment::from_dir(
                    tokens[0].to_string(),
                    path.join(tokens[0]),
                    tokens[2].to_string(),
                    NaiveDateTime::from_str(tokens[3])?,
                ))
            } else {
                let size_tokens = tokens[3].split("&nbsp;").collect::<Vec<_>>();
                if size_tokens.len() != 2 {
                    bail!("The size of size_tokens must be 2");
                }
                segments.push(Segment::from_file(
                    tokens[0].to_string(),
                    path.join(tokens[0]),
                    Size::new(size_tokens[0].parse()?, Unit::from_str(size_tokens[1])),
                    tokens[2].to_string(),
                    NaiveDateTime::from_str(tokens[3])?,
                ))
            }
        }

        Ok(segments)
    }
}