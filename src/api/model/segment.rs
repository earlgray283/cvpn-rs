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

pub struct Size {
    size: f64,
    unit: Unit,
}

impl Size {
    pub fn new(size: f64, unit: Unit) -> Self {
        Self { size, unit }
    }
}

pub enum Unit {
    B,
    KB,
    MB,
    GB,
    TB,
}

impl Unit {
    pub fn from_str(s: &str) -> Self {
        match &*s {
            "B" => Unit::B,
            "KB" => Unit::KB,
            "MB" => Unit::MB,
            "GB" => Unit::GB,
            "TB" => Unit::TB,
            _ => Unit::B,
        }
    }
}
