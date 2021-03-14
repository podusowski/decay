/// Taken from JPL's HORIZONS system: https://ssd.jpl.nasa.gov/
/// A.D. 2016-Oct-15 00:00:00.0000 TDB

use crate::{algebra::Vector, physics::Mass};
use crate::physics::{Body, Distance, Space};

const SECONDS_IN_DAY: f64 = 24.0 * 60.0 * 60.0;

impl Space {
    pub fn solar_system() -> Space {
        let mut space = Space::default();

        // Sun
        space.bodies.push(Body {
            position: Vector::default(),
            velocity: Vector::default(),
            mass: Mass::from_kgs(1.989e30),
            name: "Sun",
        });

        // Mercury
        // 2457676.500000000 = A.D. 2016-Oct-15 00:00:00.0000 TDB
        //  X =-3.610946582889994E-01 Y = 7.655753687572452E-02 Z = 3.938313941762204E-02
        //  VX=-1.166921930880622E-02 VY=-2.631562924335937E-02 VZ=-1.079745298798429E-03
        space.bodies.push(Body {
            position: Vector {
                x: Distance::from_aus(-3.610946582889994E-01).as_meters(),
                y: Distance::from_aus(7.655753687572452E-02).as_meters(),
                z: Distance::from_aus(3.938313941762204E-02).as_meters(),
            },
            velocity: Vector {
                x: Distance::from_aus(-1.166921930880622E-02).as_meters() / SECONDS_IN_DAY,
                y: Distance::from_aus(-2.631562924335937E-02).as_meters() / SECONDS_IN_DAY,
                z: Distance::from_aus(-1.079745298798429E-03).as_meters() / SECONDS_IN_DAY,
            },
            mass: Mass::from_kgs(3.302e23),
            name: "Mercury",
        });

        space.bodies.push(Body {
            position: Vector {
                x: Distance::from_aus(1.973338103014433E-01).as_meters(),
                y: Distance::from_aus(-7.001287841606206E-01).as_meters(),
                z: Distance::from_aus(-2.098736267890693E-02).as_meters(),
            },
            velocity: Vector {
                x: Distance::from_aus(1.933209186041313E-02).as_meters() / SECONDS_IN_DAY,
                y: Distance::from_aus(5.418163683984627E-03).as_meters() / SECONDS_IN_DAY,
                z: Distance::from_aus(-1.041291312991296E-03).as_meters() / SECONDS_IN_DAY,
            },
            mass: Mass::from_kgs(4.867e24),
            name: "Venus",
        });

        // X = 1.973338103014433E-01 Y =-7.001287841606206E-01 Z =-2.098736267890693E-02
        //VX= 1.933209186041313E-02 VY= 5.418163683984627E-03 VZ=-1.041291312991296E-03

        space
    }
}
