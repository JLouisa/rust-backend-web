// use crate::{domain::shops::Shop, utils::constants::SHOP_CONFIGS};
// use actix_web::{
//     dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
//     http, web,
//     web::Data,
//     Error, HttpResponse,
// };
// use futures_util::future::{ready, LocalBoxFuture, Ready};
// use std::future::Future;

// use crate::db::sqlite::SqliteDB; // Ensure you have the correct path to your database module

// pub struct ShopLoader;

// impl<S, B> Transform<S, ServiceRequest> for ShopLoader
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = ShopLoaderMiddleware<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ready(Ok(ShopLoaderMiddleware { service }))
//     }
// }

// pub struct ShopLoaderMiddleware<S> {
//     service: S,
// }

// impl<S, B> Service<ServiceRequest> for ShopLoaderMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

//     forward_ready!(service);

//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         // println!("Middleware was called");

//         let db_pool = req.app_data::<web::Data<SqliteDB>>().unwrap().clone();

//         let shop_configs = SHOP_CONFIGS.lock().unwrap(); // Consider using a more fault-tolerant approach

//         let host = req
//             .connection_info()
//             .host()
//             .split(':')
//             .next()
//             .unwrap_or_default()
//             .to_string();

//         println!("Request received for host: {}", &host);

//         match shop_configs.get(&host) {
//             Some(shop_config) => {
//                 req.extensions_mut().insert(shop_config.clone());
//                 self.service.call(req).await
//             }
//             None => {
//                 // Log the error or handle it appropriately
//                 let response = HttpResponse::InternalServerError().finish().into_body();
//                 let (req, _) = req.into_parts();
//                 Ok(ServiceResponse::new(req, response))
//             }
//         }
//     }
// }

use crate::{domain::shops::Shop, utils::constants::SHOP_CONFIGS};
use actix_web::HttpRequest;

pub async fn get_shop_domain(req: HttpRequest) -> (String, Option<Shop>) {
    let shop_configs = SHOP_CONFIGS.lock().unwrap(); // Consider using a more fault-tolerant approach
    let domain = req
        .connection_info()
        .host()
        .split(':')
        .next()
        .unwrap_or_default()
        .to_string();

    println!("Request received for host: {}", &domain);

    let shop = match shop_configs.get(&domain) {
        Some(config) => (domain, Some(config.clone())),
        None => (domain, None),
    };

    return shop;
}
