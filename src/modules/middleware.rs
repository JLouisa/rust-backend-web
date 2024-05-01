use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http, Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::domain::datatypes::CookieVariations;

pub struct CheckLogin;

impl<S, B> Transform<S, ServiceRequest> for CheckLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware { service }))
    }
}
pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        println!("Middleware was called");

        let authentication_cookie = request.cookie(CookieVariations::Auth.get_name().as_str());

        match authentication_cookie {
            Some(cookie) => {
                println!("Cookie found: {:?}", cookie);
                match crate::modules::token_pub::verify_token(cookie.value()) {
                    Some(user) => {
                        let res = self.service.call(request);

                        return Box::pin(async move {
                            // forwarded responses map to "left" body
                            res.await.map(ServiceResponse::map_into_left_body)
                        });
                    }
                    None => {
                        if request.path() != "/login" {
                            let (request, _pl) = request.into_parts();

                            let response = HttpResponse::Found()
                                .insert_header((http::header::LOCATION, "/login"))
                                .finish()
                                // constructed responses map to "right" body
                                .map_into_right_body();

                            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                        } else {
                            let res = self.service.call(request);

                            return Box::pin(async move {
                                // forwarded responses map to "left" body
                                res.await.map(ServiceResponse::map_into_left_body)
                            });
                        }
                    }
                };
            }
            None => {
                if request.path() != "/login" {
                    let (request, _pl) = request.into_parts();

                    let response = HttpResponse::Found()
                        .insert_header((http::header::LOCATION, "/login"))
                        .finish()
                        // constructed responses map to "right" body
                        .map_into_right_body();

                    return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                } else {
                    let res = self.service.call(request);

                    return Box::pin(async move {
                        // forwarded responses map to "left" body
                        res.await.map(ServiceResponse::map_into_left_body)
                    });
                }
            }
        }
    }
}
