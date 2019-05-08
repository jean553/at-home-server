//! The ride structure stored into Redis and exposed via APIs.

#[derive(Deserialize)]
pub struct Ride {
    pub phone_number: String,
    pub latitude: f32,
    pub longitude: f32,
}
