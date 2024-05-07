use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
// use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct ShopConfig {
    pub domain: String,
    pub name: String,
    pub product_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Shop {
    pub name: String,
    pub product_type: String,
}
