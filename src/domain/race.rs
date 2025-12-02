use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    domain::{
        distance::{DistanceUnit, Kilometers, Miles},
        town::SubmitTown,
    },
    util::pagination::{Paginatable, Pagination},
};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Race {
    pub id: i64,
    pub town_id: i64,
    pub name: String,
    pub miles: f64,
    pub start_date: chrono::NaiveDate,
    pub street_address: Option<String>,
    pub race_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct RaceView {
    pub id: i64,
    pub town_id: i64,
    pub name: String,
    pub town: String,
    pub county: String,
    pub miles: f64,
    pub start_date: chrono::NaiveDate,
    pub street_address: Option<String>,
    pub race_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Paginatable for RaceView {
    fn table_name() -> &'static str {
        "races_view"
    }
}

#[derive(Deserialize)]
pub struct SubmitTownSearchParams {
    pub race_name: String,
    pub town_id: i64,
}

#[derive(Deserialize)]
pub struct RaceSearchParams {
    pub race_name: Option<String>,
    pub town_id: Option<i64>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl From<RaceSearchParams> for Pagination {
    fn from(params: RaceSearchParams) -> Self {
        Self {
            page: params.page,
            page_size: params.page_size,
        }
    }
}

pub struct NewRace {
    pub name: String,
    pub town_id: i64,
    pub miles: Miles,
    pub start_date: chrono::NaiveDate,
    pub street_address: Option<String>,
    pub race_url: Option<String>,
}

pub struct NewRaceResult {
    pub user_id: i64,
    pub race_id: i64,
    pub notes: Option<String>,
}

impl NewRaceResult {
    pub fn new(user_id: i64, race: &RaceView, notes: Option<String>) -> Self {
        Self {
            user_id,
            race_id: race.id,
            notes,
        }
    }
}

impl From<SubmitTown> for NewRace {
    fn from(form: SubmitTown) -> Self {
        let miles = match form.distance_unit {
            DistanceUnit::Miles => Miles::new(form.distance_val),
            DistanceUnit::Kilometers => Kilometers::new(form.distance_val).to_miles(),
        };
        Self {
            name: form.race_name,
            town_id: form.town_id,
            miles,
            start_date: form.race_date,
            street_address: None,
            race_url: None,
        }
    }
}
