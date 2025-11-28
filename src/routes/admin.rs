use crate::{
    SharedState,
    domain::{User, rbac::Role, user::UpdateUser},
    util::htmx::HTMX,
};

use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Form, Router,
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, patch},
};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    extract::admin_user::AdminUser,
    routes::SharedContext,
    util::pagination::{PaginatedResponse, Pagination},
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/admin/users", get(users))
        .route("/admin/users/{id}", get(view_user))
        .route("/admin/users/{id}", patch(edit_user))
}

#[derive(Deserialize)]
struct UserSearch {
    page_size: Option<i64>,
    page: Option<i64>,
    q: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "admin/users.html")]
struct AdminUsersTemplate {
    shared: SharedContext,
    users: PaginatedResponse<User>,
}

#[derive(Template, WebTemplate)]
#[template(path = "admin/view_user.html")]
struct AdminViewUserTemplate {
    user: User,
}

#[derive(Deserialize)]
struct UpdateUserForm {
    locked: Option<String>,
    role: Role,
}

async fn users(
    State(state): State<SharedState>,
    AdminUser(user): AdminUser,
    Query(params): Query<UserSearch>,
) -> impl IntoResponse {
    let pagination = Pagination {
        page: params.page,
        page_size: params.page_size,
    };
    AdminUsersTemplate {
        shared: SharedContext::new(&state.app_info, Some(*user)),
        users: state
            .user_service
            .search(&pagination, &params.q.unwrap_or_default())
            .await,
    }
}

async fn view_user(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    match state.user_service.find_by_id(user_id).await {
        Ok(user) => AdminViewUserTemplate { user }.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn edit_user(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Path(user_id): Path<i64>,
    Form(form): Form<UpdateUserForm>,
) -> impl IntoResponse {
    let user = match state.user_service.find_by_id(user_id).await {
        Ok(user) => user,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    let mut user = UpdateUser::from(user);
    user.locked = form.locked.is_some();
    user.role = form.role;

    match state.user_service.update(&user).await {
        Ok(_) => HTMX::refresh().into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
