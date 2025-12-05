use crate::domain::{rbac::Role, user::UserView};
use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde::Deserialize;

use crate::{
    SharedState,
    extract::maybe_current_user::MaybeCurrentUser,
    routes::SharedContext,
    util::pagination::{PaginatedResponse, Pagination},
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/members", get(members))
        .route("/members/{username}", get(profile_page))
}

#[derive(Deserialize)]
struct MemberSearch {
    page_size: Option<i64>,
    page: Option<i64>,
    q: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "members/members.html")]
struct MembersTemplate {
    shared: SharedContext,
    users: PaginatedResponse<UserView>,
}

#[derive(Template, WebTemplate)]
#[template(path = "members/profile_page.html")]
struct ProfilePageTemplate {
    shared: SharedContext,
    user: UserView,
}

async fn members(
    State(state): State<SharedState>,
    MaybeCurrentUser(user): MaybeCurrentUser,
    Query(params): Query<MemberSearch>,
) -> MembersTemplate {
    let pagination = Pagination {
        page: params.page,
        page_size: params.page_size,
    };
    MembersTemplate {
        shared: SharedContext::new(&state.app_info, user.as_deref().cloned()),
        users: state
            .user_service
            .search(&pagination, &params.q.unwrap_or_default())
            .await,
    }
}

async fn profile_page(
    State(state): State<SharedState>,
    MaybeCurrentUser(current_user): MaybeCurrentUser,
    Path(username): Path<i64>,
) -> ProfilePageTemplate {
    // TODO: Handle user not found.
    let user = state
        .user_service
        .find_by_runner_id(username)
        .await
        .unwrap();
    ProfilePageTemplate {
        shared: SharedContext::new(&state.app_info, current_user.as_deref().cloned()),
        user,
    }
}
