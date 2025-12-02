use crate::{
    DbConnection,
    domain::{
        Town, User,
        distance::{DistanceUnit, Kilometers, Miles},
        town::{SubmitTown, SubmitTownForGoogle, SubmitTownGoogleForm},
    },
    infrastructure::db::TownRepository,
};

pub struct TownService {
    town_repository: TownRepository,
}

impl TownService {
    pub fn new(db: &DbConnection) -> Self {
        Self {
            town_repository: TownRepository::new(db),
        }
    }

    pub async fn find_all(&self) -> Vec<Town> {
        self.town_repository.find_all().await
    }

    pub async fn submit_completed_town(&self, user: User, form: SubmitTown) -> Result<(), String> {
        let distance_val = match form.distance_unit {
            DistanceUnit::Miles => Miles::new(form.distance_val),
            DistanceUnit::Kilometers => Kilometers::new(form.distance_val).to_miles(),
        };
        let google_form = SubmitTownForGoogle {
            distance_val,
            town_name: "Andover".to_string(),
            race_name: "Andover Run".to_string(),
            race_date: form.race_date,
            notes: form.notes,
        };

        SubmitTownGoogleForm::new()
            .add_answers(user, google_form)
            .submit()
            .await

        // TODO: Once this is submitted successfully, then we can update our tables
    }
}
