use crate::entities::car_log;
use crate::entities::car_log::Entity as CarLogEntity;
use axum::{extract::Query, http::StatusCode, response::Json, Extension};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::DatabaseConnection;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryParams {
    current: Option<String>,
}

#[derive(Serialize)]
pub struct CarResponse {
    id: i64,
    car_arrived: DateTimeWithTimeZone,
    car_left: Option<DateTimeWithTimeZone>,
    spot_id: i16,
}

pub async fn get_cars(
    Extension(database): Extension<DatabaseConnection>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Vec<CarResponse>>, StatusCode> {
    let models = match params.current.as_deref() {
        Some("true") => {
            CarLogEntity::find()
                .filter(car_log::Column::CarLeft.is_null())
                .all(&database)
                .await
        }
        _ => CarLogEntity::find().all(&database).await,
    };

    match models {
        Ok(cars) => {
            let response: Vec<CarResponse> = cars
                .into_iter()
                .map(|car| CarResponse {
                    id: car.id,
                    car_arrived: car.car_arrived,
                    car_left: car.car_left,
                    spot_id: car.spot_id,
                })
                .collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::NO_CONTENT),
    }
}
