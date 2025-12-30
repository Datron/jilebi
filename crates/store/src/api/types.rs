use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

pub const BUCKET_NAME: &str = "JILEBI";

#[derive(Debug, strum_macros::Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum FileType {
    Plugins,
    Bin,
    Templates,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadStat {
    pub id: i64,
    pub name: String,
    pub count: i64,
    pub r#type: FileType,
}

#[derive(Deserialize)]
pub struct PluginQuery {
    #[serde(default)]
    pub version: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    PluginNotFound(&'static str),
    BinaryNotFound(&'static str),
    TemplateNotFound(&'static str),
    BadRequest(String),
    D1Error(String, worker::Error),
    R2Error(String, worker::Error),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("An error occurred: {:#?}", self);
        match self {
            AppError::PluginNotFound(s)
            | AppError::BinaryNotFound(s)
            | AppError::TemplateNotFound(s) => {
                (axum::http::StatusCode::NOT_FOUND, s).into_response()
            }
            AppError::BadRequest(message) => {
                (axum::http::StatusCode::BAD_REQUEST, message).into_response()
            }
            AppError::D1Error(message, _) | AppError::R2Error(message, _) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
            AppError::InternalError(message) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        }
    }
}
