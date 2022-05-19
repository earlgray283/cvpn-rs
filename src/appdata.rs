use crate::api::Client;
use anyhow::{anyhow, Result};
use spinners::{Spinner, Spinners};
use std::{
    env, error,
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Write},
    path::PathBuf,
    str::FromStr,
};

pub async fn setup() -> Result<(String, String)> {
    eprintln!("You seem to login for the first time. Please input your account information.");
    Ok(loop {
        let username: String = loop {
            let u = input_with_prompt::<String>("username: ").unwrap();
            if !u.is_empty() {
                break u;
            }
        };
        let password: String = loop {
            print!("password: ");
            stdout().flush().unwrap();
            let p = rpassword::read_password().unwrap();
            if !p.is_empty() {
                break p;
            }
        };

        let mut sp = Spinner::new(Spinners::Dots9, "Waiting for login...".to_string());
        match Client::with_login(&username, &password).await {
            Ok(_) => {
                sp.stop_with_newline();
                save_account_info(&username, &password)?;
                break (username, password);
            }
            Err(_) => {
                sp.stop_with_newline();
                eprintln!("Failed to login. Maybe Username or Password is invalid");
            }
        }
    })
}

pub fn save_account_info(username: &str, password: &str) -> Result<()> {
    let mut config_dir = config_dir().ok_or_else(|| anyhow!("could not find config dir"))?;
    config_dir.push(".env");
    let mut f = File::create(config_dir)?;
    writeln!(f, "CVPN_USERNAME={}", username)?;
    writeln!(f, "CVPN_PASSWORD={}", password)?;
    Ok(())
}

pub fn load_account_info() -> Result<(String, String)> {
    let mut config_dir = config_dir().ok_or_else(|| anyhow!("could not find config dir"))?;
    config_dir.push(".env");
    dotenv::from_filename(config_dir)?;
    Ok((env::var("CVPN_USERNAME")?, env::var("CVPN_PASSWORD")?))
}

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
    use std::fs::create_dir_all;
    let home = match env::var_os("HOME") {
        Some(home) => home,
        None => return None,
    };
    let mut p = PathBuf::from_str(home.to_str().unwrap()).unwrap();
    p.push("Library/Caches/cvpn-rs");
    create_dir_all(&p).unwrap();
    Some(p)
}

#[cfg(target_os = "macos")]
pub fn config_dir() -> Option<PathBuf> {
    use std::fs::create_dir_all;
    let home = match env::var_os("HOME") {
        Some(home) => home,
        None => return None,
    };
    let mut p = PathBuf::from_str(home.to_str().unwrap()).unwrap();
    p.push("Library/Application Support/cvpn-rs");
    create_dir_all(&p).unwrap();
    Some(p)
}

fn input_with_prompt<T: FromStr>(prompt: &str) -> Result<T, Box<dyn error::Error>>
where
    <T as FromStr>::Err: std::error::Error,
{
    print!("{}", prompt);
    stdout().flush()?;
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.trim().parse().map_err(|_e| "")?)
}
