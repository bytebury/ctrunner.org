use crate::{
    domain::{
        race::{RaceSearchParams, RaceView},
        rbac::Role,
    },
    util::pagination::PaginatedResponse,
};
use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Router,
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
};

use crate::{SharedState, extract::MaybeCurrentUser, routes::SharedContext};

pub fn routes() -> Router<SharedState> {
    Router::new().route("/upcoming-races", get(upcoming_races))
}

#[derive(Template, WebTemplate)]
#[template(path = "races/upcoming.html")]
pub struct UpcomingRacesTemplate {
    shared: SharedContext,
    races: PaginatedResponse<RaceView>,
}

async fn upcoming_races(
    State(state): State<SharedState>,
    MaybeCurrentUser(user): MaybeCurrentUser,
    Query(params): Query<RaceSearchParams>,
) -> impl IntoResponse {
    UpcomingRacesTemplate {
        shared: SharedContext::new(&state.app_info, user.as_deref().cloned()),
        races: state.race_service.search_for_upcoming(params).await,
    }
}
