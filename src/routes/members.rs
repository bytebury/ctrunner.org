use crate::domain::rbac::Role;
use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Router,
    extract::{Query, State},
    routing::get,
};
use serde::Deserialize;

use crate::{
    SharedState,
    domain::User,
    extract::maybe_current_user::MaybeCurrentUser,
    routes::SharedContext,
    util::pagination::{PaginatedResponse, Pagination},
};

pub fn routes() -> Router<SharedState> {
    Router::new().route("/members", get(members))
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
    users: PaginatedResponse<User>,
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
