extern crate interface_tests_helpers;

extern crate reqwest;
extern crate serde;

use interface_tests_helpers::{ResponseHandler, HasBaseUrl};

use reqwest::{
    Client,
    Response,
};

use serde::{
    Serialize,
    Serializer,
};

use std::collections::HashMap;

/// We use a HashMap to declare the data we send to the service;
/// this data is then serialized in order to be sent as JSON;
/// so we are mandatory to declare a HashMap from the tests code side,
/// but this HashMap has different data types according to the fields
/// (the phone_number is a string, while the latitude is a float);
/// so we declare a dedicated enum in order to handle multiple data types
/// into the HashMap and so we can declare HashMap<&str, JsonValue>;
enum JsonValue<'a> {
    PhoneNumber(&'a str),
    GPSCoordinate(f32),
}

/// As explained just before, we need a HashMap<&str, JsonValue>
/// in order to being able to handle multiple data types from the same HashMap;
/// during serialization and before sending the JSON,
/// we have to remove the "enum" layer from the HashMap in order to send it correctly;
/// so we rewrite the serialization function for the JsonValue enum,
/// we manually serialize into a string if the field is the phone number,
/// and we manually serialize into a float if the field is a latitude or a longitude;
impl<'a> Serialize for JsonValue<'a> {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        match *self {
            JsonValue::PhoneNumber(pn) => serializer.serialize_str(pn),
            JsonValue::GPSCoordinate(c) => serializer.serialize_f32(c),
        }
    }
}

/// FIXME: not sure if I have to declare again the trait,
/// only because HasBaseUrl is foreign and Client is also foreign,
/// so it throws the error E0117...

trait HasBaseUrl {

    fn get_base_url(&self) -> &str;
}

impl HasBaseUrl for Client {

    fn get_base_url(&self) -> &str {
        "http://localhost:8000"
    }
}

trait HandleRides {

    fn post_ride(
        &self,
        json: &HashMap<&str, JsonValue>,
    ) -> Response;
}

impl HandleRides for Client {

    fn post_ride(
        &self,
        json: &HashMap<&str, JsonValue>,
    ) -> Response {

        self.post(&format!("{}/api/rides", self.get_base_url()))
            .json(&json)
            .send()
            .unwrap()
    }
}

#[test]
fn test_create_ride_returns_201() {

    let mut json: HashMap<&str, JsonValue> = HashMap::new();
    json.insert("phone_number", JsonValue::PhoneNumber("0102030405"));
    json.insert("latitude", JsonValue::GPSCoordinate(44.256));
    json.insert("longitude", JsonValue::GPSCoordinate(2.333));

    let client = reqwest::Client::new();
    let response = client.post_ride(&json);

    assert_eq!(response.status(), 201);
}
