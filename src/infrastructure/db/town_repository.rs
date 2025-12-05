use sqlx::query;
use sqlx::query_as;

use crate::domain::town::CompletedTown;
use crate::{DbConnection, domain::Town};

pub struct TownRepository {
    db: DbConnection,
}

impl TownRepository {
    pub fn new(db: &DbConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_all(&self) -> Vec<Town> {
        query_as("SELECT * FROM towns_view ORDER BY name ASC")
            .fetch_all(self.db.as_ref())
            .await
            .unwrap_or_default()
    }

    pub async fn find_completed(&self, user_id: i64) -> Vec<CompletedTown> {
        query_as("SELECT * FROM completed_towns_view WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(self.db.as_ref())
            .await
            .unwrap_or_default()
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Town, String> {
        query_as("SELECT * FROM towns_view WHERE id = ?")
            .bind(id)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(|_| "Unable to find that town".to_string())
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Town, String> {
        query_as("SELECT * FROM towns_view WHERE name = ?")
            .bind(name)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(|_| "Unable to find that town".to_string())
    }

    pub async fn mark_completed(&self, user_id: i64, town_id: i64) -> Result<(), String> {
        query("INSERT INTO completed_towns (user_id, town_id) VALUES(?, ?)")
            .bind(user_id)
            .bind(town_id)
            .execute(self.db.as_ref())
            .await
            .map_err(|_| "Unable to mark town as completed".to_string())?;
        Ok(())
    }
}
