use crate::SharedState;
use crate::domain::user::UserView;
use crate::extract::BaseUser;
use axum::response::{IntoResponse, Redirect, Response};
use axum::{extract::FromRequestParts, http::request::Parts};

pub struct OrphanUser(pub Box<UserView>);

impl FromRequestParts<SharedState> for OrphanUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let user = BaseUser::from_request_parts(parts, state)
            .await
            .map_err(|_| Redirect::to("/").into_response())?;

        match user {
            BaseUser::User(user) => match user.runner_id {
                Some(_) => Err(Redirect::to("/dashboard").into_response()),
                None => Ok(OrphanUser(user)),
            },
            _ => Err(Redirect::to("/").into_response()),
        }
    }
}
