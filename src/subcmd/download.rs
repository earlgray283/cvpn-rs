use crate::api::{model::volume_id::VolumeID, Client};
use anyhow::{bail, Result};
use futures::future::join_all;
use std::{fs::File, io::Write, path::PathBuf, sync::Arc};

pub async fn download(
    client: Client,
    pathes: Vec<PathBuf>,
    volume_name: &str,
    output_dir: PathBuf,
) -> Result<()> {
    let volume_id_arc = Arc::new(VolumeID::from_str(volume_name)?);
    let mut handles = vec![];
    let client_arc = Arc::new(client);
    let output_dir_str = output_dir.to_str().unwrap().to_string();
    let output_dir_str_arc = Arc::new(output_dir_str);

    for mut path in pathes {
        let client = client_arc.clone();
        let output_dir_str = output_dir_str_arc.clone();
        let volume_id = volume_id_arc.clone();
        let handle = tokio::spawn(async move {
            let filename = match path.file_name() {
                Some(osstr) => osstr.to_str().unwrap().to_owned(),
                None => bail!("path must be filepath"),
            };
            path.pop();

            let bytes = client.download(path, &filename, &volume_id).await?;
            let mut f = match File::create(format!("{}/{}", output_dir_str, &filename)) {
                Ok(f) => f,
                Err(_e) => todo!("if file has already existed..."),
            };
            f.write_all(&bytes)?;
            Ok(())
        });
        handles.push(handle);
    }

    join_all(handles).await;

    Ok(())
}
