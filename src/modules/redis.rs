use redis::{Client, Commands, RedisError};

#[derive(Debug, thiserror::Error)]
pub enum RedisDbError {
    #[error("Database error: {0}")]
    RedisError(#[from] RedisError),
}

#[derive(Debug, Clone)]
pub struct RedisDB {
    pub client: Client,
}

impl RedisDB {
    // Create Redis Client
    pub fn new(db_redis_url: &str) -> Result<Self, RedisDbError> {
        let client = Client::open(db_redis_url).map_err(RedisDbError::from)?;
        Ok(RedisDB { client })
    }

    // Return Redis Client
    pub fn get_client(&self) -> Client {
        self.client.clone()
    }

    // Get a value by key
    pub fn get_value<T: redis::FromRedisValue>(&mut self, key: &str) -> Result<T, RedisDbError> {
        self.client.get(key).map_err(RedisDbError::from)
    }

    // Set a value
    pub fn set_value<T: redis::ToRedisArgs>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<(), RedisDbError> {
        self.client.set(key, value).map_err(RedisDbError::from)
    }
}
