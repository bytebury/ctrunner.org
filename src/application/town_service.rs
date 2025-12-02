use crate::{
    DbConnection,
    domain::{
        Town, User,
        distance::{DistanceUnit, Kilometers, Miles},
        race::NewRace,
        town::{Run169TownsSocietyGoogleForm, SubmitTown, SubmitTownForGoogle},
    },
    infrastructure::db::{RaceRepository, TownRepository},
};

pub struct TownService {
    town_repository: TownRepository,
    race_repository: RaceRepository,
}

impl TownService {
    pub fn new(db: &DbConnection) -> Self {
        Self {
            town_repository: TownRepository::new(db),
            race_repository: RaceRepository::new(db),
        }
    }

    pub async fn find_all(&self) -> Vec<Town> {
        self.town_repository.find_all().await
    }

    pub async fn submit_completed_town(&self, user: User, form: SubmitTown) -> Result<(), String> {
        let user_id = user.id;
        let town_id = form.town_id;
        let town_name = self.town_repository.find_by_id(form.town_id).await?.name;
        let distance_val = match form.distance_unit {
            DistanceUnit::Miles => Miles::new(form.distance_val),
            DistanceUnit::Kilometers => Kilometers::new(form.distance_val).to_miles(),
        };

        let google_form = SubmitTownForGoogle {
            distance_val,
            town_name,
            race_name: form.race_name.clone(),
            race_date: form.race_date,
            notes: form.notes.clone(),
        };

        // Submit the town to Run169Towns Society
        Run169TownsSocietyGoogleForm::new()
            .add_answers(user, google_form)
            .submit()
            .await?;

        // Mark the town as completed.
        let _ = self.town_repository.mark_completed(user_id, town_id).await;
        // Always try to create a race, it will reject if there's one that exists.
        let _ = self.race_repository.create_race(NewRace::from(form)).await;

        Ok(())
    }
}
