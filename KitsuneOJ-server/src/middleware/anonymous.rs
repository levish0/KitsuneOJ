use crate::config::db_config::DbConfig;
use crate::dto::auth::internal::anonymous_user::AnonymousUserContext;
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use cookie::SameSite;
use cookie::time::Duration;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub const ANONYMOUS_USER_COOKIE_NAME: &str = "anonymous_user_id";

pub async fn anonymous_user_middleware(mut req: Request<Body>, next: Next) -> Response {
    // 쿠키에서 anonymous_user_id 확인
    let (final_anonymous_id, has_anonymous_id) = {
        let cookies = req.extensions().get::<Cookies>().unwrap();
        match cookies.get(ANONYMOUS_USER_COOKIE_NAME) {
            Some(cookie) => (cookie.value().to_string(), true),
            None => (Uuid::new_v4().to_string(), false),
        }
    };

    // Extension에 익명 사용자 컨텍스트 추가
    req.extensions_mut().insert(AnonymousUserContext {
        anonymous_user_id: final_anonymous_id.clone(),
    });

    let response = next.run(req).await;

    // 쿠키가 없었다면 새로 생성해서 설정
    if !has_anonymous_id {
        let is_dev = DbConfig::get().is_dev;

        let same_site_attribute = if is_dev {
            SameSite::None
        } else {
            SameSite::Lax
        };

        let cookie = Cookie::build((ANONYMOUS_USER_COOKIE_NAME, final_anonymous_id))
            .http_only(true)
            .secure(true)
            .same_site(same_site_attribute)
            .path("/")
            .max_age(Duration::days(365)) // 1년
            .build();

        // Response에서 쿠키 추출해서 추가
        let cookies = response.extensions().get::<Cookies>().unwrap();
        cookies.add(cookie);
    }

    response
}