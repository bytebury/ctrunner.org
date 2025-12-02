use crate::{
    DbConnection,
    domain::race::{NewRace, NewRaceResult, RaceSearchParams, RaceView, SubmitTownSearchParams},
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
        self.race_repository.save_result(result).await
    }

    pub async fn get_or_create(&self, race: NewRace) -> Result<RaceView, String> {
        self.race_repository.get_or_create(race).await
    }

    pub async fn search_for_upcoming(
        &self,
        params: RaceSearchParams,
    ) -> PaginatedResponse<RaceView> {
        self.race_repository.search_for_upcoming(params).await
    }
}
