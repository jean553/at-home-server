//! Handles all the rides API actions.

use ride::Ride;

use rocket_contrib::json::Json;
use uuid::Uuid;
use rocket::{
    State,
    Response,
};
use rocket::http::Status;
use redis::Commands;
use serde_json::json;


/// Creates a ride.
///
/// # Args:
///
/// `redis_client` - the Redis client to use for database connection
/// `ride` - the received ride properties
#[post(
    "/rides",
    format="application/json",
    data="<ride>"
)]
pub fn create_ride(
    redis_client: State<redis::Client>,
    ride: Json<Ride>,
) -> Response {

    let redis_client = redis_client.inner();
    let redis_connection = redis_client.get_connection().unwrap();

    let id = Uuid::new_v4()
        .to_hyphenated()
        .to_string();

    let _ : () = redis_connection.hset_multiple(
        &id,
        &[
            ("latitude", ride.latitude.to_string()),
            ("longitude", ride.longitude.to_string()),
            ("phone_number", ride.phone_number.clone()),
        ]
    ).unwrap();

    Response::build()
        .status(Status::Created)
        .raw_header(
            "Location",
            format!("/rides/{}", &id))
        .finalize()
}

/// Check if a ride is terminated.
///
/// # Args:
///
/// `redis_client` - the Redis client to use for database connection
/// `ride_id` - the id of the ride to check
/// `latitude` - the latitude sent through query params
/// `longitude` - the longitude sent through query params
#[get("/rides/<ride_id>/is-arrived?<latitude>&<longitude>")]
pub fn check_is_arrived(
    redis_client: State<redis::Client>,
    ride_id: String,
    latitude: String,
    longitude: String
) -> Json<serde_json::Value> {

    let redis_client = redis_client.inner();
    let redis_connection = redis_client.get_connection().unwrap();

    /* TODO: returns 404 if the ride cannot be found */

    let (
        destination_latitude,
        destination_longitude
    ) : (String, String) = redis_connection.hget(
        &ride_id,
        &[
            "latitude",
            "longitude",
        ]
    ).unwrap();

    let latitude: f32 = latitude.parse().unwrap();
    let longitude: f32 = longitude.parse().unwrap();
    let destination_latitude: f32 = destination_latitude.parse().unwrap();
    let destination_longitude: f32 = destination_longitude.parse().unwrap();

    let distance = (
        (latitude - destination_latitude).powi(2) +
        (longitude - destination_longitude).powi(2)
    ).sqrt();

    if distance < 0.0003 {
        return Json(json!({"arrived": "true"}));
    }

    Json(json!({"arrived": "false"}))
}

/// Deletes a ride and sends the text message.
///
/// # Args:
///
/// `redis_client` - the Redis client to use for database connection
/// `ride_id` - the id of the ride to check
#[delete("/rides/<ride_id>")]
pub fn remove_ride(
    redis_client: State<redis::Client>,
    ride_id: String
) -> Response {

    let redis_client = redis_client.inner();
    let redis_connection = redis_client.get_connection().unwrap();

    let _: () = redis_connection.del(&ride_id).unwrap();

    /* TODO: send a SMS using Roboto */

    Response::build()
        .status(Status::Ok)
        .finalize()
}

