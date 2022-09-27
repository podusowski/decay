//! https://ssd.jpl.nasa.gov/planets/phys_par.html

struct Object {
    name: &'static str,
    mass: f32,
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
