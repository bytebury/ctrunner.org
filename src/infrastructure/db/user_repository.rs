use crate::domain::user::UpdateUser;
use crate::domain::{User, user::NewUser};
use crate::util::pagination::{Paginatable, PaginatedResponse, Pagination};
use sqlx::{SqlitePool, query, query_as};
use std::sync::Arc;

pub struct UserRepository {
    db: Arc<SqlitePool>,
}
impl UserRepository {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<User, sqlx::Error> {
        query_as(r#"SELECT * FROM users WHERE id = ?"#)
            .bind(id)
            .fetch_one(self.db.as_ref())
            .await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        query_as(r#"SELECT * FROM users WHERE email = LOWER(?)"#)
            .bind(email)
            .fetch_optional(self.db.as_ref())
            .await
    }

    pub async fn update(&self, user: &UpdateUser) -> Result<User, sqlx::Error> {
        let _ = query(r#"UPDATE users SET role = ?, locked = ? WHERE id = ?"#)
            .bind(&user.role)
            .bind(user.locked)
            .bind(user.id)
            .execute(self.db.as_ref())
            .await?;

        self.find_by_id(user.id).await
    }

    pub async fn search(&self, pagination: &Pagination, search: &str) -> PaginatedResponse<User> {
        let pattern = &format!("%{}%", search.to_lowercase());

        User::paginate_filter(
            &self.db,
            pagination,
            Some(r#"LOWER(full_name) LIKE ? OR LOWER(email) LIKE ? ORDER BY updated_at DESC"#),
            vec![pattern, pattern],
        )
        .await
        .unwrap()
    }

    pub async fn create(&self, user: &NewUser) -> Result<User, sqlx::Error> {
        query_as(
            r#"
        INSERT INTO users (
            email, full_name, first_name, last_name, image_url, verified, locked
        )
        VALUES (LOWER(?), LOWER(?), LOWER(?), LOWER(?), ?, ?, ?)
        RETURNING *
        "#,
        )
        .bind(&user.email)
        .bind(&user.full_name)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.image_url)
        .bind(user.verified)
        .bind(user.locked)
        .fetch_one(self.db.as_ref())
        .await
    }
}
