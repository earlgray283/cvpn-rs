use super::size::Size;
use chrono::NaiveDateTime;
use std::path::PathBuf;

pub struct Segment {
    pub name: String,
    pub path: PathBuf,
    pub size: Option<Size>,
    pub volume_id: String,
    pub uploaded_at: NaiveDateTime,
    pub is_file: bool,
    pub is_dir: bool,
}

impl Segment {
    pub fn from_file(
        name: String,
        path: PathBuf,
        size: Size,
        volume_id: String,
        uploaded_at: NaiveDateTime,
    ) -> Self {
        Self {
            name,
            path,
            size: Some(size),
            volume_id,
            uploaded_at,
            is_file: true,
            is_dir: false,
        }
    }

    pub fn from_dir(
        name: String,
        path: PathBuf,
        volume_id: String,
        uploaded_at: NaiveDateTime,
    ) -> Self {
        Self {
            name,
            path,
            size: None,
            volume_id,
            uploaded_at,
            is_file: false,
            is_dir: true,
        }
    }
}

const DATETIME_FORMAT: &str = "%c";
const SHOW_ICON: bool = true;

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_file {
            write!(
                f,
                "{}  {}  {} {}",
                self.size.as_ref().unwrap(),
                self.uploaded_at.format(DATETIME_FORMAT),
                if SHOW_ICON { "" } else { "" },
                self.name,
            )
        } else {
            write!(
                f,
                "{:>10}  {}  {} {}",
                "-",
                self.uploaded_at.format(DATETIME_FORMAT),
                if SHOW_ICON { "" } else { "" },
                self.name,
            )
        }
    }
}
