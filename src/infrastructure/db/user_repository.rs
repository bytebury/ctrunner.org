use crate::DbConnection;
use crate::domain::user::{UpdateRunnerInfoForm, UpdateUser};
use crate::domain::{User, user::NewUser};
use crate::util::pagination::{Paginatable, PaginatedResponse, Pagination};
use sqlx::{query, query_as};

pub struct UserRepository {
    db: DbConnection,
}
impl UserRepository {
    pub fn new(db: &DbConnection) -> Self {
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
        let _ = query(
            r#"UPDATE users SET role = ?, locked = ?, updated_at = current_timestamp WHERE id = ?"#,
        )
        .bind(&user.role)
        .bind(user.locked)
        .bind(user.id)
        .execute(self.db.as_ref())
        .await?;

        self.find_by_id(user.id).await
    }

    pub async fn update_runner_info(
        &self,
        user_id: i64,
        user: &UpdateRunnerInfoForm,
    ) -> Result<User, sqlx::Error> {
        query_as(r#"UPDATE users SET first_name = ?, last_name = ?, runner_id = ?, hometown_id = ?, full_name = ?, updated_at = current_timestamp WHERE id = ? RETURNING *"#)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(&user.runner_id)
            .bind(&user.hometown_id)
            .bind(format!("{} {}", user.first_name, user.last_name))
            .bind(user_id)
            .fetch_one(self.db.as_ref())
            .await
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
