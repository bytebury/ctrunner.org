use crate::{
    DbConnection,
    domain::{
        Town, User,
        race::{NewRace, NewRaceResult},
        town::{Run169TownsSocietyGoogleForm, Run169TownsSocietyGoogleFormAnswers, SubmitTown},
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

    pub async fn find_completed(&self, user_id: i64) -> Vec<Town> {
        self.town_repository.find_completed(user_id).await
    }

    pub async fn submit_completed_town(&self, user: User, form: SubmitTown) -> Result<(), String> {
        let town_id = form.town_id;
        let town = self.town_repository.find_by_id(town_id).await?;

        let new_race = NewRace::from(form.clone());
        let race = self.race_repository.get_or_create(new_race).await?;

        // Submit the form to Run169Towns Society.
        let answers = Run169TownsSocietyGoogleFormAnswers::new(&user, &town, &form);
        Run169TownsSocietyGoogleForm::submit_with_answers(answers).await?;

        let race_result = NewRaceResult::new(user.id, &race, form.notes);
        let _ = self.town_repository.mark_completed(user.id, town_id).await;
        let _ = self.race_repository.save_result(race_result).await;

        Ok(())
    }
}
