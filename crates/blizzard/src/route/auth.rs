use super::prelude::*;
use poem::session::Session;
use poem::web::cookie::{Cookie, CookieJar};
use poem_openapi::SecurityScheme;
use poem_openapi::auth::ApiKey;
use sea_orm::DatabaseConnection;

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
    user::Entity::find_by_api_key(key.key)
        .one(req.data::<DatabaseConnection>().unwrap())
        .await
        .unwrap_or(None)
}
// endregion

// region Login
#[derive(Object)]
struct Credentials {
    /// Either email or handle.
    handle: String,
    password: Password,
}

#[derive(ApiResponse)]
enum LoginResp {
    #[oai(status = 200)]
    Ok,
    /// Invalid credentials.
    #[oai(status = 400)]
    Bad,
    /// Internal error (e.g., DBErr).
    #[oai(status = 500)]
    InternalError,
}
// endregion

// region Register
#[derive(Object)]
#[oai(rename_all = "camelCase")]
struct RegisterReq {
    display_name: Option<String>,
    #[oai(validator(min_length = 6, max_length = 24, pattern = r"^[a-zA-Z0-9_]+$"))]
    handle: String,
    email: Email,
    password: Password,
}

#[derive(ApiResponse)]
enum RegisterResp {
    /// Returns ID of inserted user.
    #[oai(status = 201)]
    Ok(Json<i32>),

    /// Handle or email already in use.
    /// Returns SQL error message.
    #[oai(status = 409)]
    Conflict(PlainText<String>),

    /// Internal error (e.g., DBErr).
    #[oai(status = 500)]
    InternalError,
}
// endregion

#[OpenApi(prefix_path = "/auth", tag = "Tags::Auth")]
impl Endpoints {
    #[oai(path = "/login", method = "post", operation_id = "login")]
    async fn login(
        &self,
        creds: Json<Credentials>,
        jar: &CookieJar,
        state: Data<&AppState>,
    ) -> LoginResp {
        match user::Entity::find_by_email_or_handle(&creds.handle)
            .one(&state.conn)
            .await
        {
            Ok(u) => match u {
                Some(user) => {
                    if user.verify_pwd(&creds.password) {
                        jar.add(Cookie::new("token", user.create_token().unwrap()));
                        LoginResp::Ok
                    } else {
                        LoginResp::Bad
                    }
                }
                None => LoginResp::Bad,
            },
            Err(e) => {
                #[cfg(debug_assertions)]
                error!(err = %e, "err during login");
                LoginResp::InternalError
            }
        }
    }

    #[oai(path = "/register", method = "post", operation_id = "register")]
    async fn register(
        &self,
        req: Json<RegisterReq>,
        jar: &CookieJar,
        state: Data<&AppState>,
    ) -> RegisterResp {
        let u = user::ActiveModel {
            display_name: Set(req.display_name.clone()),
            handle: Set(req.handle.clone()),
            email: Set(req.email.0.clone()),
            password: Set(req.password.0.clone()),
            ..Default::default()
        };
        match user::Entity::insert(u)
            .exec_with_returning(&state.conn)
            .await
        {
            Ok(r) => {
                // TODO: proper err handling
                jar.add(Cookie::new("token", r.create_token().unwrap()));
                RegisterResp::Ok(Json(r.id))
            }
            Err(e) => match e.sql_err() {
                Some(SqlErr::UniqueConstraintViolation(f)) => RegisterResp::Conflict(PlainText(f)),
                e => {
                    #[cfg(debug_assertions)]
                    error!(err = ?e, "err during registration");
                    RegisterResp::InternalError
                }
            },
        }
    }
}
