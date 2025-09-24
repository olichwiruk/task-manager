use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use cookie::Cookie;
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{app::state::AppState, services::auth_service::Claims};

pub async fn auth_middleware(
    app_state: State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let authorized = if let Some(jwt) = parse_jwt_from_request(&request) {
        let token_result = decode::<Claims>(
            &jwt,
            &DecodingKey::from_secret(app_state.jwt_secret.as_ref()),
            &Validation::default(),
        );

        token_result.is_ok()
    } else {
        false
    };

    if !authorized {
        return Redirect::temporary("/login").into_response();
    }

    next.run(request).await
}

fn parse_jwt_from_request(request: &Request) -> Option<String> {
    let cookie_header = request.headers().get("cookie")?.to_str().ok()?;
    for cookie_str in cookie_header.split(';') {
        let cookie_str = cookie_str.trim();

        let Ok(cookie) = Cookie::parse(cookie_str) else {
            continue;
        };
        if cookie.name() == "jwt" {
            return Some(cookie.value().to_string());
        }
    }
    None
}
