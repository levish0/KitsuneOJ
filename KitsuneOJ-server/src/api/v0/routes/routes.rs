use super::health::routes::health_routes as HealthRoutes;
use super::openapi::ApiDoc;
use crate::errors::errors::handler_404;
use crate::state::AppState;
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;

pub fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/swagger.json", ApiDoc::merged()))
        .nest("/v0", HealthRoutes())
        .fallback(handler_404)
}