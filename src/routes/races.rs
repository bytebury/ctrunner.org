use axum::{Router, extract::State, routing::get};

use crate::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new().route("/races", get(races))
}

async fn races(State(_): State<SharedState>) {
    todo!()
}
