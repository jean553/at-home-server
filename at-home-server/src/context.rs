//! The context passed to route, all required objects wrapped into a structure

use redis::Client as RedisClient;

use rusoto_sns::SnsClient;

pub struct Context {
    pub redis_client: RedisClient, 
    pub sns_client: SnsClient,
}
