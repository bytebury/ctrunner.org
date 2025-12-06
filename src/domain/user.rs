use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    domain::{Town, rbac::Role},
    infrastructure::auth::GoogleUser,
    util::{StringExt, pagination::Paginatable, validation::Validate},
};

pub struct UpdateUser {
    pub id: i64,
    pub locked: bool,
    pub role: Role,
}

impl From<UserView> for UpdateUser {
    fn from(user: UserView) -> Self {
        Self {
            id: user.id,
            locked: user.locked,
            role: user.role,
        }
    }
}

pub struct NewUser {
    pub id: i64,
    pub runner_id: Option<String>,
    pub email: String,
    pub verified: bool,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub image_url: String,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl From<GoogleUser> for NewUser {
    fn from(google_user: GoogleUser) -> Self {
        Self {
            id: 0,
            runner_id: None,
            email: google_user.email,
            verified: google_user.email_verified,
            first_name: google_user.given_name.unwrap_or(google_user.name.clone()),
            last_name: google_user.family_name.unwrap_or("".to_string()),
            full_name: google_user.name,
            image_url: google_user.picture,
            locked: false,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub runner_id: Option<i64>,
    pub hometown_id: Option<i64>,
    pub email: String,
    pub verified: bool,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub image_url: String,
    pub role: Role,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct UserView {
    pub id: i64,
    pub runner_id: Option<i64>,
    pub hometown_id: Option<i64>,
    pub hometown: Option<String>,
    pub hometown_county_id: Option<i64>,
    pub hometown_county: Option<String>,
    pub completed_towns_count: i64,
    pub email: String,
    pub verified: bool,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub image_url: String,
    pub role: Role,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserView {
    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }
}

impl Paginatable for UserView {
    fn table_name() -> &'static str {
        "users_view"
    }
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct UpdateRunnerInfo {
    pub runner_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub hometown_id: i64,
    pub towns: Option<Vec<i64>>,
}

impl Validate for UpdateRunnerInfo {
    fn validate(&self) -> Result<(), String> {
        if !(1..10_000).contains(&self.runner_id) {
            return Err("Member ID is not valid".to_string());
        }

        if self.first_name.is_whitespace_or_empty() {
            return Err("First name cannot be empty".to_string());
        }

        if self.last_name.is_whitespace_or_empty() {
            return Err("Last name cannot be empty".to_string());
        }

        if self.first_name.len() > 25 {
            return Err("First name cannot be longer than 25 characters".to_string());
        }

        if self.last_name.len() > 25 {
            return Err("Last name cannot be longer than 25 characters".to_string());
        }

        if Town::is_not_valid(self.hometown_id) {
            return Err("Town is not valid".to_string());
        }

        Ok(())
    }
}
