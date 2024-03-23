mod car_arrived;
mod car_left;
mod get_cars;

use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
use car_arrived::car_arrived;
use car_left::car_left;
use get_cars::get_cars;

use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/cars", get(get_cars))
        .route("/arrived", post(car_arrived))
        .route("/left", post(car_left))
        .layer(cors)
        .layer(Extension(database))
}
