use serde::Serialize;

use crate::algebra::Vector;
use crate::units::Mass;

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
            -G * ((self.mass().as_kgs() * other.mass().as_kgs()) / offset.length().powi(2))
                * offset.normalized()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Body {
    pub name: &'static str,
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

/// Ships are objects too, but unlike regular bodies they can move by their own.
#[derive(Debug)]
pub struct Ship {
    pub position: Vector,
    pub velocity: Vector,
    pub thrust: Vector,
    pub name: &'static str,
}

impl MassObject for Ship {
    fn mass(&self) -> Mass {
        Mass::from_kgs(10000.0)
    }

    fn position(&self) -> Vector {
        self.position
    }
}

pub const G: f64 = 6.67408e-11f64;

#[derive(Debug)]
pub struct Space /* perhaps time some day... */ {
    pub time: chrono::DateTime<chrono::Utc>,
    pub bodies: Vec<Body>,
    pub ships: Vec<Ship>,
}

impl Default for Space {
    fn default() -> Self {
        Space {
            time: chrono::Utc::now(),
            bodies: Default::default(),
            ships: Default::default(),
        }
    }
}
