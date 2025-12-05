use crate::domain::Town;
use crate::domain::user::UpdateRunnerInfo;
use crate::{
    SharedState,
    domain::rbac::Role,
    util::{htmx::HTMX, validation::Validate},
};
use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Form, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, patch},
};

use crate::{
    extract::{CurrentUser, NoUser, OrphanUser},
    routes::SharedContext,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(homepage))
        .route("/dashboard", get(dashboard))
        .route("/completed-towns-map", get(completed_towns_map))
        .route("/update-info", get(update_runner_info_page))
        .route("/update-info", patch(update_runner_info))
}

#[derive(Template, WebTemplate)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    shared: SharedContext,
}

#[derive(Template, WebTemplate)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    shared: SharedContext,
}

#[derive(Template, WebTemplate)]
#[template(path = "completed_towns_map.html")]
struct CompletedTownsMapTemplate {
    completed_towns: Vec<Town>,
}

#[derive(Template, WebTemplate, Default)]
#[template(path = "update_runner_info.html")]
struct UpdateRunnerInfoTemplate {
    shared: SharedContext,
    towns: Vec<Town>,
    form: UpdateRunnerInfo,
    form_error_message: Option<String>,
}

async fn homepage(State(state): State<SharedState>, NoUser: NoUser) -> HomepageTemplate {
    HomepageTemplate {
        shared: SharedContext::new(&state.app_info, None),
    }
}

async fn dashboard(
    State(state): State<SharedState>,
    CurrentUser(user): CurrentUser,
) -> DashboardTemplate {
    DashboardTemplate {
        shared: SharedContext::new(&state.app_info, Some(*user.clone())),
    }
}

async fn completed_towns_map(
    State(state): State<SharedState>,
    CurrentUser(user): CurrentUser,
) -> CompletedTownsMapTemplate {
    CompletedTownsMapTemplate {
        completed_towns: state.town_service.find_completed(user.id).await,
    }
}

async fn update_runner_info_page(
    State(state): State<SharedState>,
    OrphanUser(user): OrphanUser,
) -> UpdateRunnerInfoTemplate {
    UpdateRunnerInfoTemplate {
        shared: SharedContext::new(&state.app_info, Some(*user)),
        towns: state.town_service.find_all().await,
        ..Default::default()
    }
}

async fn update_runner_info(
    State(state): State<SharedState>,
    OrphanUser(user): OrphanUser,
    Form(form): Form<UpdateRunnerInfo>,
) -> impl IntoResponse {
    let user_id = user.id;
    let towns = state.town_service.find_all().await;
    let error_response = |error_message: &str, form| {
        UpdateRunnerInfoTemplate {
            shared: SharedContext::new(&state.app_info, Some(*user)),
            form,
            form_error_message: Some(error_message.to_string()),
            towns,
        }
        .into_response()
    };

    if let Err(message) = form.validate() {
        return error_response(&message, form);
    }

    // TODO: we'll need to specify the error type here
    match state.user_service.update_runner_info(user_id, &form).await {
        Ok(_) => HTMX::redirect("/dashboard").into_response(),
        Err(error) => error_response(&error.to_string(), form),
    }
}
