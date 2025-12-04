use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::Deserialize;
use serde_json::Value;

use crate::domain::race::UpcomingRaceFromRun169Society;

#[derive(Deserialize, Debug)]
pub struct GvizResponse {
    pub table: GvizTable,
}

#[derive(Deserialize, Debug)]
pub struct GvizTable {
    pub rows: Vec<GvizRow>,
}

#[derive(Deserialize, Debug)]
pub struct GvizRow {
    pub c: Option<Vec<Option<GvizCell>>>,
}

#[derive(Deserialize, Debug)]
pub struct GvizCell {
    pub v: Option<Value>,
}

pub struct GoogleSheet {
    pub table: GvizTable,
}

impl GoogleSheet {
    pub async fn upcoming_races() -> Result<Vec<UpcomingRaceFromRun169Society>, String> {
        let url = "https://docs.google.com/spreadsheets/d/1QLjGbAQzxOHqdoE4tKi2V5kqOnYQmiL6qbrPZPOoc28/gviz/tq?gid=0&headers=1&tqx=reqId%3A0";
        Ok(GoogleSheet::load_from(url).await?.into())
    }

    async fn load_from(url: &str) -> Result<Self, String> {
        let client = reqwest::Client::new();
        let raw = client
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;

        let start = raw.find('{').ok_or("no JSON found")?;
        let end = raw.rfind('}').ok_or("no JSON end found")?;
        let json = &raw[start..=end];

        Ok(Self {
            table: serde_json::from_str::<GvizResponse>(json)
                .map_err(|e| e.to_string())?
                .table,
        })
    }

    pub fn parse_date_cells(date: String, time: String) -> Result<NaiveDateTime, String> {
        let (y1, m1, d1, _, _, _) = Self::parse_date(&date)?;
        let (_, _, _, h, mi, s) = Self::parse_date(&time)?;

        let date = NaiveDate::from_ymd_opt(y1, m1, d1).ok_or_else(|| "Invalid date".to_string())?;
        let time = NaiveTime::from_hms_opt(h, mi, s).ok_or_else(|| "Invalid time".to_string())?;

        Ok(NaiveDateTime::new(date, time))
    }

    fn parse_date(input: &str) -> Result<(i32, u32, u32, u32, u32, u32), String> {
        let s = input.trim().trim_matches('"');

        let inner = s
            .strip_prefix("Date(")
            .and_then(|v| v.strip_suffix(")"))
            .ok_or_else(|| format!("Invalid date format: {}", input))?;

        let parts: Vec<_> = inner.split(',').collect();

        let nums: Vec<i32> = parts
            .into_iter()
            .map(|p| p.trim().parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| format!("Invalid number in: {}", input))?;

        Ok(match nums.len() {
            3 => (nums[0], nums[1] as u32, nums[2] as u32, 0, 0, 0),
            6 => (
                nums[0],
                nums[1] as u32,
                nums[2] as u32,
                nums[3] as u32,
                nums[4] as u32,
                nums[5] as u32,
            ),
            _ => return Err(format!("Unexpected date format: {}", input)),
        })
    }
}
