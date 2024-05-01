use crate::domain::datatypes::{UserCookie, UserServer};
use crate::utils;
use core::convert::TryFrom;
use dotenv::dotenv;
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::{Generate, SymmetricKey};
use pasetors::paserk::FormatAsPaserk;
use pasetors::token::UntrustedToken;
use pasetors::{local, version4::V4, Local};

// Generate Trusted Token for Client
pub fn generete_public_token(user: UserServer) -> String {
    dotenv().expect(".env file not found");
    let token_secret: &[u8] = utils::constants::TOKEN_SECRET.as_bytes();
    let token_sk = utils::constants::TOKEN_SK.to_string();

    // Add a custom `data` claims.
    let mut claims = Claims::new().expect("Creating claim failed");
    claims
        .add_additional("user_id", user.user_id)
        .expect("Addition 1 failed");
    claims
        .add_additional("username", user.username)
        .expect("Addition 1 fail");
    claims
        .expiration("2039-01-01T00:00:00+00:00")
        .expect("Experation claim failed");

    // Generate the key and encrypt the claims.
    let sk = SymmetricKey::<V4>::try_from(token_sk.as_str()).expect("Generating Key failed");

    // Create Token
    let token =
        local::encrypt(&sk, &claims, None, Some(token_secret)).expect("Creating token failed");

    return token;
}

// Verify Untrusted Token from Client
pub fn verify_token(untrusted_inc_token: &str) -> Option<UserCookie> {
    dotenv().expect(".env file not found");
    let token_secret: &[u8] = utils::constants::TOKEN_SECRET.as_bytes();
    let token_sk = utils::constants::TOKEN_SK.to_string();

    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Local, V4>::try_from(untrusted_inc_token)
        .expect("Parsing of Untrusted token failed");

    // Generate the key and encrypt the claims.
    let sk = SymmetricKey::<V4>::try_from(token_sk.as_str()).expect("Generating Key failed");
    let trusted_token = local::decrypt(
        &sk,
        &untrusted_token,
        &validation_rules,
        None,
        Some(token_secret),
    );
    match trusted_token {
        Ok(token) => match token.payload_claims().to_owned() {
            Some(info) => Some(UserCookie::new(info)),
            None => None,
        },
        Err(_) => None,
    }
    // .expect("Untrusted token failed");
}

pub fn generete_public_token_test() {
    dotenv().expect(".env file not found");
    let token_secret: &[u8] = utils::constants::TOKEN_SECRET.as_bytes();

    // Setup the default claims, which include `iat` and `nbf` as the current time and `exp` of one hour.
    // Add a custom `data` claim as well.
    let mut claims = Claims::new().expect("Creating claim failed");
    claims
        .add_additional("data", "A secret encrypted message")
        .expect("Addition 1 failed");
    claims
        .add_additional("user_id", "1234")
        .expect("Addition 1 fail");
    claims
        .expiration("2039-01-01T00:00:00+00:00")
        .expect("Experation claim failed");

    // Generate the key and encrypt the claims.
    let token_sk = "k4.local.JvUcBYO9vWzStfoaGdvuWAEBgLJDxIq1mgVAKIQLmH8";
    let sk = SymmetricKey::<V4>::try_from(token_sk).expect("Generating Key failed");

    // Generate Token SK
    fn create_sk_token() -> String {
        let sk = SymmetricKey::<V4>::generate().expect("Generating Key failed");
        let mut paserk = String::new();
        sk.fmt(&mut paserk).expect("failed to park");
        format!("Paserk: {:?}", paserk)
    }

    let sk_token = create_sk_token();

    let token =
        local::encrypt(&sk, &claims, None, Some(token_secret)).expect("Creating token failed");

    // Decide how we want to validate the claims after verifying the token itself.
    // The default verifies the `nbf`, `iat` and `exp` claims. `nbf` and `iat` are always
    // expected to be present.
    // NOTE: Custom claims, defined through `add_additional()`, are not validated. This must be done
    // manually.
    let validation_rules = ClaimsValidationRules::new();

    let untrusted_token =
        UntrustedToken::<Local, V4>::try_from(&token).expect("Untrusted token failed");

    let trusted_token = local::decrypt(
        &sk,
        &untrusted_token,
        &validation_rules,
        None,
        Some(token_secret),
    )
    .expect("Untrusted token failed");

    assert_eq!(&claims, trusted_token.payload_claims().unwrap());

    let claims = trusted_token.payload_claims().unwrap();

    println!("{:?}", claims);
    println!("Secret Key: {:?}", sk_token);
    println!("Secret Key: {:?}", sk);
    println!("{:?}", claims.get_claim("data"));
    println!("{:?}", claims.get_claim("user_id"));
    println!("{:?}", claims.get_claim("iat"));
}
