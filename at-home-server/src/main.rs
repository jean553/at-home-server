//! The AtHome micro-service server.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_contrib;
extern crate uuid;
extern crate redis;

mod ride;
mod rides;

use std::env;

/// Ping API for healthchecks.
#[get("/ping")]
fn get_ping() -> &'static str {
    "OK"
}

fn main() {

    let redis_url: &str = &env::var("REDIS_URL").expect("Missing REDIS_URL");
    let redis_client = redis::Client::open(redis_url).unwrap();

    rocket::ignite()
        .manage(redis_client)
        .mount(
            "/api",
            routes![
                get_ping,
                rides::create_ride,
                rides::check_is_arrived,
                rides::remove_ride
            ]
        ).launch();
}
