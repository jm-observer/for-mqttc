use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    msg: String,
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

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}
