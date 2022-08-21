use serde::{Deserialize, Serialize};

pub struct Distance(f64);

impl Distance {
    const METERS_IN_AU: f64 = 149597870700.0;

    pub fn from_meters(meters: f64) -> Self {
        Distance(meters)
    }

    pub fn from_aus(aus: f64) -> Self {
        Distance(aus * Self::METERS_IN_AU)
    }

    pub fn as_meters(&self) -> f64 {
        self.0
    }

    pub fn as_au(&self) -> f64 {
        self.0 / Self::METERS_IN_AU
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Mass(f64);

impl Mass {
    pub fn from_kgs(kgs: f64) -> Mass {
        Mass(kgs * 1000.0)
    }

    pub fn as_kgs(&self) -> f64 {
        self.0 / 1000.0
    }

    pub fn as_gs(&self) -> f64 {
        self.0
    }
}
