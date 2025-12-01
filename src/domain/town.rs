use crate::domain::distance::Miles;
use crate::domain::{User, distance::DistanceUnit};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::{collections::HashMap, fmt};

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

impl fmt::Display for Town {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SubmitTown {
    pub town_id: i64,
    pub race_id: i64,
    pub distance_val: f64,
    pub distance_unit: DistanceUnit,
    pub race_date: NaiveDate,
    pub notes: String,
}

pub struct SubmitTownForGoogle {
    pub town_name: String,
    pub race_name: String,
    pub distance_val: Miles,
    pub race_date: NaiveDate,
    pub notes: String,
}

pub struct SubmitTownGoogleForm {
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

impl SubmitTownGoogleForm {
    pub fn new() -> Self {
        Self {
            form_id: "1FAIpQLScHViJvQL0G_ZPuCZOIFNsBPthZwDSzbkgiFFeL93wp831diA".to_string(),
            member_id: "1858653824".to_string(),
            action: "517872474".to_string(),
            first_name: "1421839249".to_string(),
            last_name: "390953767".to_string(),
            town_of_race: "1178659240".to_string(),
            date_of_race_year: "1640631443_year".to_string(),
            date_of_race_month: "1640631443_month".to_string(),
            date_of_race_day: "1640631443_day".to_string(),
            distance: "1543094814".to_string(),
            name_of_race: "1606581847".to_string(),
            is_169th_town: "809023255".to_string(),
            notify_others: "1292315262".to_string(),
            comment: "1729945787".to_string(),
            answers: HashMap::new(),
        }
    }

    pub fn add_answers(
        mut self,
        user: User,
        form: SubmitTownForGoogle,
    ) -> CompletedSubmitTownGoogleForm {
        self.answers.insert(
            format!("entry.{}", self.member_id),
            user.runner_id.unwrap().to_string(),
        );
        self.answers
            .insert(format!("entry.{}", self.action), "New".to_string());
        self.answers
            .insert(format!("entry.{}", self.first_name), user.first_name);
        self.answers
            .insert(format!("entry.{}", self.last_name), user.last_name);
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

        CompletedSubmitTownGoogleForm(self)
    }
}

pub struct CompletedSubmitTownGoogleForm(SubmitTownGoogleForm);

impl CompletedSubmitTownGoogleForm {
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
