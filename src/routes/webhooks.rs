use axum::{
    Router,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
};
use log::error;
use stripe_webhooks::StripeEvent;

use crate::{SharedState, infrastructure::payment::stripe::Stripe};

pub fn routes() -> Router<SharedState> {
    Router::new().route("/webhooks/stripe", post(stripe_webhook_listener))
}

pub async fn stripe_webhook_listener(headers: HeaderMap, body: String) -> impl IntoResponse {
    match Stripe::process_webhook_request(&headers, &body) {
        Ok(event) => {
            match event {
                StripeEvent::CheckoutSessionCompleted(value) => println!("{:?}", value),
                StripeEvent::InvoicePaymentFailed(value)
                | StripeEvent::CustomerSubscriptionDeleted(value) => println!("{:?}", value),
                StripeEvent::Unknown(value) => println!("{:?}", value),
            };
        }
        Err(e) => {
            error!("Error processing Stripe Event: {e:?}");
            return (StatusCode::BAD_REQUEST, "Error processing event.").into_response();
        }
    };

    StatusCode::OK.into_response()
}
