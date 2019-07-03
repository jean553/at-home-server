//! The AtHome micro-service server.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_contrib;
extern crate uuid;
extern crate redis;
extern crate futures;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_sns;

mod ping;
mod ride;
mod context;
mod rides;

use context::Context;

use rusoto_core::{
    Region,
    HttpClient,
};
use rusoto_sns::SnsClient;
use rusoto_credential::EnvironmentProvider;

use std::env;

fn main() {

    let redis_url: &str = &env::var("REDIS_URL").expect("Missing REDIS_URL");

    let context = Context {
        redis_client: redis::Client::open(redis_url).unwrap(),
        sns_client: SnsClient::new_with(
            HttpClient::new().unwrap(),
            EnvironmentProvider::default(),
            Region::EuWest1
        )
    };

    rocket::ignite()
        .manage(context)
        .mount(
            "/api",
            routes![
                ping::get_ping,
                rides::create_ride,
                rides::check_is_arrived,
                rides::remove_ride
            ]
        ).launch();
}
