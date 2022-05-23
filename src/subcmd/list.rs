use crate::api::{
    model::{size::Size, volume_id::VolumeID},
    Client,
};
use anyhow::Result;
use std::{
    io::{stdout, Write},
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug)]
pub enum Sort {
    Size,
    Date,
    Name,
    None,
}

impl FromStr for Sort {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s {
            "size" => Ok(Self::Size),
            "date" => Ok(Self::Date),
            "name" => Ok(Self::Name),
            "none" => Ok(Self::None),
            _ => Err("{size, date, name, default}"),
        }
    }
}

pub async fn list(
    client: Client,
    path: PathBuf,
    volume_name: &str,
    sort_by: Sort,
    name_only: bool,
) -> Result<()> {
    let volume_id = VolumeID::from_str(volume_name)?;
    let mut segments = client.list(path, &volume_id).await?;

    match sort_by {
        Sort::Date => segments.sort_by_key(|segment| segment.uploaded_at),
        Sort::Name => segments.sort_by(|l, r| l.name.cmp(&r.name)),
        Sort::Size => segments.sort_by(|l, r| {
            l.size
                .as_ref()
                .unwrap_or(&Size::zero())
                .partial_cmp(r.size.as_ref().unwrap_or(&Size::zero()))
                .unwrap()
        }),
        _ => (),
    }

    let mut output = String::new();
    for segment in segments {
        if name_only {
            output.push_str(&(segment.path.to_str().unwrap().to_string() + "\n"))
        } else {
            output.push_str(&(segment.to_string() + "\n"));
        }
    }
    print!("{}", output);
    stdout().flush()?;
    Ok(())
}
