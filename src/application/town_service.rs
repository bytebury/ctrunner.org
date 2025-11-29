use crate::{DbConnection, domain::Town, infrastructure::db::TownRepository};

pub struct TownService {
    town_repository: TownRepository,
}

impl TownService {
    pub fn new(db: &DbConnection) -> Self {
        TownService {
            town_repository: TownRepository::new(db),
        }
    }

    pub async fn find_all(&self) -> Vec<Town> {
        self.town_repository.find_all().await
    }
}
