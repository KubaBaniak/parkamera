use crate::entities::car_log;
use axum::{http::StatusCode, Extension, Json};
use sea_orm::{prelude::DateTimeWithTimeZone, ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestCar {
    car_arrived: DateTimeWithTimeZone,
}

#[derive(Serialize)]
pub struct ResponseCar {
    id: i64,
    car_arrived: DateTimeWithTimeZone,
    car_left: Option<DateTimeWithTimeZone>,
}

pub async fn car_arrived(
    Extension(database): Extension<DatabaseConnection>,
    Json(body): Json<RequestCar>,
) -> Result<Json<ResponseCar>, StatusCode> {
    let new_car = car_log::ActiveModel {
        car_arrived: Set(body.car_arrived),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseCar {
        id: new_car.id.unwrap(),
        car_arrived: new_car.car_arrived.unwrap(),
        car_left: new_car.car_left.unwrap(),
    }))
}
