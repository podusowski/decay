//! Couple of hardcoded physical properties of the largest bodies in the
//! Solar System. They are gathered from various places as JPL Horizons doesn't
//! provide any systematic way of obtaining them.
//!
//! Sources:
//! - <https://ssd.jpl.nasa.gov/planets/phys_par.html>
//! - <https://nssdc.gsfc.nasa.gov/planetary/factsheet/sunfact.html>

use bevy::prelude::Color;

pub fn mass_of(name: &str) -> Option<f64> {
    OBJECTS
        .iter()
        .find(|object| object.name == name)
        .map(|object| object.mass)
}

pub fn about(name: &str) -> Option<&'static Object> {
    OBJECTS.iter().find(|object| object.name == name)
}

const DEFAULT_COLOR: Color = Color::rgb(0.28, 0.35, 0.4);

pub struct Object {
    pub name: &'static str,
    pub mass: f64,
    pub color: Color,
}

const OBJECTS: &[Object] = &[
    Object {
        name: "Sun",
        mass: 1988500e24,
        color: DEFAULT_COLOR,
    },
    Object {
        name: "Mercury",
        mass: 0.330103e24,
        color: DEFAULT_COLOR,
    },
    Object {
        name: "Venus",
        mass: 4.86731e24,
        color: DEFAULT_COLOR,
    },
    Object {
        name: "Earth",
        mass: 5.97217e24,
        color: DEFAULT_COLOR,
    },
    Object {
        name: "Mars",
        mass: 0.641691e24,
        color: DEFAULT_COLOR,
    },
    Object {
        name: "Jupiter",
        mass: 1898.125e24,
        color: DEFAULT_COLOR,
    },
    Object {
        name: "Saturn",
        mass: 568.317e24,
        color: DEFAULT_COLOR,
    },
    Object {
        name: "Uranus",
        mass: 86.8099e24,
        color: DEFAULT_COLOR,
    },
    Object {
        name: "Neptune",
        mass: 102.4092e24,
        color: DEFAULT_COLOR,
    },
];
