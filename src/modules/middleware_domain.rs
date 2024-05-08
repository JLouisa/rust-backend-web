use std::{
    future::{ready, Ready},
    task::{Context, Poll},
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};

use crate::utils::constants::SHOP_CONFIGS;

#[derive(Clone, Debug)]
pub struct AddShopDomain {
    enabled: bool,
}

impl AddShopDomain {
    pub fn enabled() -> Self {
        Self { enabled: true }
    }

    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AddShopDomain
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Transform = AddShopDomainService<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AddShopDomainService {
            service,
            enabled: self.enabled,
        }))
    }
}

#[derive(Debug, Clone)]
pub struct Shop {
    pub domain: String,
    pub name: String,
    pub product_type: String,
}

pub struct AddShopDomainService<S> {
    service: S,
    enabled: bool,
}

impl<S, B> Service<ServiceRequest> for AddShopDomainService<S>
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

        // insert data into extensions if enabled
        if self.enabled {
            let host = req.connection_info().host().to_string();

            // Using a mutex to guard global state
            let shop_configs = SHOP_CONFIGS.lock().unwrap();

            match shop_configs.get(&host) {
                Some(config) => {
                    let shop = Shop {
                        domain: host.to_string(),
                        name: config.name.clone(),
                        product_type: config.product_type.clone(),
                    };
                    req.extensions_mut().insert((host, Some(shop)));
                }
                None => {
                    req.extensions_mut().insert((host, None::<Shop>));
                }
            }
        }

        self.service.call(req)
    }
}
