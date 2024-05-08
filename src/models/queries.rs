pub enum UserQueries {
    CreateOneUser,
    GetOneUser,
    GetOneUserWithUsername,
    GetAllUsers,
    UpdateOneUser,
    UpdateOneUserPwd,
    DeleteOneUser,
}

impl UserQueries {
    pub fn convert_to_str(&self) -> &'static str {
        match self {
            UserQueries::CreateOneUser => {
                "INSERT INTO users (user_id, username, hashed_password, active) VALUES (?, ?, ?, ?)"
            }
            UserQueries::GetOneUser => "SELECT * FROM users WHERE user_id = ?",
            UserQueries::GetOneUserWithUsername => "SELECT * FROM users WHERE username = ?",
            UserQueries::GetAllUsers => "SELECT * FROM users",
            UserQueries::UpdateOneUser => {
                "UPDATE users SET username = ?, hashed_password = ?, active = ? WHERE user_id = ?"
            }
            UserQueries::UpdateOneUserPwd => {
                "UPDATE users SET hashed_password = ? WHERE username = ?"
            }
            UserQueries::DeleteOneUser => "DELETE FROM users WHERE user_id = ?",
        }
    }
}

pub enum ShopQueries {
    GetAllShops,
    CreateOneShop,
    GetOneShop,
    UpdateOneShop,
    DeleteOneShop,
}
impl ShopQueries {
    pub fn convert_to_str(&self) -> &'static str {
        match self {
            ShopQueries::GetAllShops => {
                "SELECT domain, name, product_type FROM shop_configurations"
            }
            ShopQueries::GetOneShop => "SELECT * FROM shop_configurations WHERE domain = ?",
            _ => unreachable!("ShopQueries variant not implemented"),
        }
    }
}
