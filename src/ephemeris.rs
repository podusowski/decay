use crate::physics::{Body, Space};
use crate::{algebra::Vector, units::Distance, units::Mass};

const SECONDS_IN_DAY: f64 = 24.0 * 60.0 * 60.0;

impl Space {
    /// Taken from JPL's HORIZONS for A.D. 2016-Oct-15 00:00:00.0000 TDB
    pub fn solar_system() -> Space {
        let mut space = Space::default();

        space.bodies.push(Body {
            position: Vector::default(),
            velocity: Vector::default(),
            mass: Mass::from_kgs(1988500e24),
            name: "Sun",
        });

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
            mass: Mass::from_kgs(48.685e23),
            name: "Venus",
        });

        space.bodies.push(Body {
            position: Vector {
                x: Distance::from_aus(9.255531850624332E-01).as_meters(),
                y: Distance::from_aus(3.708906496672169E-01).as_meters(),
                z: Distance::from_aus(-1.740719250795340E-05).as_meters(),
            },
            velocity: Vector {
                x: Distance::from_aus(-6.678671400247983E-03).as_meters() / SECONDS_IN_DAY,
                y: Distance::from_aus(1.589848110988471E-02).as_meters() / SECONDS_IN_DAY,
                z: Distance::from_aus(5.510397457798760E-08).as_meters() / SECONDS_IN_DAY,
            },
            mass: Mass::from_kgs(5.97219e24),
            name: "Earth",
        });

        space.bodies.push(Body {
            position: Vector {
                x: Distance::from_aus(1.158495532744392E+00).as_meters(),
                y: Distance::from_aus(-7.536556465639087E-01).as_meters(),
                z: Distance::from_aus(-4.422526991966770E-02).as_meters(),
            },
            velocity: Vector {
                x: Distance::from_aus(8.162919959786599E-03).as_meters() / SECONDS_IN_DAY,
                y: Distance::from_aus(1.292885802502735E-02).as_meters() / SECONDS_IN_DAY,
                z: Distance::from_aus(7.059671810539333E-05).as_meters() / SECONDS_IN_DAY,
            },
            mass: Mass::from_kgs(6.4171e23),
            name: "Mars",
        });

        space
    }
}
