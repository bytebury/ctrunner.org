use crate::domain::race::{NewRace, NewRaceForm};
use crate::extract::CurrentUser;
use crate::filters;
use crate::util::htmx::HTMX;
use crate::{
    domain::{
        race::{RaceSearchParams, RaceView},
        rbac::Role,
    },
    util::pagination::PaginatedResponse,
};
use askama::Template;
use askama_web::WebTemplate;
use axum::Form;
use axum::routing::put;
use axum::{
    Router,
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
};
use chrono::{Days, NaiveDateTime, NaiveTime, Utc};
use chrono_tz::America::New_York;

use crate::{SharedState, extract::MaybeCurrentUser, routes::SharedContext};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/races", put(add_race))
        .route("/upcoming-races", get(upcoming_races_page))
        .route("/upcoming-races/add", get(add_race_page))
}

#[derive(Template, WebTemplate)]
#[template(path = "races/upcoming.html")]
pub struct UpcomingRacesTemplate {
    shared: SharedContext,
    races: PaginatedResponse<RaceView>,
}

#[derive(Template, WebTemplate)]
#[template(path = "races/add_upcoming.html")]
pub struct AddUpcomingRaceTemplate {
    min_race_date: NaiveDateTime,
}

pub async fn add_race(
    State(state): State<SharedState>,
    CurrentUser(_): CurrentUser,
    Form(form): Form<NewRaceForm>,
) -> impl IntoResponse {
    // TODO: validate the input and handle error scenarios
    match state.race_service.get_or_create(NewRace::from(form)).await {
        Ok(_) => HTMX::refresh().into_response(),
        Err(_) => HTMX::refresh().into_response(),
    }
}

async fn upcoming_races_page(
    State(state): State<SharedState>,
    MaybeCurrentUser(user): MaybeCurrentUser,
    Query(params): Query<RaceSearchParams>,
) -> impl IntoResponse {
    UpcomingRacesTemplate {
        shared: SharedContext::new(&state.app_info, user.as_deref().cloned()),
        races: state.race_service.search_for_upcoming(params).await,
    }
}

async fn add_race_page(CurrentUser(_): CurrentUser) -> impl IntoResponse {
    AddUpcomingRaceTemplate {
        min_race_date: Utc::now()
            .with_timezone(&New_York)
            .date_naive()
            .checked_add_days(Days::new(1))
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
    }
}
