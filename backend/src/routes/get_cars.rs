use crate::constants::TIMEZONE_OFFSET_IN_S;
use crate::entities::car_log;
use crate::entities::car_log::Entity as CarLog;
use axum::{extract::Query, http::StatusCode, response::Json, Extension};
use chrono::{FixedOffset, Utc};
use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Response {
    id: i64,
    spot_id: i16,
    car_arrived: DateTimeWithTimeZone,
    car_left: Option<DateTimeWithTimeZone>,
}

pub async fn get_cars(
    Extension(database): Extension<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<CarLog>>, StatusCode> {
    let offset = FixedOffset::east_opt(TIMEZONE_OFFSET_IN_S).unwrap();
    let now_with_offset = Utc::now().with_timezone(&offset);

    let current_cars = CarLog::find()
        .filter(car_log::Column::CarLeft.is_not_null())
        .all(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    current_cars[0].id
    let response = match params.get("current").map(String::as_str) {
        Some("true") => CarLog::find()
            .filter(car_log::Column::CarLeft.is_not_null())
            .all(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        _ => CarLog::find()
            .all(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };



    Ok(Json(response))
}
