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

mod ride;
mod rides;

use rusoto_core::{
    Region,
    HttpClient,
};
use rusoto_sns::{
    Sns,
    SnsClient,
    PublishInput,
};
use rusoto_credential::EnvironmentProvider;

use std::env;

/// Ping API for healthchecks.
#[get("/ping")]
fn get_ping() -> &'static str {
    "OK"
}

fn main() {

    let redis_url: &str = &env::var("REDIS_URL").expect("Missing REDIS_URL");
    let redis_client = redis::Client::open(redis_url).unwrap();

    let credentials = EnvironmentProvider::default();

    let sns_client = SnsClient::new_with(
        HttpClient::new().unwrap(),
        credentials,
        Region::EuWest1
    );

    /* FIXME: sending a text message is handled here for now for tests purposes,
       it should be sent when an user is arrived at his final destination */
    let message = PublishInput {
        message: "Your friend is at home!".to_string(),
        phone_number: Some("".to_string()),
        message_attributes: None,
        message_structure: None,
        subject: Some("AtHome".to_string()),
        topic_arn: None,
        target_arn: None,
    };
    sns_client.publish(message).sync().unwrap();

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
