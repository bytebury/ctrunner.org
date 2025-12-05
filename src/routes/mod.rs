use crate::{AppInfo, domain::user::UserView};

pub mod admin;
pub mod auth;
pub mod homepage;
pub mod members;
pub mod races;
pub mod submit_town;

#[derive(Default)]
pub struct SharedContext {
    pub app_info: AppInfo,
    pub current_user: Option<UserView>,
}
impl SharedContext {
    pub fn new(app_info: &AppInfo, user: Option<UserView>) -> Self {
        Self {
            app_info: app_info.clone(),
            current_user: user,
        }
    }
}
