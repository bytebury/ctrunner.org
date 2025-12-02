use crate::domain::distance::DistanceUnit;
use crate::domain::race::{RaceView, SubmitTownSearchParams};
use crate::filters;
use crate::util::pagination::PaginatedResponse;
use crate::{
    SharedState,
    domain::{Town, town::SubmitTown},
    extract::current_user::CurrentUser,
};

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Query;
use axum::{
    Form, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, put},
};
use chrono::{NaiveDate, Utc};
use chrono_tz::America::New_York;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/submit-town", get(submit_town_page))
        .route("/submit-town", put(submit_town))
        .route("/submit-town/search", get(search_races))
}

#[derive(Template, WebTemplate, Default)]
#[template(path = "submit_town/submit_town.html")]
struct SubmitTownPage {
    _form: SubmitTown,
    towns: Vec<Town>,
    max_race_date: NaiveDate,
}

#[derive(Template, WebTemplate)]
#[template(path = "submit_town/race_autocomplete.html")]
struct RaceAutocompletePage {
    races: PaginatedResponse<RaceView>,
}

async fn search_races(
    State(state): State<SharedState>,
    CurrentUser(_): CurrentUser,
    Query(params): Query<SubmitTownSearchParams>,
) -> impl IntoResponse {
    RaceAutocompletePage {
        races: state.race_service.submit_town_search(&params).await,
    }
}

async fn submit_town_page(
    State(state): State<SharedState>,
    CurrentUser(_): CurrentUser,
) -> SubmitTownPage {
    SubmitTownPage {
        towns: state.town_service.find_all().await,
        max_race_date: Utc::now().with_timezone(&New_York).date_naive(),
        ..Default::default()
    }
}

async fn submit_town(
    State(state): State<SharedState>,
    CurrentUser(user): CurrentUser,
    Form(form): Form<SubmitTown>,
) -> impl IntoResponse {
    // TODO: We'll need to handle the error scenario and success scenario.
    //       We should celebrate the user's achievement!
    match state.town_service.submit_completed_town(*user, form).await {
        Ok(_) => SubmitTownPage {
            towns: state.town_service.find_all().await,
            max_race_date: Utc::now().with_timezone(&New_York).date_naive(),
            ..Default::default()
        }
        .into_response(),
        Err(_) => SubmitTownPage {
            towns: state.town_service.find_all().await,
            max_race_date: Utc::now().with_timezone(&New_York).date_naive(),
            ..Default::default()
        }
        .into_response(),
    }
}
