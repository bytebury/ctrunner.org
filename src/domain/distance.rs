use serde::{Deserialize, Serialize};

pub struct Miles(f64);

impl Miles {
    pub fn new(distance: f64) -> Self {
        Miles(distance)
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn to_kilometers(&self) -> Kilometers {
        Kilometers((self.0 * 1.60934 * 10.0).round() / 10.0)
    }
}

pub struct Kilometers(f64);

impl Kilometers {
    pub fn new(distance: f64) -> Self {
        Kilometers(distance)
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn to_miles(&self) -> Miles {
        Miles((self.0 / 1.60934 * 10.0).round() / 10.0)
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq)]
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
