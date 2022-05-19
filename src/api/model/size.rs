#[derive(PartialEq, PartialOrd)]
pub struct Size {
    pub size: f64,
    pub unit: Unit,
}

impl Size {
    pub fn new(size: f64, unit: Unit) -> Self {
        Self { size, unit }
    }

    pub fn zero() -> Self {
        Self::new(0.0, Unit::B)
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:6.2}[{}]", self.size, self.unit)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::B => "B",
                Self::KB => "KB",
                Self::MB => "MB",
                Self::GB => "GB",
                Self::TB => "TB",
            }
        )
    }
}
