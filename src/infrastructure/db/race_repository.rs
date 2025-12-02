use crate::{
    DbConnection,
    domain::race::{RaceView, SubmitTownSearchParams},
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
}
