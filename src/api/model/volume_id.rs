use anyhow::{anyhow, bail, Result};
use std::fmt::{self, Display, Formatter};

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
