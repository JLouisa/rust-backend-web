use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub struct Password(pub String);

impl Password {
    // Create new
    pub fn new(pwd: &String) -> Self {
        Password(pwd.to_string())
    }

    // Password hashing
    pub fn hash_password(password: &str) -> Result<Self, argon2::password_hash::Error> {
        // Generate salt
        let salt = SaltString::generate(&mut OsRng);

        // Hash the password
        let argon2 = Argon2::default();

        match argon2.hash_password(password.as_bytes(), &salt) {
            // Return the hashed password
            Ok(pass) => Ok(Password(pass.to_string())),
            // Return the error
            Err(err) => Err(err),
        }
    }

    // Get the string
    pub fn get_password_string(&self) -> String {
        self.0.to_string()
    }

    // Verify Password
    pub fn verify_password(
        &self,
        client_password: &str,
    ) -> Result<bool, argon2::password_hash::Error> {
        match PasswordHash::new(&self.0) {
            Ok(pass) => {
                let verified_password = Argon2::default()
                    .verify_password(client_password.as_bytes(), &pass)
                    .is_ok();

                Ok(verified_password)
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod password_tests {
    use super::*;

    #[test]
    fn check_password_good() {
        // Setup
        let password = "password123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        // Hash and Salt the password
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Something went wrong")
            .to_string();
        let parsed_hash = PasswordHash::new(&password_hash).expect("Something went wrong2");

        // Check if the passwords are the same
        let verify_password = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        // Test outcome
        assert_eq!(
            verify_password, true,
            "Expecting Eve to be found in the list of users"
        );
    }

    #[test]
    fn check_password_bad() {
        let password = "password12";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password("hunter42".as_bytes(), &salt)
            .expect("Something went wrong")
            .to_string();
        let parsed_hash = PasswordHash::new(&password_hash).expect("Something went wrong2");

        let verify_password = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        assert_eq!(
            verify_password, false,
            "Expecting Eve to be found in the list of users"
        );
    }

    #[test]
    fn check_password_real_true() {
        let password = "password123".to_string();

        let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$+Gn22CB2y5j1xeMh/COeuw$hFhJ2ORLPXui2BlumrMjFV0fNiVADzOuswFvO/6BvEw".to_string();

        let new_pass = Password::new(&password_hash);

        let answer = match new_pass.verify_password(password.as_str()) {
            Ok(verified) => {
                println!("{:?}", verified);
                verified
            }
            Err(_) => false,
        };

        assert_eq!(
            answer, true,
            "Expecting Eve to be found in the list of users"
        );
    }

    #[test]
    fn check_password_real_false() {
        let password = "password12".to_string();

        let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$+Gn22CB2y5j1xeMh/COeuw$hFhJ2ORLPXui2BlumrMjFV0fNiVADzOuswFvO/6BvEw".to_string();

        let new_pass = Password::new(&password_hash);

        let answer = match new_pass.verify_password(password.as_str()) {
            Ok(verified) => {
                println!("{:?}", verified);
                verified
            }
            Err(_) => false,
        };

        assert_eq!(
            answer, false,
            "Expecting Eve to be found in the list of users"
        );
    }
}
