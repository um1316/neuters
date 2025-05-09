use std::io;

#[derive(Debug)]
pub enum ApiError {
    Redirect(u16, String),
    External(u16, String),
    Internal(String),
    Empty,
}

pub type ApiResult<T> = Result<T, ApiError>;

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        Self::Internal(format!("Failed to deserialize API response: {e}"))
    }
}

impl From<ureq::Error> for ApiError {
    fn from(error: ureq::Error) -> Self {
        match error {
            ureq::Error::Status(code, response) => {
                if (300..400).contains(&code) {
                    let target = response
                        .header("Location")
                        .unwrap_or("/");
                    Self::Redirect(
                        code,
                        target.to_string(),
                    )
                } else {
                    Self::External(
                        code,
                        response
                            .into_string()
                            .unwrap_or_else(|_| "failed to parse response".to_owned()),
                    )
                }
            }
            ureq::Error::Transport(err) => Self::Internal(
                err.message()
                    .unwrap_or("failed to parse response")
                    .to_owned(),
            ),
        }
    }
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> Self {
        Self::Internal(format!("IO error: {}", err))
    }
}
