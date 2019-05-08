//! The AtHome micro-service server.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_contrib;
extern crate uuid;
extern crate redis;

mod ride;

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

/// Ping API for healthchecks.
#[get("/ping")]
fn get_ping() -> &'static str {
    "OK"
}

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
fn create_ride(
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
fn check_is_arrived(
    redis_client: State<redis::Client>,
    ride_id: String,
    latitude: String,
    longitude: String
) -> Json<serde_json::Value> {

    let redis_client = redis_client.inner();
    let redis_connection = redis_client.get_connection().unwrap();

    /* TODO: returns 404 if the ride cannot be found */

    let (latitude, longitude) : (String, String) = redis_connection.hget(
        &ride_id,
        &[
            "latitude",
            "longitude",
        ]
    ).unwrap();

    /* TODO: determine if the points are closed to each other */

    Json(json!({"arrived": "true"}))
}

/// Deletes a ride and sends the text message.
///
/// # Args:
///
/// `redis_client` - the Redis client to use for database connection
/// `ride_id` - the id of the ride to check
#[delete("/rides/<ride_id>")]
fn remove_ride(
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

fn main() {

    const REDIS_URL: &str = "redis://at-home-server_db/";
    let redis_client = redis::Client::open(REDIS_URL).unwrap();

    rocket::ignite()
        .manage(redis_client)
        .mount(
            "/api",
            routes![
                get_ping,
                create_ride,
                check_is_arrived,
                remove_ride
            ]
        ).launch();
}
