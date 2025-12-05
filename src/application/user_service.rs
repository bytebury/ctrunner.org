use crate::{
    DbConnection,
    domain::{
        User,
        user::{NewUser, UpdateRunnerInfo, UpdateUser, UserView},
    },
    infrastructure::db::UserRepository,
    util::pagination::{PaginatedResponse, Pagination},
};

pub struct UserService {
    user_repository: UserRepository,
}
impl UserService {
    pub fn new(db: &DbConnection) -> Self {
        Self {
            user_repository: UserRepository::new(db),
        }
    }

    pub async fn find_by_id(&self, user_id: i64) -> Result<UserView, sqlx::Error> {
        self.user_repository.find_by_id(user_id).await
    }

    pub async fn find_by_runner_id(&self, runner_id: i64) -> Result<UserView, sqlx::Error> {
        self.user_repository.find_by_runner_id(runner_id).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        self.user_repository.find_by_email(email).await
    }

    pub async fn update(&self, user: &UpdateUser) -> Result<UserView, sqlx::Error> {
        self.user_repository.update(user).await
    }

    pub async fn update_runner_info(
        &self,
        user_id: i64,
        runner_info: &UpdateRunnerInfo,
    ) -> Result<User, sqlx::Error> {
        self.user_repository
            .update_runner_info(user_id, runner_info)
            .await
    }

    pub async fn search(
        &self,
        pagination: &Pagination,
        search: &str,
    ) -> PaginatedResponse<UserView> {
        self.user_repository.search(pagination, search).await
    }

    pub async fn create(&self, user: &NewUser) -> Result<User, sqlx::Error> {
        self.user_repository.create(user).await
    }
}
