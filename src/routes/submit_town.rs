use crate::{
    SharedState,
    domain::{Town, distance::DistanceUnit, town::SubmitTown},
    extract::current_user::CurrentUser,
};

use askama::Template;
use askama_web::WebTemplate;
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
        .route("/submit", get(submit_town_modal))
        .route("/submit", put(submit_town))
}

#[derive(Template, WebTemplate, Default)]
#[template(path = "submit_town.html")]
struct SubmitTownPage {
    _form: SubmitTown,
    towns: Vec<Town>,
    max_race_date: NaiveDate,
}

async fn submit_town_modal(
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
