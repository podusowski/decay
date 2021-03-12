// Taken from JPL's HORIZONS system: https://ssd.jpl.nasa.gov/
//
// Mercury
// 2457676.500000000 = A.D. 2016-Oct-15 00:00:00.0000 TDB
//  X =-3.610946582889994E-01 Y = 7.655753687572452E-02 Z = 3.938313941762204E-02
//  VX=-1.166921930880622E-02 VY=-2.631562924335937E-02 VZ=-1.079745298798429E-03

use crate::algebra::Vector;
use crate::physics::{Body, Space};

impl Space {
    pub fn solar_system() -> Space {
        let mut space = Space::default();

        // Sun
        space.bodies.push(Body {
            position: Vector { x: 0.0, y: 0.0 },
            velocity: Vector { x: 0.0, y: 0.0 },
            mass: 1.989e30,
        });

        space
    }
}
