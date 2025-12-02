use crate::{
    DbConnection,
    domain::race::{NewRaceResult, RaceView, SubmitTownSearchParams},
    infrastructure::db::RaceRepository,
    util::pagination::PaginatedResponse,
};

pub struct RaceService {
    race_repository: RaceRepository,
}

impl RaceService {
    pub fn new(db: &DbConnection) -> Self {
        Self {
            race_repository: RaceRepository::new(db),
        }
    }

    pub async fn submit_town_search(
        &self,
        params: &SubmitTownSearchParams,
    ) -> PaginatedResponse<RaceView> {
        self.race_repository.submit_town_search(params).await
    }

    pub async fn submit_result(&self, result: NewRaceResult) -> Result<(), String> {
        self.race_repository.save(result).await
    }
}
