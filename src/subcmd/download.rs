use crate::api::{model::volume_id::VolumeID, Client};
use anyhow::{bail, Result};
use std::{fs::File, io::Write, path::PathBuf};

pub async fn download(
    client: Client,
    mut path: PathBuf,
    volume_name: &str,
    mut output: PathBuf,
) -> Result<()> {
    let volume_id = VolumeID::from_str(volume_name)?;
    let filename = match path.file_name() {
        Some(osstr) => osstr.to_str().unwrap().to_owned(),
        None => bail!("path must be filepath"),
    };
    path.pop();

    let bytes = client.download(path, &filename, &volume_id).await?;
    output.push(filename);
    let mut f = match File::create(&output) {
        Ok(f) => f,
        Err(_e) => todo!("if file has already existed..."),
    };
    if f.write(&bytes)? == 0 {
        bail!("written 0 byte");
    }
    Ok(())
}
