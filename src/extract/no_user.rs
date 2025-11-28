use crate::SharedState;
use crate::extract::BaseUser;
use axum::response::{IntoResponse, Redirect, Response};
use axum::{extract::FromRequestParts, http::request::Parts};

pub struct NoUser;

impl FromRequestParts<SharedState> for NoUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let user = BaseUser::from_request_parts(parts, state)
            .await
            .map_err(|_| Redirect::to("/").into_response())?;

        match user {
            BaseUser::User(_) => Err(Redirect::to("/dashboard").into_response()),
            _ => Ok(NoUser),
        }
    }
}
