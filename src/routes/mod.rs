mod car_arrived;
mod cars;

use axum::{http::Method, routing::get, Router};
use car_arrived::car_arrived;
use cars::{car_left, get_current_cars};

use tower_http::cors::{Any, CorsLayer};

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route(
            "/cars",
            get(get_current_cars).post(car_arrived).patch(car_left),
        )
        .layer(cors)
}
