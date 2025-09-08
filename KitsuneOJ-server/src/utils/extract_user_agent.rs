use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;

pub fn extract_user_agent(user_agent: Option<TypedHeader<UserAgent>>) -> String {
    user_agent.map(|ua| ua.0.to_string()).unwrap_or_default()
}