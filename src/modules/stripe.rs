use crate::utils::constants::STRIPE_SECRET;
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer};
use serde_json::json;
use std::borrow::Borrow;
use stripe::{
    ApiErrors, CancelSubscription, CheckoutSession, CheckoutSessionMode, Client,
    CreateCheckoutSession, CreateCheckoutSessionLineItems, CreatePrice, CreatePriceRecurring,
    CreatePriceRecurringInterval, CreateProduct, Currency, IdOrCreate, PaymentIntent, Price,
    Product, SubscriptionId, UpdateSubscription, UpdateSubscriptionItems,
};
use stripe::{EventObject, EventType, Webhook, WebhookError};

enum StripeCurrency {
    USD,
    EUR,
    GBP,
}
impl StripeCurrency {
    fn currency(&self) -> Currency {
        match self {
            StripeCurrency::USD => Currency::USD,
            StripeCurrency::EUR => Currency::EUR,
            StripeCurrency::GBP => Currency::GBP,
        }
    }
}

pub struct Stripe {
    pub client: Client,
}

impl Stripe {
    pub fn new() -> Self {
        let client = stripe::Client::new(STRIPE_SECRET.as_str());
        Stripe { client }
    }
    // Create a new product
    pub async fn create_product(&self, name: &str) -> Product {
        let product = {
            let mut create_product = CreateProduct::new(name);
            create_product.metadata = Some(std::collections::HashMap::from([(
                String::from("async-stripe"),
                String::from("true"),
            )]));
            Product::create(&self.client, create_product).await.unwrap()
        };
        product
    }

    // Create a new price
    async fn create_product_price(client: &Client, product: &Product) -> Result<Price, ApiErrors> {
        let price = {
            let mut create_price = CreatePrice::new(Currency::USD);
            create_price.product = Some(IdOrCreate::Id(&product.id));
            create_price.metadata = Some(std::collections::HashMap::from([(
                String::from("async-stripe"),
                String::from("true"),
            )]));
            create_price.unit_amount = Some(1000);

            create_price.recurring = Some(CreatePriceRecurring {
                interval: CreatePriceRecurringInterval::Month,
                ..Default::default()
            });
            create_price.expand = &["product"];
            Price::create(client, create_price)
                .await
                .expect("Failed to create price")
        };

        Ok(price)
    }

    async fn cancel_subscription(&self, subscription_id: String) -> Result<(), stripe::Error> {
        let _ = Subscription::cancel(
            &self.client,
            &SubscriptionId::from_str(subscription_id).unwrap(),
            CancelSubscription {
                cancellation_details: None,
                invoice_now: Some(true),
                prorate: Some(true),
            },
        )
        .await?;

        Ok(())
    }

    async fn update_subscription(
        &self,
        user_subscription_id: String,
        old_item_id: String,
        new_item_id: String,
        new_price_id: String,
    ) -> Result<(), stripe::Error> {
        let subscription_item = Subscription::retrieve(
            &self.client,
            &SubscriptionId::from_str(&user_subscription_id).unwrap(),
            &["items"],
        )
        .await?
        .items;

        let subscription_item = &subscription_item.data[0];

        let _ = Subscription::update(
            &self.client,
            &SubscriptionId::from_str(&user_subscription_id).unwrap(),
            UpdateSubscription {
                items: Some(vec![
                    UpdateSubscriptionItems {
                        id: Some(old_item_id),
                        deleted: Some(true),
                        ..Default::default()
                    },
                    UpdateSubscriptionItems {
                        id: Some(new_item_id),
                        price: Some(new_price_id).Default::default(),
                    },
                ]),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    async fn create_checkout_session(&self, customer_id: String) -> String {
        let price = {
            let mut create_price = CreatePrice::new(Currency::USD);
            create_price.product = Some(IdOrCreate::Id(&product.id));
            create_price.metadata = Some(std::collections::HashMap::from([(
                String::from("async-stripe"),
                String::from("true"),
            )]));
            create_price.unit_amount = Some(1000);
            create_price.expand = &["product"];
            Price::create(&self.client, create_price).await.unwrap()
        };

        let checkout_session = {
            let mut params = CreateCheckoutSession::new();
            params.cancel_url = Some("http://test.com/cancel");
            params.success_url = Some("http://test.com/success");
            params.customer = Some(customer_id);
            params.mode = Some(CheckoutSessionMode::Payment);
            params.line_items = Some(vec![CreateCheckoutSessionLineItems {
                quantity: Some(1),
                price: Some(price.id.to_string()),
                ..Default::default()
            }]);
            params.expand = &["line_items", "line_items.data.price.product"];

            CheckoutSession::create(&self.client, params).await.unwrap()
        };

        let line_items = checkout_session.line_items.unwrap();

        checkout_session.url.unwrap()
    }
}

// #[post("stripe_webhooks")]
// pub async fn webhook_handler(req: HttpRequest, payload: web::Bytes) -> HttpResponse {
//     handle_webhook(req, payload).unwrap();
//     HttpResponse::Ok().finish()
// }

pub fn handle_webhook(req: HttpRequest, payload: web::Bytes) -> Result<(), WebhookError> {
    let payload_str = std::str::from_utf8(payload.borrow()).unwrap();

    let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    if let Ok(event) = Webhook::construct_event(payload_str, stripe_signature, "whsec_xxxxx") {
        match event.type_ {
            EventType::AccountUpdated => {
                if let EventObject::Account(account) = event.data.object {
                    handle_account_updated(account)?;
                }
            }
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    handle_checkout_session(session)?;
                }
            }
            _ => {
                println!("Unknown event encountered in webhook: {:?}", event.type_);
            }
        }
    } else {
        println!("Failed to construct webhook event, ensure your webhook secret is correct.");
    }

    Ok(())
}

fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
    req.headers().get(key)?.to_str().ok()
}

fn handle_account_updated(account: stripe::Account) -> Result<(), WebhookError> {
    println!(
        "Received account updated webhook for account: {:?}",
        account.id
    );
    Ok(())
}

fn handle_checkout_session(session: stripe::CheckoutSession) -> Result<(), WebhookError> {
    println!(
        "Received checkout session completed webhook with id: {:?}",
        session.id
    );
    Ok(())
}
