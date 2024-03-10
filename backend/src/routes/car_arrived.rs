use axum::{http::header::HeaderMap, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CarArrived {
    time_arrived: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarArrivedResponse {
    time_arrived: DateTime<Utc>,
    time_left: Option<DateTime<Utc>>,
}

pub async fn car_arrived(
    headers: HeaderMap,
    Json(body): Json<CarArrived>,
) -> Json<CarArrivedResponse> {
    for (name, value) in headers.iter() {
        println!("{:?}: {:?}", name, value);
    }
    let _message = headers
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    Json(CarArrivedResponse {
        time_arrived: body.time_arrived,
        time_left: None,
    })
}
