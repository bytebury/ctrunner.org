use sqlx::query;

use crate::{
    DbConnection,
    domain::race::{NewRace, RaceView, SubmitTownSearchParams},
    util::pagination::{Paginatable, PaginatedResponse, Pagination},
};

pub struct RaceRepository {
    db: DbConnection,
}

impl RaceRepository {
    pub fn new(db: &DbConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn submit_town_search(
        &self,
        params: &SubmitTownSearchParams,
    ) -> PaginatedResponse<RaceView> {
        let pattern = &format!("%{}%", params.race_name.to_lowercase());

        RaceView::paginate_filter(
            &self.db,
            &Pagination::default(),
            Some(r#"(LOWER(name) LIKE ? AND town_id = ?) ORDER BY start_date DESC"#),
            vec![pattern, &params.town_id.to_string()],
        )
        .await
        .unwrap()
    }

    pub async fn create_race(&self, race: NewRace) -> Result<(), String> {
        query(
            r#"
           INSERT INTO races (town_id, name, miles, start_date, street_address, race_url)
           VALUES (?, LOWER(?), ?, ?, ?, ?)
           "#,
        )
        .bind(race.town_id)
        .bind(race.name)
        .bind(race.miles.value())
        .bind(race.start_date)
        .bind(race.street_address)
        .bind(race.race_url)
        .execute(self.db.as_ref())
        .await
        .map_err(|_| "Something went wrong creating the race".to_string())?;

        Ok(())
    }
}
