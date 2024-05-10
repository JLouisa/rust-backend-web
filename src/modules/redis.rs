use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use redis::{Client, Commands, RedisError};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum RedisDbError {
    #[error("Database error: {0}")]
    RedisError(#[from] RedisError),
}

pub enum RedisKeyNames {
    PasswordReset,
    Verification,
    Session,
    User,
    Shops,
    Cart,
    Stack,
    Queue,
}
impl RedisKeyNames {
    pub fn get_key(&self, domain: &str) -> String {
        match self {
            RedisKeyNames::PasswordReset => format!("password_reset:{}", domain),
            RedisKeyNames::Verification => format!("verification:{}", domain),
            RedisKeyNames::Session => format!("session:{}", domain),
            RedisKeyNames::User => format!("user:{}", domain),
            RedisKeyNames::Shops => format!("shop:{}", domain),
            RedisKeyNames::Cart => format!("cart:{}", domain),
            RedisKeyNames::Stack => format!("stack:{}", domain),
            RedisKeyNames::Queue => format!("queue:{}", domain),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            RedisKeyNames::PasswordReset => "password_reset",
            RedisKeyNames::Verification => "verification",
            RedisKeyNames::Session => "session",
            RedisKeyNames::User => "user",
            RedisKeyNames::Shops => "shop",
            RedisKeyNames::Cart => "cart",
            RedisKeyNames::Stack => "stack",
            RedisKeyNames::Queue => "queue",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shop {
    domain: String,
    user_id: String,
    shop_name: String,
    product_type: String,
}

pub struct RedisDbAsync {
    pub client: MultiplexedConnection,
}

impl RedisDbAsync {
    // Create Redis Client
    pub async fn new_async(db_redis_url: &str) -> Result<Self, RedisDbError> {
        let con: Client = Client::open(db_redis_url).map_err(RedisDbError::from)?;
        let client = con
            .get_multiplexed_tokio_connection()
            .await
            .map_err(RedisDbError::from)?;
        Ok(RedisDbAsync { client })
    }

    // Set a value for shop
    pub async fn set_shop_config(&mut self, shop: &Shop) -> Result<(), RedisDbError> {
        let key: &str = RedisKeyNames::Shops.as_str();
        let field: String = RedisKeyNames::Shops.get_key(&shop.domain);
        let value: String = serde_json::to_string(shop).unwrap();
        self.client.hset(key, field, value).await?;
        Ok(())
    }

    // Set a value for shop for multiple fields
    pub async fn set_shop_config_multi(&mut self, shop: &Shop) -> Result<(), RedisDbError> {
        let key: String = RedisKeyNames::Shops.get_key(&shop.domain);
        self.client
            .hset_multiple(
                key,
                &[
                    ("domain", shop.domain.as_str()),
                    ("user_id", shop.user_id.as_str()),
                    ("shop_name", shop.shop_name.as_str()),
                    ("product_type", shop.product_type.as_str()),
                ],
            )
            .await?;
        Ok(())
    }

    // Get a value for shop
    pub async fn get_shop_config(&mut self, domain: &str) -> Result<Shop, RedisDbError> {
        let key = RedisKeyNames::Shops.get_key(domain);
        let field = RedisKeyNames::Shops.as_str();
        let result: String = self.client.hget(key, field).await?;
        let result: Shop = serde_json::from_str(&result).unwrap();
        Ok(result)
    }
}

pub struct RedisDB {
    pub client: Client,
    pub low_client: redis::Connection,
}
impl RedisDB {
    // Create Redis Client
    pub fn new(db_redis_url: &str) -> Result<Self, RedisDbError> {
        let client = Client::open(db_redis_url).map_err(RedisDbError::from)?;
        let con = client.get_connection()?;
        Ok(RedisDB {
            client,
            low_client: con,
        })
    }

    // Return Redis Client
    pub fn get_client(&self) -> Client {
        self.client.clone()
    }

    // Set a value
    pub fn set_value<T: redis::ToRedisArgs>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<(), RedisDbError> {
        self.client.set(key, value).map_err(RedisDbError::from)
    }

    // Get a value by key
    pub fn get_value<T: redis::FromRedisValue>(&mut self, key: &str) -> Result<T, RedisDbError> {
        self.client.get(key).map_err(RedisDbError::from)
    }

    // Increment an integer value
    pub fn incr(&mut self, key: &str, increment: isize) -> redis::RedisResult<isize> {
        self.client.incr(key, increment)
    }

    // Get a value by key
    pub fn delete_key(&mut self, key: &str) -> Result<bool, RedisDbError> {
        let removed_count: isize = self.client.del(key).map_err(RedisDbError::from)?;
        Ok(removed_count > 0)
    }

    // Add a value to a set
    pub fn sadd<T: redis::ToRedisArgs>(&mut self, key: &str, value: T) -> redis::RedisResult<()> {
        self.client.sadd(key, value)
    }

    // Get all members of a set
    pub fn smembers<T: redis::FromRedisValue>(&mut self, key: &str) -> redis::RedisResult<Vec<T>> {
        self.client.smembers(key)
    }

    // Remove a value from a set
    pub fn srem<T: redis::ToRedisArgs>(
        &mut self,
        key: &str,
        value: T,
    ) -> redis::RedisResult<isize> {
        self.client.srem(key, value)
    }

    // Set a field in a hash
    pub fn hset<T: redis::ToRedisArgs>(
        &mut self,
        key: &str,
        field: &str,
        value: T,
    ) -> redis::RedisResult<()> {
        self.client.hset(key, field, value)
    }

    // Get a value from a hash field
    pub fn hget<T: redis::FromRedisValue>(
        &mut self,
        key: &str,
        field: &str,
    ) -> redis::RedisResult<Option<T>> {
        self.client.hget(key, field)
    }

    //? Bug in the code below - it doesn't work as expected - it doesn't convert the value to a string
    // // Push to the stack
    // pub fn push<T: redis::ToRedisArgs>(&mut self, key: &str, value: T) -> redis::RedisResult<()> {
    //     self.client.lpush(key, value)
    // }

    // // Pop from the stack
    // pub fn pop(&mut self, key: &str) -> redis::RedisResult<Option<String>> {
    //     let count = NonZeroUsize::new(1).expect("Failed to create NonZeroUsize");
    //     let result: Option<String> = self.client.lpop(key, Some(count))?;
    //     // If the result contains a string, trim quotation marks if present
    //     Ok(result.map(|s| s.trim_matches('"').to_string()))
    // }

    // // Enqueue a value to the back of the queue
    // pub fn enqueue<T: redis::ToRedisArgs + Display + Debug>(
    //     &mut self,
    //     key: &str,
    //     value: T,
    // ) -> redis::RedisResult<()> {
    //     self.client.rpush(key, value.to_string().trim_matches('"'))
    // }

    // // Dequeue a value from the front of the queue
    // pub fn dequeue<T: redis::FromRedisValue + Debug>(&mut self, key: &str) -> Option<T> {
    //     let count = NonZeroUsize::new(1).expect("Failed to create NonZeroUsize");
    //     let result = self.client.lpop(key, Some(count));
    //     match result {
    //         Ok(value) => {
    //             if let Some(ref v) = value {
    //                 println!("Dequeued: {:?}", v);
    //             } else {
    //                 println!("Queue is empty");
    //             }
    //             value
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to dequeue: {}", e);
    //             None
    //         }
    //     }
    // }

    // Push to the stack
    pub fn low_lpush<T: redis::ToRedisArgs>(
        &mut self,
        key: &str,
        value: T,
    ) -> redis::RedisResult<()> {
        redis::cmd("LPUSH")
            .arg(key)
            .arg(value)
            .query(&mut self.low_client)?;
        Ok(())
    }

    // Pop from the stack
    pub fn low_rpop(&mut self, key: &str) -> redis::RedisResult<Option<String>> {
        let result: Option<String> = redis::cmd("RPOP").arg(key).query(&mut self.low_client)?;
        Ok(result)
    }

    // Enqueue a value to the back of the queue
    pub fn low_enqueue<T: redis::ToRedisArgs>(
        &mut self,
        key: &str,
        value: T,
    ) -> redis::RedisResult<()> {
        redis::cmd("RPUSH")
            .arg(key)
            .arg(value)
            .query(&mut self.low_client)?;
        Ok(())
    }

    // Dequeue a value from the front of the queue
    pub fn low_dequeue(&mut self, key: &str) -> redis::RedisResult<Option<String>> {
        let result: Option<String> = redis::cmd("LPOP").arg(key).query(&mut self.low_client)?;
        Ok(result)
    }
}

#[cfg(test)]
mod redis_tests {
    use super::*; // Adjust this according to your actual module structure to import necessary items

    #[test]
    fn test_redis_operations() {
        // Setup Redis Connection
        let redis_url = crate::utils::constants::REDIS_URL.clone();
        let mut redis_db = match RedisDB::new(&redis_url) {
            Ok(db) => db,
            Err(e) => {
                eprintln!("Failed to connect to Redis: {}", e);
                // Decide how to handle the error: return an error or panic
                // For critical applications where Redis is mandatory, you might want to panic
                panic!("Application cannot start without Redis: {}", e);
            }
        };

        // Set and get a string value
        redis_db
            .set_value("test2", "testing2")
            .expect("Failed to set string value");
        let value: String = redis_db
            .get_value("test2")
            .expect("Failed to get string value");
        assert_eq!(value, "testing2", "String value does not match");

        // Set and increment an integer value
        redis_db
            .set_value("counter", 100)
            .expect("Failed to set integer value");
        redis_db
            .incr("counter", 1)
            .expect("Failed to increment value");
        let counter: isize = redis_db
            .get_value("counter")
            .expect("Failed to get incremented value");
        assert_eq!(counter, 101, "Counter value does not match");

        // Test deleting a key
        let was_deleted = redis_db.delete_key("test2").expect("Failed to delete key");
        assert!(was_deleted, "Key was not deleted");

        // Add elements to a set and verify
        redis_db
            .sadd("myset", "member1")
            .expect("Failed to add to set");
        redis_db
            .sadd("myset", "member2")
            .expect("Failed to add to set");
        let members: Vec<String> = redis_db.smembers("myset").expect("Failed to get members");
        assert!(
            members.contains(&"member1".to_string()) && members.contains(&"member2".to_string()),
            "Set members do not match"
        );

        // Remove a member from the set and verify
        let removed_count = redis_db
            .srem("myset", "member2")
            .expect("Failed to remove member");
        assert_eq!(removed_count, 1, "Incorrect number of members removed");
        let members_after: Vec<String> = redis_db
            .smembers("myset")
            .expect("Failed to get members after removal");
        assert!(
            !members_after.contains(&"member2".to_string()),
            "Member was not removed from set"
        );

        // Set fields in a hash and verify
        redis_db
            .hset("user:1000", "name", "Alice")
            .expect("Failed to set hash value");
        redis_db
            .hset("user:1000", "job", "Engineer")
            .expect("Failed to set hash value");
        let name: Option<String> = redis_db
            .hget("user:1000", "name")
            .expect("Failed to get hash value");
        let job: Option<String> = redis_db
            .hget("user:1000", "job")
            .expect("Failed to get hash value");
        assert_eq!(
            name,
            Some("Alice".to_string()),
            "Hash name value does not match"
        );
        assert_eq!(
            job,
            Some("Engineer".to_string()),
            "Hash job value does not match"
        );
    }

    #[test]
    fn test_stack_and_queue_operations() {
        // Setup Redis Connection
        let redis_url = crate::utils::constants::REDIS_URL.clone();
        let mut redis_db = match RedisDB::new(&redis_url) {
            Ok(db) => db,
            Err(e) => {
                eprintln!("Failed to connect to Redis: {}", e);
                // Decide how to handle the error: return an error or panic
                // For critical applications where Redis is mandatory, you might want to panic
                panic!("Application cannot start without Redis: {}", e);
            }
        };

        // Testing low_lpush
        redis_db
            .low_lpush("test_stack", "item1")
            .expect("Failed to LPUSH");
        redis_db
            .low_lpush("test_stack", "item2")
            .expect("Failed to LPUSH");

        // Testing low_lpop
        let popped: Option<String> = redis_db.low_rpop("test_stack").expect("Failed to RPOP");
        assert_eq!(
            popped,
            Some("item1".to_string()),
            "RPOP did not return the last pushed item"
        );

        // Ensure the stack's state is as expected
        let remaining: Option<String> = redis_db.low_rpop("test_stack").expect("Failed to RPOP");
        assert_eq!(
            remaining,
            Some("item2".to_string()),
            "Stack state was not as expected after operations"
        );

        // Testing low_enqueue
        redis_db
            .low_enqueue("test_queue", "first")
            .expect("Failed to RPUSH");
        redis_db
            .low_enqueue("test_queue", "second")
            .expect("Failed to RPUSH");

        // Testing low_dequeue
        let first_out = redis_db.low_dequeue("test_queue").expect("Failed to LPOP");
        assert_eq!(
            first_out,
            Some("first".to_string()),
            "Dequeued item was not 'first'"
        );

        let second_out = redis_db.low_dequeue("test_queue").expect("Failed to LPOP");
        assert_eq!(
            second_out,
            Some("second".to_string()),
            "Dequeued item was not 'second'"
        );

        // Ensure the queue is empty
        let should_be_none = redis_db.low_dequeue("test_queue").expect("Failed to LPOP");
        assert!(should_be_none.is_none(), "Queue should be empty but wasn't");

        // // Pop elements from the stack and check correctness
        // let item2: Option<String> = redis_db.pop("mystack").expect("Failed to pop from stack");
        // assert_eq!(
        //     item2,
        //     Some("item2".to_string()),
        //     "Incorrect item popped from stack"
        // );

        // let item1: Option<String> = redis_db.pop("mystack").expect("Failed to pop from stack");
        // assert_eq!(
        //     item1,
        //     Some("item1".to_string()),
        //     "Incorrect item popped from stack"
        // );

        // // Ensure the stack is empty
        // let none: Option<String> = redis_db.pop("mystack").expect("Failed to pop from stack");
        // assert!(none.is_none(), "Stack should be empty");

        // // Enqueue items to the queue
        // redis_db
        //     .enqueue("myqueue", "first item")
        //     .expect("Failed to enqueue item");
        // redis_db
        //     .enqueue("myqueue", "second item")
        //     .expect("Failed to enqueue item");

        // // Dequeue items from the queue and check correctness
        // let first_item: Option<String> =
        //     redis_db.dequeue("myqueue").expect("Failed to dequeue item");
        // assert_eq!(
        //     first_item,
        //     Some("first item".to_string()),
        //     "Incorrect item dequeued"
        // );

        // let second_item: Option<String> =
        //     redis_db.dequeue("myqueue").expect("Failed to dequeue item");
        // assert_eq!(
        //     second_item,
        //     Some("second item".to_string()),
        //     "Incorrect item dequeued"
        // );

        // // Ensure the queue is empty
        // let none: Option<String> = redis_db.dequeue("myqueue").expect("Failed to dequeue item");
        // assert!(none.is_none(), "Queue should be empty");
    }

    #[tokio::test]
    async fn test_redis_shop_setup() {
        let mut client =
            RedisDbAsync::new_async(crate::utils::constants::REDIS_URL.clone().as_str())
                .await
                .unwrap();

        let new_shop = Shop {
            domain: "example.com".to_string(),
            user_id: "abcdef12345".to_string(),
            shop_name: "The Example Shop".to_string(),
            product_type: "Books".to_string(),
        };

        let new_shop2 = Shop {
            domain: "honeydragons.com".to_string(),
            user_id: "abcdef12345".to_string(),
            shop_name: "The Honeydragons Shop".to_string(),
            product_type: "Books".to_string(),
        };

        let new_shop3 = Shop {
            domain: "lafleur.com".to_string(),
            user_id: "abcdef12345".to_string(),
            shop_name: "The Lafleur Shop".to_string(),
            product_type: "Books".to_string(),
        };

        let set_shop = client.set_shop_config(&new_shop).await;
        let set_shop2 = client.set_shop_config(&new_shop2).await;
        let set_shop3 = client.set_shop_config(&new_shop3).await;

        assert!(set_shop.is_ok(), "Set shop config failed");
        assert!(set_shop2.is_ok(), "Set shop2 config failed");
        assert!(set_shop3.is_ok(), "Set shop3 config failed");

        let get_shop = client
            .get_shop_config("example.com")
            .await
            .expect("Get shop config");

        assert_eq!(get_shop.domain, new_shop.domain, "Domain does not match");
    }
}
