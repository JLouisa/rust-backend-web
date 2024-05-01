use crate::domain::datatypes::{CookieVariations, Settings};
use actix_web::cookie::Cookie;

pub fn generate_cookie(variation: &CookieVariations, setting: Settings) -> Cookie {
    match variation {
        &CookieVariations::Auth => Cookie::build("auth", setting.value)
            .path("/")
            .expires(setting.time.to_owned())
            .secure(true)
            .http_only(false)
            .finish(),
        &CookieVariations::ShoppingCarts => Cookie::build("shopping_carts", setting.value)
            .path("/")
            .expires(setting.time.to_owned())
            .secure(true)
            .http_only(false)
            .finish(),
        _ => unreachable!("Cookies section should be unreachable"),
    }
}
