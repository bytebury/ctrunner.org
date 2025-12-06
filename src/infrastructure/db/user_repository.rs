use crate::DbConnection;
use crate::domain::user::{UpdateRunnerInfo, UpdateUser, UserView};
use crate::domain::{User, user::NewUser};
use crate::util::pagination::{Paginatable, PaginatedResponse, Pagination};
use sqlx::{query, query_as, query_scalar};

pub struct UserRepository {
    db: DbConnection,
}
impl UserRepository {
    pub fn new(db: &DbConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<UserView, sqlx::Error> {
        query_as(r#"SELECT * FROM users_view WHERE id = ?"#)
            .bind(id)
            .fetch_one(self.db.as_ref())
            .await
    }

    pub async fn find_by_runner_id(&self, runner_id: i64) -> Result<UserView, sqlx::Error> {
        query_as(r#"SELECT * FROM users_view WHERE runner_id = ?"#)
            .bind(runner_id)
            .fetch_one(self.db.as_ref())
            .await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserView>, sqlx::Error> {
        query_as(r#"SELECT * FROM users_view WHERE email = LOWER(?)"#)
            .bind(email)
            .fetch_optional(self.db.as_ref())
            .await
    }

    pub async fn update(&self, user: &UpdateUser) -> Result<UserView, sqlx::Error> {
        let _ = query(r#"UPDATE users SET role = ?, locked = ? WHERE id = ?"#)
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
        user: &UpdateRunnerInfo,
    ) -> Result<User, sqlx::Error> {
        query_as(r#"UPDATE users SET first_name = ?, last_name = ?, runner_id = ?, hometown_id = ?, full_name = ? WHERE id = ? RETURNING *"#)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(user.runner_id)
            .bind(user.hometown_id)
            .bind(format!("{} {}", user.first_name, user.last_name))
            .bind(user_id)
            .fetch_one(self.db.as_ref())
            .await
    }

    pub async fn search(
        &self,
        pagination: &Pagination,
        search: &str,
    ) -> PaginatedResponse<UserView> {
        let pattern = &format!("%{}%", search.to_lowercase());

        UserView::paginate_filter(
            &self.db,
            pagination,
            Some(r#"(LOWER(full_name) LIKE ? OR LOWER(email) LIKE ?) AND runner_id IS NOT NULL ORDER BY full_name ASC"#),
            vec![pattern, pattern],
        )
        .await
        .unwrap()
    }

    pub async fn create(&self, user: &NewUser) -> Result<UserView, sqlx::Error> {
        let created_id: i64 = query_scalar(
            r#"
            INSERT INTO users (
                email, full_name, first_name, last_name, image_url, verified, locked
            )
            VALUES (LOWER(?), LOWER(?), LOWER(?), LOWER(?), ?, ?, ?)
            RETURNING id
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
        .await?;

        let user_view: UserView = query_as(r#"SELECT * FROM users_view WHERE id = ?"#)
            .bind(created_id)
            .fetch_one(self.db.as_ref())
            .await?;

        Ok(user_view)
    }
}
