use crate::utils::constants::STRIPE_SECRET;
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer};
use serde_json::json;
use std::borrow::Borrow;
use stripe::{
    AccountLink, AccountLinkType, ApiErrors, CancelSubscription, CheckoutSession,
    CheckoutSessionMode, Client, CreateAccountLink, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreatePrice, CreatePriceRecurring,
    CreatePriceRecurringInterval, CreateProduct, Currency, IdOrCreate, PaymentIntent, Price,
    Product, StripeError, SubscriptionId, UpdateSubscription, UpdateSubscriptionItems,
};
use stripe::{EventObject, EventType, Webhook, WebhookError};

pub enum StripeCurrency {
    USD,
    EUR,
    GBP,
}
impl StripeCurrency {
    pub fn currency(&self) -> Currency {
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
    // Create new Account for Stripe Connect
    async fn create_account_stripe_connect(&self) -> Result<stripe::Account, StripeError> {
        let account = stripe::Account::create(
            &self.client,
            stripe::CreateAccount {
                type_: Some(stripe::AccountType::Express),
                capabilities: Some(stripe::CreateAccountCapabilities {
                    card_payments: Some(stripe::CreateAccountCapabilitiesCardPayments {
                        requested: Some(true),
                    }),
                    transfers: Some(stripe::CreateAccountCapabilitiesTransfers {
                        requested: Some(true),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await;
        account
    }

    // Linking Account with Stripe Connect
    pub async fn create_account_link_stripe_connect(
        &self,
        ref_url: &str,
        ret_url: &str,
    ) -> Result<stripe::AccountLink, StripeError> {
        let account = self.create_account_stripe_connect().await;
        if account.is_err() {
            return Err(account.err().unwrap());
        }
        let account = account.unwrap();
        let link = stripe::AccountLink::create(
            &self.client,
            stripe::CreateAccountLink {
                account: account.id,
                type_: AccountLinkType::AccountOnboarding,
                collect: None,
                expand: &[],
                refresh_url: Some(ref_url),
                return_url: Some(ret_url),
                collection_options: None,
            },
        )
        .await;
        link
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
    pub async fn create_product_price(
        client: &Client,
        product: &Product,
    ) -> Result<Price, ApiErrors> {
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
}
