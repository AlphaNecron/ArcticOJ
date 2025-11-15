use super::prelude::*;
use crate::hash;
use poem_openapi::auth::{ApiKey};
use poem_openapi::SecurityScheme;

pub(super) struct Endpoints;

// region APIKeyAuth
#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_in = "header",
    key_name = "X-API-Key",
    checker = "api_key_checker"
)]
struct ApiKeyAuth(user::Model);

async fn api_key_checker(req: &Request, key: ApiKey) -> Option<user::Model> {
    None
}
// endregion

// region Login
#[derive(Object)]
struct Credentials {
    /// Either email or handle
    handle: String,
    password: Password,
}

#[derive(ApiResponse)]
enum LoginResp {
    #[oai(status = 200)]
    Ok(PlainText<String>),
    /// Invalid credentials
    #[oai(status = 400)]
    Bad,
    /// Internal error (e.g., DBErr)
    #[oai(status = 500)]
    InternalError,
}
// endregion

// region Register
#[derive(Object)]
#[oai(rename_all = "camelCase")]
struct RegisterReq {
    display_name: Option<String>,
    handle: String,
    email: Email,
    password: Password,
}

#[derive(ApiResponse)]
enum RegisterResp {
    /// Returns ID of inserted user
    #[oai(status = 200)]
    Ok(Json<i32>),

    /// Handle or email already in use
    ///
    /// Returns SQL error message
    #[oai(status = 409)]
    Conflict(PlainText<String>),

    /// Internal error (e.g., DBErr)
    #[oai(status = 500)]
    InternalError,
}
// endregion

#[OpenApi(prefix_path = "/auth")]
impl Endpoints {
    #[oai(path = "/login", method = "post")]
    async fn login(&self, creds: Json<Credentials>, state: Data<&AppState>) -> LoginResp {
        match user::Entity::find_by_email_or_handle(&creds.handle)
            .one(&state.conn)
            .await
        {
            Ok(u) => match u {
                Some(user) => {
                    if user.verify_pwd(&creds.password) {
                        LoginResp::Ok(PlainText("mock_api_token".to_string()))
                    } else {
                        LoginResp::Bad
                    }
                }
                None => LoginResp::Bad,
            },
            Err(e) => {
                error!(err = e.to_string(), "err during login");
                LoginResp::InternalError
            }
        }
    }

    #[oai(path = "/register", method = "post")]
    async fn register(&self, req: Json<RegisterReq>, state: Data<&AppState>) -> RegisterResp {
        let u = user::ActiveModel {
            display_name: Set(req.display_name.clone()),
            handle: Set(req.handle.clone()),
            email: Set(req.email.0.clone()),
            password: Set(hash::hash_pwd(&req.password)),
            ..Default::default()
        };
        match user::Entity::insert(u).exec(&state.conn).await {
            Ok(r) => RegisterResp::Ok(Json(r.last_insert_id)),
            Err(e) => match e.sql_err() {
                Some(SqlErr::UniqueConstraintViolation(f)) => RegisterResp::Conflict(PlainText(f)),
                _ => RegisterResp::InternalError,
            },
        }
    }
}
