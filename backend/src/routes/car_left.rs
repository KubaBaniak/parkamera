use crate::constants::TIMEZONE_OFFSET_IN_S;
use crate::entities::car_log;
use crate::entities::car_log::Entity as CarLog;
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
pub struct ResponseTimeLeft {
    car_left: DateTimeWithTimeZone,
}

pub async fn car_left(
    Extension(database): Extension<DatabaseConnection>,
    Json(body): Json<RequestCar>,
) -> Result<Json<ResponseTimeLeft>, StatusCode> {
    let car = CarLog::find()
        .filter(car_log::Column::SpotId.eq(body.spot_id as i16))
        .one(&database)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    match car {
        Some(car) => {
            let mut car: car_log::ActiveModel = car.into();

            let offset = FixedOffset::east_opt(TIMEZONE_OFFSET_IN_S).unwrap();
            let now_with_offset = Utc::now().with_timezone(&offset);
            car.car_left = Set(Some(now_with_offset));

            let car = car
                .update(&database)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Json(ResponseTimeLeft {
                car_left: car.car_left.unwrap(),
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
