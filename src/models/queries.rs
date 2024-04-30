pub enum UserQueries {
    CreateOneUser,
    GetOneUser,
    GetAllUsers,
    UpdateOneUser,
    DeleteOneUser,
}

impl UserQueries {
    pub fn convert_to_str(&self) -> &str {
        match self {
            UserQueries::CreateOneUser => {
                "INSERT INTO users (user_id, username, hashed_password, active) VALUES (?, ?, ?, ?)"
            }
            UserQueries::GetOneUser => "SELECT * FROM users WHERE user_id = ?",
            UserQueries::GetAllUsers => "SELECT * FROM users",
            UserQueries::UpdateOneUser => {
                "UPDATE users SET username = ?, hashed_password = ?, active = ? WHERE user_id = ?"
            }
            UserQueries::DeleteOneUser => "DELETE FROM users WHERE user_id = ?",
        }
    }
}
