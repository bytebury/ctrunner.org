use std::iter::Filter;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    domain::rbac::Role,
    infrastructure::auth::GoogleUser,
    util::{StringExt, pagination::Paginatable, validation::Validate},
};

pub struct UpdateUser {
    pub id: i64,
    pub locked: bool,
    pub role: Role,
}

impl From<User> for UpdateUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            locked: user.locked,
            role: user.role,
        }
    }
}

pub struct NewUser {
    pub id: i64,
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

impl User {
    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }
}

impl Paginatable for User {
    fn table_name() -> &'static str {
        "users"
    }
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct UpdateRunnerInfoForm {
    pub runner_id: String,
    pub first_name: String,
    pub last_name: String,
    pub hometown_id: Option<String>,
}

impl Validate for UpdateRunnerInfoForm {
    fn validate(&self) -> Result<(), String> {
        if self.runner_id.is_whitespace_or_empty() {
            return Err("Member ID cannot be empty".to_string());
        }

        if self.first_name.is_whitespace_or_empty() {
            return Err("First name cannot be empty".to_string());
        }

        if self.last_name.is_whitespace_or_empty() {
            return Err("Last name cannot be empty".to_string());
        }

        RunnerId::try_from(self.runner_id.as_ref())?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RunnerId(i64);

impl RunnerId {
    pub fn value(&self) -> i64 {
        self.0
    }

    fn parse(id: i64) -> Result<i64, String> {
        if (1..10_000).contains(&id) {
            return Ok(id);
        }
        Err("Invalid Member ID".to_string())
    }
}

impl TryFrom<&str> for RunnerId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id = value.parse::<i64>().map_err(|_| "Invalid Member ID")?;
        Self::parse(id).map(Self)
    }
}
