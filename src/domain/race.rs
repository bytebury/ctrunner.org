use std::str::FromStr;

use crate::{
    domain::google_sheet::GoogleSheet,
    util::{StringExt, parse_no_seconds},
};
use chrono::{NaiveDateTime, Utc};
use chrono_tz::America::New_York;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    domain::{
        distance::{DistanceUnit, Miles},
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
    pub race_url: Option<String>,
    pub start_at: chrono::NaiveDateTime,
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
    pub is_elusive: bool,
    pub miles: f64,
    pub race_url: Option<String>,
    pub start_at: chrono::NaiveDateTime,
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

#[derive(Deserialize)]
pub struct NewRaceForm {
    pub name: String,
    pub town_id: i64,
    pub distance_val: f64,
    pub distance_unit: DistanceUnit,
    #[serde(deserialize_with = "parse_no_seconds")]
    pub start_at: NaiveDateTime,
    pub race_url: String,
}

pub struct NewRace {
    pub name: String,
    pub town_id: i64,
    pub miles: Miles,
    pub start_at: chrono::NaiveDateTime,
    pub race_url: Option<String>,
}

#[derive(Debug)]
pub struct UpcomingRaceFromRun169Society {
    pub name: String,
    pub town_name: String,
    pub miles: Miles,
    pub start_at: chrono::NaiveDateTime,
    pub race_url: String,
}

impl From<GoogleSheet> for Vec<UpcomingRaceFromRun169Society> {
    fn from(value: GoogleSheet) -> Self {
        let mut races = Vec::new();

        for row in value.table.rows {
            let columns: Vec<String> = row
                .c
                .unwrap_or_default()
                .into_iter()
                .map(|cell| {
                    cell.and_then(|cell| cell.v)
                        .map(|v| v.to_string().trim_matches('"').to_string())
                        .unwrap_or_default()
                })
                .collect();

            let start_date = columns[0].clone();
            let start_time = columns[1].clone();
            let is_confirmed = columns[8].clone().is_whitespace_or_empty();
            let start_at = match GoogleSheet::parse_date_cells(start_date, start_time) {
                Ok(date) => date,
                Err(_) => continue,
            };

            if start_at < Utc::now().with_timezone(&New_York).naive_local() {
                continue;
            }

            if !is_confirmed {
                continue;
            }

            let miles: Vec<&str> = columns[6].split(',').map(|s| s.trim()).collect();
            for miles in &miles {
                let name = columns[4].clone();
                let town_name = columns[2].clone();
                let race_url = columns[7].clone();
                let miles = match Miles::from_str(miles) {
                    Ok(miles) => miles,
                    Err(_) => continue,
                };

                races.push(UpcomingRaceFromRun169Society {
                    name,
                    town_name,
                    miles,
                    start_at,
                    race_url,
                });
            }
        }

        races
    }
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
        Self {
            name: form.race_name,
            town_id: form.town_id,
            miles: Miles::parse(form.distance_val, form.distance_unit),
            start_at: form.start_at,
            race_url: None,
        }
    }
}

impl From<NewRaceForm> for NewRace {
    fn from(form: NewRaceForm) -> Self {
        Self {
            name: form.name,
            town_id: form.town_id,
            miles: Miles::parse(form.distance_val, form.distance_unit),
            start_at: form.start_at,
            race_url: Some(form.race_url),
        }
    }
}
