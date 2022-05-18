use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

pub fn save_cookies(cookies: &Vec<String>) -> Result<()> {
    let mut cache_dir = cache_dir().ok_or_else(|| anyhow!("could not find cache dir"))?;
    cache_dir.push("cookies.txt");

    let mut f = File::create(cache_dir)?;
    for cookie in cookies {
        writeln!(f, "{}", cookie)?;
    }
    Ok(())
}

pub fn load_cookies() -> Result<Vec<String>> {
    let mut cache_dir = cache_dir().ok_or_else(|| anyhow!("could not find cache dir"))?;
    cache_dir.push("cookies.txt");

    let f = File::open(cache_dir)?;
    let mut r = BufReader::new(f);
    let mut cookies = Vec::new();
    loop {
        let mut buf = String::new();
        if let Ok(n) = r.read_line(&mut buf) {
            if n == 0 {
                break;
            }
            cookies.push(buf.trim().to_string());
        } else {
            break;
        }
    }
    Ok(cookies)
}

#[cfg(target_os = "windows")]
pub fn user_config_dir() -> Option<PathBuf> {}

#[cfg(target_os = "linux")]
pub fn user_config_dir() -> Option<PathBuf> {}

#[cfg(target_os = "macos")]
pub fn cache_dir() -> Option<PathBuf> {
    use std::{env, fs::create_dir_all, str::FromStr};
    let home = match env::var_os("HOME") {
        Some(home) => home,
        None => return None,
    };
    let mut p = PathBuf::from_str(home.to_str().unwrap()).unwrap();
    p.push("Library/Caches/cvpn-rs");
    create_dir_all(&p).unwrap();
    Some(p)
}
