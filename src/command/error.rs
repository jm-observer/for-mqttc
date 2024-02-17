use serde::Serialize;

/// anyhow::Error not impl Serialize
#[derive(Serialize)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn init_rs<T>(msg: impl Into<String>) -> Result<T, Self> {
        Err(Error { msg: msg.into() })
    }
    pub fn init(msg: impl Into<String>) -> Self {
        Error { msg: msg.into() }
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}
impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}
