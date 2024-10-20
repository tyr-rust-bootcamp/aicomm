use super::TokenVerify;
use axum::{
    extract::{FromRequestParts, Query, Request, State},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::Deserialize;
use tracing::warn;

#[derive(Debug, Deserialize)]
struct Params {
    token: String,
}

pub async fn verify_token<T>(State(state): State<T>, req: Request, next: Next) -> Response
where
    T: TokenVerify + Clone + Send + Sync + 'static,
{
    let (mut parts, body) = req.into_parts();
    match extract_token(&state, &mut parts).await {
        Ok(token) => {
            let mut req = Request::from_parts(parts, body);
            match set_user(&state, &token, &mut req) {
                Ok(_) => next.run(req).await,
                Err(msg) => (StatusCode::FORBIDDEN, msg).into_response(),
            }
        }
        Err(msg) => (StatusCode::UNAUTHORIZED, msg).into_response(),
    }
}

pub async fn extract_user<T>(State(state): State<T>, req: Request, next: Next) -> Response
where
    T: TokenVerify + Clone + Send + Sync + 'static,
{
    let (mut parts, body) = req.into_parts();
    let req = if let Ok(token) = extract_token(&state, &mut parts).await {
        let mut req = Request::from_parts(parts, body);
        let _ = set_user(&state, &token, &mut req);
        req
    } else {
        Request::from_parts(parts, body)
    };

    next.run(req).await
}

async fn extract_token<T>(state: &T, parts: &mut Parts) -> Result<String, String>
where
    T: TokenVerify + Clone + Send + Sync + 'static,
{
    match TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, &state).await {
        Ok(TypedHeader(Authorization(bearer))) => Ok(bearer.token().to_string()),
        Err(e) => {
            if e.is_missing() {
                match Query::<Params>::from_request_parts(parts, &state).await {
                    Ok(params) => Ok(params.token.clone()),
                    Err(e) => {
                        let msg = format!("parse query params failed: {}", e);
                        warn!(msg);
                        Err(msg)
                    }
                }
            } else {
                let msg = format!("parse Authorization header failed: {}", e);
                warn!(msg);
                Err(msg)
            }
        }
    }
}

fn set_user<T>(state: &T, token: &str, req: &mut Request) -> Result<(), String>
where
    T: TokenVerify + Clone + Send + Sync + 'static,
{
    match state.verify(token) {
        Ok(user) => {
            req.extensions_mut().insert(user);
            Ok(())
        }
        Err(e) => {
            let msg = format!("verify token failed: {:?}", e);
            warn!(msg);
            Err(msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecodingKey, EncodingKey, User};
    use anyhow::Result;
    use axum::{body::Body, middleware::from_fn_with_state, routing::get, Router};
    use std::sync::Arc;
    use tower::ServiceExt;

    #[derive(Clone)]
    struct AppState(Arc<AppStateInner>);

    struct AppStateInner {
        ek: EncodingKey,
        dk: DecodingKey,
    }

    impl TokenVerify for AppState {
        type Error = ();

        fn verify(&self, token: &str) -> Result<User, Self::Error> {
            self.0.dk.verify(token).map_err(|_| ())
        }
    }

    async fn handler(_req: Request) -> impl IntoResponse {
        (StatusCode::OK, "ok")
    }

    #[tokio::test]
    async fn verify_token_middleware_should_work() -> Result<()> {
        let encoding_pem = include_str!("../../fixtures/encoding.pem");
        let decoding_pem = include_str!("../../fixtures/decoding.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;
        let state = AppState(Arc::new(AppStateInner { ek, dk }));

        let user = User::new(1, "Tyr Chen", "tchen@acme.org");
        let token = state.0.ek.sign(user)?;

        let app = Router::new()
            .route("/", get(handler))
            .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
            .with_state(state);

        // good token
        let req = Request::builder()
            .uri("/")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);

        // good token in query params
        let req = Request::builder()
            .uri(format!("/?token={}", token))
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);

        // no token
        let req = Request::builder().uri("/").body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        // bad token
        let req = Request::builder()
            .uri("/")
            .header("Authorization", "Bearer bad-token")
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::FORBIDDEN);

        // bad token in query params
        let req = Request::builder()
            .uri("/?token=bad-token")
            .body(Body::empty())?;
        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::FORBIDDEN);

        Ok(())
    }
}
