use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use uom::si::{f64::Mass, mass::kilogram};

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

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct Body {
    pub name: String,
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
