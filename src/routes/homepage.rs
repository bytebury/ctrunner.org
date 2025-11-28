use crate::{SharedState, domain::rbac::Role};
use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, routing::get};

use crate::{
    extract::{current_user::CurrentUser, no_user::NoUser},
    routes::SharedContext,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(homepage))
        .route("/dashboard", get(dashboard))
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

async fn homepage(State(state): State<SharedState>, NoUser: NoUser) -> HomepageTemplate {
    HomepageTemplate {
        shared: SharedContext::new(&state.app_info, None),
    }
}

async fn dashboard(
    State(state): State<SharedState>,
    CurrentUser(current_user): CurrentUser,
) -> DashboardTemplate {
    DashboardTemplate {
        shared: SharedContext::new(&state.app_info, Some(*current_user)),
    }
}
