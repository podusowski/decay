use bevy::prelude::Component;
use serde::Deserialize;
pub use uom::si::{f64::Mass, mass::kilogram};

use crate::algebra::Vector;

// Object having a mass and position in space.
pub trait MassObject {
    fn mass(&self) -> Mass;
    fn position(&self) -> Vector;

    fn newtonian_gravity(&self, other: &impl MassObject) -> Vector {
        // Pauli exclusion principle FTW!
        if self.position() == other.position() {
            Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            let offset = self.position() - other.position();
            -G * ((self.mass().get::<kilogram>() * other.mass().get::<kilogram>())
                / offset.length().powi(2))
                * offset.normalized()
        }
    }
}

/// While `uom` provides `serde` support, it only reads and writes the
/// underlying unit-less value. For instance, if `uom::si::f64::Mass` is used,
/// it assumes that value it reads holds kilograms (as kgs are base type for
/// `uom::si::f64::Mass`.
///
/// See https://github.com/iliekturtles/uom/issues/110
mod mass_serializer {
    use serde::Deserialize;
    use serde::Deserializer;
    use uom::si::f64::Mass;
    use uom::si::mass::gram;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Mass, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mass = f64::deserialize(deserializer)?;
        // Encode unit in YAML.
        Ok(Mass::new::<gram>(mass))
    }
}

#[derive(Debug, Deserialize, Component)]
pub struct Body {
    pub name: String,
    #[serde(with = "mass_serializer")]
    pub mass: Mass,
    pub position: Vector,
    pub velocity: Vector,
}

impl MassObject for Body {
    fn mass(&self) -> Mass {
        self.mass
    }

    fn position(&self) -> Vector {
        self.position
    }
}

pub const G: f64 = 6.67408e-11f64;
