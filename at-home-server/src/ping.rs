//! Handles the healcheck action.

/// Ping API for healthchecks.
#[get("/ping")]
pub fn get_ping() -> &'static str {
    "OK"
}

