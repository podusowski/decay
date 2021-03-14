// Taken from JPL's HORIZONS system: https://ssd.jpl.nasa.gov/

use crate::algebra::Vector;
use crate::physics::{Body, Space};

struct Au(f64);

impl Au {
    fn as_meters(&self) -> f64 {
        597870700.0 * self.0
    }
}

impl Space {
    pub fn solar_system() -> Space {
        let mut space = Space::default();

        // Sun
        space.bodies.push(Body {
            position: Vector::default(),
            velocity: Vector::default(),
            mass: 1.989e30,
            name: "Sun"
        });

        // Mercury
        // 2457676.500000000 = A.D. 2016-Oct-15 00:00:00.0000 TDB
        //  X =-3.610946582889994E-01 Y = 7.655753687572452E-02 Z = 3.938313941762204E-02
        //  VX=-1.166921930880622E-02 VY=-2.631562924335937E-02 VZ=-1.079745298798429E-03
        space.bodies.push(Body {
            position: Vector {
                x: Au(-3.610946582889994E-01).as_meters(),
                y: Au(7.655753687572452E-02).as_meters(),
                z: Au(3.938313941762204E-02).as_meters(),
            },
            velocity: Vector {
                x: Au(-1.166921930880622E-02).as_meters(),
                y: Au(-2.631562924335937E-02).as_meters(),
                z: Au(-1.079745298798429E-03).as_meters(),
            },
            mass: 3.302e23,
            name: "Mercury"
        });

        space
    }
}
