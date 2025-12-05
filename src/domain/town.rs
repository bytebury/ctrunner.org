use crate::domain::distance::DistanceUnit;
use crate::domain::distance::Kilometers;
use crate::domain::distance::Miles;
use crate::domain::user::UserView;
use crate::util::parse_no_seconds;
use chrono::NaiveDateTime;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Town {
    pub id: i64,
    pub name: String,
    pub county_id: i64,
    pub county: String,
}

impl Town {
    pub fn is_valid(town_id: i64) -> bool {
        (1..169).contains(&town_id)
    }

    pub fn is_not_valid(town_id: i64) -> bool {
        !Self::is_valid(town_id)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CompletedTown {
    pub id: i64,
    pub user_id: i64,
    pub town_id: i64,
    pub name: String,
    pub county: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SubmitTown {
    pub town_id: i64,
    pub race_name: String,
    pub race_id: i64,
    pub distance_val: f64,
    pub distance_unit: DistanceUnit,
    #[serde(deserialize_with = "parse_no_seconds")]
    pub start_at: NaiveDateTime,
    pub notes: Option<String>,
}

pub struct Run169TownsSocietyGoogleFormAnswers {
    pub town_name: String,
    pub race_name: String,
    pub distance_val: Miles,
    pub race_date: NaiveDate,
    pub notes: String,
    pub member_id: String,
    pub first_name: String,
    pub last_name: String,
}

impl Run169TownsSocietyGoogleFormAnswers {
    pub fn new(user: &UserView, town: &Town, form: &SubmitTown) -> Self {
        let distance_val = match form.distance_unit {
            DistanceUnit::Miles => Miles::new(form.distance_val),
            DistanceUnit::Kilometers => Kilometers::new(form.distance_val).to_miles(),
        };

        Self {
            member_id: user.runner_id.unwrap().to_string(),
            distance_val,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            town_name: town.name.clone(),
            race_name: form.race_name.clone(),
            race_date: form.start_at.date(),
            notes: form.notes.clone().unwrap_or_default(),
        }
    }
}

pub struct Run169TownsSocietyGoogleForm {
    form_id: String,
    member_id: String,
    action: String,
    first_name: String,
    last_name: String,
    town_of_race: String,
    date_of_race_year: String,
    date_of_race_month: String,
    date_of_race_day: String,
    distance: String,
    name_of_race: String,
    is_169th_town: String,
    notify_others: String,
    comment: String,
    answers: HashMap<String, String>,
}

impl Run169TownsSocietyGoogleForm {
    fn from_env() -> Self {
        Self {
            form_id: std::env::var("SUBMIT_FORM_ID").expect("SUBMIT_FORM_ID is missing"),
            member_id: std::env::var("SUBMIT_MEMBER_ID").expect("SUBMIT_MEMBER_ID is missing"),
            action: std::env::var("SUBMIT_ACTION").expect("SUBMIT_ACTION is missing"),
            first_name: std::env::var("SUBMIT_FIRST_NAME").expect("SUBMIT_FIRST_NAME is missing"),
            last_name: std::env::var("SUBMIT_LAST_NAME").expect("SUBMIT_LAST_NAME is missing"),
            town_of_race: std::env::var("SUBMIT_TOWN_OF_RACE")
                .expect("SUBMIT_TOWN_OF_RACE is missing"),
            date_of_race_year: std::env::var("SUBMIT_DATE_OF_RACE_YEAR")
                .expect("SUBMIT_DATE_OF_RACE_YEAR is missing"),
            date_of_race_month: std::env::var("SUBMIT_DATE_OF_RACE_MONTH")
                .expect("SUBMIT_DATE_OF_RACE_MONTH is missing"),
            date_of_race_day: std::env::var("SUBMIT_DATE_OF_RACE_DAY")
                .expect("SUBMIT_DATE_OF_RACE_DAY is missing"),
            distance: std::env::var("SUBMIT_DISTANCE").expect("SUBMIT_DISTANCE is missing"),
            name_of_race: std::env::var("SUBMIT_NAME_OF_RACE")
                .expect("SUBMIT_NAME_OF_RACE is missing"),
            is_169th_town: std::env::var("SUBMIT_IS_169TH_TOWN")
                .expect("SUBMIT_IS_169TH_TOWN is missing"),
            notify_others: std::env::var("SUBMIT_NOTIFY_OTHERS")
                .expect("SUBMIT_NOTIFY_OTHERS is missing"),
            comment: std::env::var("SUBMIT_COMMENT").expect("SUBMIT_COMMENT is missing"),
            answers: HashMap::new(),
        }
    }

    pub async fn submit_with_answers(
        answers: Run169TownsSocietyGoogleFormAnswers,
    ) -> Result<(), String> {
        Self::from_env().add_answers(answers).submit().await
    }

    fn add_answers(
        mut self,
        form: Run169TownsSocietyGoogleFormAnswers,
    ) -> CompletedRun169TownsSocietyGoogleForm {
        self.answers
            .insert(format!("entry.{}", self.member_id), form.member_id);
        self.answers
            .insert(format!("entry.{}", self.action), "New".to_string());
        self.answers
            .insert(format!("entry.{}", self.first_name), form.first_name);
        self.answers
            .insert(format!("entry.{}", self.last_name), form.last_name);
        self.answers
            .insert(format!("entry.{}", self.town_of_race), form.town_name);
        self.answers.insert(
            format!("entry.{}", self.date_of_race_year),
            form.race_date.year().to_string(),
        );
        self.answers.insert(
            format!("entry.{}", self.date_of_race_month),
            format!("{:02}", form.race_date.month()),
        );
        self.answers.insert(
            format!("entry.{}", self.date_of_race_day),
            format!("{:02}", form.race_date.day()),
        );
        self.answers.insert(
            format!("entry.{}", self.distance),
            form.distance_val.value().to_string(),
        );
        self.answers
            .insert(format!("entry.{}", self.name_of_race), form.race_name);
        self.answers
            .insert(format!("entry.{}", self.is_169th_town), "No".into());
        self.answers
            .insert(format!("entry.{}", self.notify_others), "No".into());
        self.answers
            .insert(format!("entry.{}", self.comment), form.notes);

        CompletedRun169TownsSocietyGoogleForm(self)
    }
}

pub struct CompletedRun169TownsSocietyGoogleForm(Run169TownsSocietyGoogleForm);

impl CompletedRun169TownsSocietyGoogleForm {
    pub async fn submit(&self) -> Result<(), String> {
        let base_url = "https://docs.google.com/forms/d/e";
        let url = format!("{}/{}/formResponse", base_url, self.0.form_id);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .form(&self.0.answers)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        response.error_for_status().map_err(|e| e.to_string())?;
        Ok(())
    }
}
