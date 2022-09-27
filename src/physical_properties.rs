//! Couple of hardcoded physical properties of largest bodies in the
//! Solar System. They are gathered from various places as JPL Horizons doesn't
//! provide any systematic way of obtaining them.
//!
//! Sources:
//! - <https://ssd.jpl.nasa.gov/planets/phys_par.html>
//! - <https://nssdc.gsfc.nasa.gov/planetary/factsheet/sunfact.html>

pub fn mass_of(name: &str) -> Option<f64> {
    OBJECTS
        .iter()
        .find(|object| object.name == name)
        .map(|object| object.mass)
}

struct Object {
    name: &'static str,
    mass: f64,
}

const OBJECTS: &[Object] = &[
    Object {
        name: "Mercury",
        mass: 0.330103e24,
    },
    Object {
        name: "Venus",
        mass: 4.86731e24,
    },
    Object {
        name: "Earth",
        mass: 5.97217e24,
    },
    Object {
        name: "Mars",
        mass: 0.641691e24,
    },
    Object {
        name: "Jupiter",
        mass: 1898.125e24,
    },
    Object {
        name: "Saturn",
        mass: 568.317e24,
    },
    Object {
        name: "Uranus",
        mass: 86.8099e24,
    },
    Object {
        name: "Neptune",
        mass: 102.4092e24,
    },
];
