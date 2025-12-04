use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Miles(f64);

impl Miles {
    pub fn new(distance: f64) -> Self {
        Miles(distance)
    }

    pub fn parse(distance: f64, unit: DistanceUnit) -> Self {
        match unit {
            DistanceUnit::Miles => Miles::new(distance),
            DistanceUnit::Kilometers => Kilometers::new(distance).to_miles(),
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn to_kilometers(&self) -> Kilometers {
        Kilometers((self.0 * 1.60934 * 10.0).round() / 10.0)
    }
}

impl FromStr for Miles {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().trim_matches('"').to_string();
        let s = s.to_lowercase();

        if let Some(stripped) = s.strip_suffix('k') {
            match stripped.parse::<f64>() {
                Ok(distance) => return Ok(Kilometers(distance).to_miles()),
                Err(_) => return Err("Invalid distance".to_string()),
            };
        }

        if let Some(stripped) = s.strip_suffix('m') {
            match stripped.parse::<f64>() {
                Ok(distance) => return Ok(Miles(distance)),
                Err(_) => return Err("Invalid distance".to_string()),
            };
        }

        Err("Invalid distance".to_string())
    }
}

#[derive(Debug)]
pub struct Kilometers(f64);

impl Kilometers {
    pub fn new(distance: f64) -> Self {
        Kilometers(distance)
    }

    pub fn parse(distance: f64, unit: DistanceUnit) -> Self {
        match unit {
            DistanceUnit::Miles => Miles::new(distance).to_kilometers(),
            DistanceUnit::Kilometers => Kilometers::new(distance),
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn to_miles(&self) -> Miles {
        Miles((self.0 / 1.60934 * 10.0).round() / 10.0)
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DistanceUnit {
    #[default]
    Miles,
    Kilometers,
}

impl std::fmt::Display for DistanceUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistanceUnit::Miles => write!(f, "miles"),
            DistanceUnit::Kilometers => write!(f, "kilometers"),
        }
    }
}
