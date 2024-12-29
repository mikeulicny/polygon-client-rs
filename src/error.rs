pub enum Error {
    ReqwestError,
    SerdeError
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        return Self::ReqwestError;
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        return Self::SerdeError;
    }
}
