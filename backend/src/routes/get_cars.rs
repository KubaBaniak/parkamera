use crate::constants::TIMEZONE_OFFSET_IN_S;
use axum::{extract::Query, response::Json};
use chrono::{FixedOffset, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Response {
    id: i64,
    car_arrived: DateTimeWithTimeZone,
    car_left: Option<DateTimeWithTimeZone>,
}

pub async fn get_cars(Query(params): Query<HashMap<String, String>>) -> Json<Response> {
    let offset = FixedOffset::east_opt(TIMEZONE_OFFSET_IN_S).unwrap();
    let now_with_offset = Utc::now().with_timezone(&offset);

    let response = match params.get("current").map(String::as_str) {
        Some("true") => Response {
            id: 5,
            car_arrived: now_with_offset,
            car_left: None,
        },
        _ => {
            let response = Response {
                id: 5,
                car_arrived: now_with_offset,
                car_left: Some(now_with_offset),
            };
            response
        }
    };

    Json(response)
}
