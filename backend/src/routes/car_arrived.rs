use crate::entities::car_log::Entity as CarLogEntity;
use crate::{constants::TIMEZONE_OFFSET_IN_S, entities::car_log};
use axum::{http::StatusCode, Extension, Json};
use chrono::{FixedOffset, Utc};
use sea_orm::{prelude::DateTimeWithTimeZone, ActiveModelTrait, DatabaseConnection, Set};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestCar {
    spot_id: u8,
}

#[derive(Serialize)]
pub struct ResponseCar {
    id: i64,
    spot_id: i16,
    car_arrived: DateTimeWithTimeZone,
}

pub async fn car_arrived(
    Extension(database): Extension<DatabaseConnection>,
    Json(body): Json<RequestCar>,
) -> Result<Json<ResponseCar>, StatusCode> {
    let offset = FixedOffset::east_opt(TIMEZONE_OFFSET_IN_S).unwrap();
    let now_with_offset = Utc::now().with_timezone(&offset);

    let existing_car = CarLogEntity::find()
        .filter(car_log::Column::SpotId.eq(body.spot_id as i16))
        .filter(car_log::Column::CarLeft.is_null())
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_car.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let new_car = car_log::ActiveModel {
        car_arrived: Set(now_with_offset),
        spot_id: Set(body.spot_id.into()),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseCar {
        id: new_car.id.unwrap(),
        car_arrived: new_car.car_arrived.unwrap(),
        spot_id: new_car.spot_id.unwrap(),
    }))
}
