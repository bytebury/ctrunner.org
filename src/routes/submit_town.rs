use crate::{
    SharedState,
    domain::{
        Town,
        town::{SubmitTown, SubmitTownFormIds},
    },
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
    form: SubmitTown,
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
    match SubmitTownFormIds::new()
        .add_answers(*user, form)
        .submit()
        .await
        .inspect_err(|e| println!("{e}"))
    {
        Ok(_) => SubmitTownPage {
            towns: state.town_service.find_all().await,
            max_race_date: Utc::now().with_timezone(&New_York).date_naive(),
            ..Default::default()
        }
        .into_response(),
        // TODO: Need to add error message when something goes wrong.
        Err(_) => SubmitTownPage {
            towns: state.town_service.find_all().await,
            max_race_date: Utc::now().with_timezone(&New_York).date_naive(),
            ..Default::default()
        }
        .into_response(),
    }
}
