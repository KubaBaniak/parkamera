use axum::{extract::Query, response::Json};
use chrono::{DateTime, FixedOffset, Utc};
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
    let now: DateTime<Utc> = Utc::now();
    let timezone = FixedOffset::west_opt(5 * 3600).expect("FixedOffset::west out of bounds");
    let now_with_timezone = now.with_timezone(&timezone);

    let now_with_timezone: DateTimeWithTimeZone =
        DateTimeWithTimeZone::from_utc(now_with_timezone.naive_utc(), timezone);

    let response = match params.get("current").map(String::as_str) {
        Some("true") => Response {
            id: 5,
            car_arrived: now_with_timezone,
            car_left: None,
        },
        _ => {
            let car_left = "2010-11-12T13:14:15Z"
                .parse::<DateTime<Utc>>()
                .unwrap()
                .with_timezone(&timezone);
            let response = Response {
                id: 5,
                car_arrived: now_with_timezone,
                car_left: Some(DateTimeWithTimeZone::from_utc(
                    car_left.naive_utc(),
                    timezone,
                )),
            };
            response
        }
    };

    Json(response)
}
