use poem::error::ResponseError;
use poem::{Body, http::StatusCode};
use poem_openapi::payload::{Json, Payload};
use poem_openapi::registry::{MetaMediaType, MetaResponse, MetaResponses, Registry};
use poem_openapi::{ApiResponse, Object};
use sea_orm::DbErr;
use std::fmt::Display;

#[derive(Debug, Object, thiserror::Error)]
#[oai(read_only_all)]
pub(crate) struct Error {
    pub(crate) code: u16,
    pub(crate) message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl ResponseError for Error {
    fn status(&self) -> StatusCode {
        StatusCode::from_u16(self.code).unwrap()
    }

    fn as_response(&self) -> poem::Response {
        let body = Body::from_json(serde_json::json!({
            "code": self.code,
            "message": self.to_string(),
        }))
        .unwrap();
        poem::Response::builder().status(self.status()).body(body)
    }
}

impl From<DbErr> for Error {
    fn from(v: DbErr) -> Self {
        match v {
            DbErr::RecordNotFound(msg) => Self::from(StatusCode::NOT_FOUND, msg),
            DbErr::RecordNotInserted | DbErr::RecordNotUpdated => {
                Self::from(StatusCode::CONFLICT, v.to_string())
            }
            _ => Self::from(StatusCode::INTERNAL_SERVER_ERROR, v.to_string()),
        }
    }
}

impl Error {
    pub(crate) fn from(c: StatusCode, msg: impl Into<String>) -> Self {
        Self {
            code: c.as_u16(),
            message: msg.into(),
        }
    }
}

impl ApiResponse for Error {
    fn meta() -> MetaResponses {
        MetaResponses {
            responses: vec![MetaResponse {
                description: "<error description>",
                status: None,
                status_range: Some("4XX/5XX".to_owned()),
                content: vec![MetaMediaType {
                    content_type: Json::<Error>::CONTENT_TYPE,
                    schema: Json::<Error>::schema_ref(),
                }],
                headers: vec![],
            }],
        }
    }

    fn register(registry: &mut Registry) {
        <Json<Error> as poem_openapi::ApiResponse>::register(registry)
    }
}
