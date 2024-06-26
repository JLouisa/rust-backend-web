use std::{
    future::{ready, Ready},
    task::{Context, Poll},
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct AddMsg {
    enabled: bool,
}

impl AddMsg {
    pub fn enabled() -> Self {
        Self { enabled: true }
    }

    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AddMsg
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Transform = AddMsgService<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AddMsgService {
            service,
            enabled: self.enabled,
        }))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Msg(pub String);

pub struct AddMsgService<S> {
    service: S,
    enabled: bool,
}

impl<S, B> Service<ServiceRequest> for AddMsgService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("request is passing through the AddMsg middleware");

        if self.enabled {
            // insert data into extensions if enabled
            req.extensions_mut()
                .insert(Msg("Hello from Middleware!".to_owned()));
        }

        self.service.call(req)
    }
}
