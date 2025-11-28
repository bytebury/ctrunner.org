use crate::SharedState;
use crate::{domain::User, extract::BaseUser};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{extract::FromRequestParts, http::request::Parts};

pub struct MaybeCurrentUser(pub Option<Box<User>>);

impl FromRequestParts<SharedState> for MaybeCurrentUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let user = BaseUser::from_request_parts(parts, state)
            .await
            .map_err(|_| Redirect::to("/").into_response())?;

        let user = match user {
            BaseUser::User(user) => Some(user),
            _ => None,
        };

        Ok(MaybeCurrentUser(user))
    }
}
