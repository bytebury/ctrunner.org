use sqlx::query_as;

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
}
