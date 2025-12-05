use crate::{
    DbConnection,
    domain::{
        google_sheet::GoogleSheet,
        race::{
            NewRace, NewRaceResult, RaceSearchParams, RaceView, SubmitTownSearchParams,
            UpcomingRaceFromRun169Society,
        },
    },
    infrastructure::db::{RaceRepository, TownRepository},
    util::pagination::PaginatedResponse,
};

pub struct RaceService {
    race_repository: RaceRepository,
    town_repository: TownRepository,
}

impl RaceService {
    pub fn new(db: &DbConnection) -> Self {
        Self {
            race_repository: RaceRepository::new(db),
            town_repository: TownRepository::new(db),
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

    pub async fn upcoming_races_nightly(&self) -> Result<(), String> {
        let races: Vec<UpcomingRaceFromRun169Society> = GoogleSheet::upcoming_races().await?;

        for race in races {
            let town_id = match self.town_repository.find_by_name(&race.town_name).await {
                Ok(town) => town.id,
                Err(_) => continue,
            };

            let race = NewRace {
                town_id,
                name: race.name.to_string(),
                start_at: race.start_at,
                race_url: Some(race.race_url.to_string()),
                miles: race.miles,
            };

            match self.race_repository.get_or_create(race).await {
                Ok(race) => race,
                Err(_) => continue,
            };
        }

        Ok(())
    }
}
