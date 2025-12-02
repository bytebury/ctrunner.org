use sqlx::{query, query_as};

use crate::{
    DbConnection,
    domain::race::{NewRace, NewRaceResult, RaceSearchParams, RaceView, SubmitTownSearchParams},
    util::pagination::{Paginatable, PaginatedResponse, Pagination},
};

pub struct RaceRepository {
    db: DbConnection,
}

impl RaceRepository {
    pub fn new(db: &DbConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_id(&self, race_id: i64) -> Result<RaceView, String> {
        let race: RaceView = query_as("SELECT * FROM races_view WHERE id = ?")
            .bind(race_id)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(|e| format!("Failed to find race by ID: {}", e))?;

        Ok(race)
    }

    pub async fn get_or_create(&self, race: NewRace) -> Result<RaceView, String> {
        let race_id: i64 = sqlx::query_scalar(
            r#"
       		INSERT INTO races (town_id, name, miles, start_at, street_address, race_url)
            VALUES (?, LOWER(?), ?, ?, ?, ?)
            ON CONFLICT(town_id, name, miles, start_at)
            DO UPDATE SET name = name
            RETURNING id
       		"#,
        )
        .bind(race.town_id)
        .bind(race.name)
        .bind(race.miles.value())
        .bind(race.start_at)
        .bind(race.street_address)
        .bind(race.race_url)
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|_| "Something went wrong creating the race")?;

        self.find_by_id(race_id).await
    }

    pub async fn save_result(&self, result: NewRaceResult) -> Result<(), String> {
        query(
            r#"
            INSERT INTO race_results (user_id, race_id, notes)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(result.user_id)
        .bind(result.race_id)
        .bind(result.notes)
        .execute(self.db.as_ref())
        .await
        .map_err(|_| "Something went wrong submitting the result".to_string())?;

        Ok(())
    }

    pub async fn search_for_upcoming(
        &self,
        params: RaceSearchParams,
    ) -> PaginatedResponse<RaceView> {
        RaceView::paginate_filter(
            &self.db,
            &Pagination::from(params),
            Some(r#"start_at >= DateTime('now') ORDER BY start_at ASC"#),
            vec![],
        )
        .await
        .unwrap()
    }

    pub async fn submit_town_search(
        &self,
        params: &SubmitTownSearchParams,
    ) -> PaginatedResponse<RaceView> {
        let pattern = &format!("%{}%", params.race_name.to_lowercase());

        RaceView::paginate_filter(
            &self.db,
            &Pagination::default(),
            Some(
                r#"
            	LOWER(name) LIKE ? AND town_id = ? AND
                start_at >= DateTime('now', '-6 months') AND
                start_at <= DateTime('now')
                ORDER BY start_at DESC
                "#,
            ),
            vec![pattern, &params.town_id.to_string()],
        )
        .await
        .unwrap()
    }
}
