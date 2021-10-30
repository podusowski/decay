use crate::physics::{Body, Space};
use crate::{algebra::Vector, units::Distance, units::Mass};
use chrono::prelude::*;

const SECONDS_IN_DAY: f64 = 24.0 * 60.0 * 60.0;

impl<UserData> Space<UserData> {
    /// Taken from JPL's HORIZONS for A.D. 2016-Oct-15 00:00:00.0000 TDB
    pub fn solar_system(user_data_factory: impl FnMut() -> UserData) -> Self {
        let mut user_data_factory = user_data_factory;
        let mut space = Self::default();
        space.time = Utc.ymd(2016, 10, 15).and_hms(0, 0, 0);

        space.bodies.push(Body {
            position: Vector::default(),
            velocity: Vector::default(),
            mass: Mass::from_kgs(1988500e24),
            name: "Sun",
            user_data: user_data_factory(),
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
            user_data: user_data_factory(),
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
            user_data: user_data_factory(),
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
            user_data: user_data_factory()
        });

        space.bodies.push(Body {
            position: Vector {
                x: Distance::from_aus(9.279584884887349E-01).as_meters(),
                y: Distance::from_aus(3.711104444050654E-01).as_meters(),
                z: Distance::from_aus(-1.037121134101948E-04).as_meters(),
            },
            velocity: Vector {
                x: Distance::from_aus(-6.761640228117041E-03).as_meters() / SECONDS_IN_DAY,
                y: Distance::from_aus(1.651990549601004E-02).as_meters() / SECONDS_IN_DAY,
                z: Distance::from_aus(-5.110814081371464E-05).as_meters() / SECONDS_IN_DAY,
            },
            mass: Mass::from_kgs(7.349e22),
            name: "Moon",
            user_data: user_data_factory()
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
            user_data: user_data_factory()
        });

        space.bodies.push(Body {
            position: Vector {
                x: Distance::from_aus(-5.432121324842138E+00).as_meters(),
                y: Distance::from_aus(-4.569844296390864E-01).as_meters(),
                z: Distance::from_aus(1.234480663921852E-01).as_meters(),
            },
            velocity: Vector {
                x: Distance::from_aus(5.420082489396598E-04).as_meters() / SECONDS_IN_DAY,
                y: Distance::from_aus(-7.169749225100362E-03).as_meters() / SECONDS_IN_DAY,
                z: Distance::from_aus(1.762648657670620E-05).as_meters() / SECONDS_IN_DAY,
            },
            mass: Mass::from_kgs(189818722e19),
            name: "Jupiter",
            user_data: user_data_factory()
        });

        space.bodies.push(Body {
            position: Vector {
                x: Distance::from_aus(-2.270616615877762e+00).as_meters(),
                y: Distance::from_aus(-9.778225136344251e+00).as_meters(),
                z: Distance::from_aus(2.603431483798866e-01).as_meters(),
            },
            velocity: Vector {
                x: Distance::from_aus(5.130339671553237E-03).as_meters() / SECONDS_IN_DAY,
                y: Distance::from_aus(-1.285344196202056E-03).as_meters() / SECONDS_IN_DAY,
                z: Distance::from_aus(-1.815947060563043E-04).as_meters() / SECONDS_IN_DAY,
            },
            mass: Mass::from_kgs(5.6834E26),
            name: "Saturn",
            user_data: user_data_factory()
        });

        space.bodies.push(Body {
            position: Vector {
                x: Distance::from_aus(2.825736958467999e+01).as_meters(),
                y: Distance::from_aus(-9.925704611354977e+00).as_meters(),
                z: Distance::from_aus(-4.468121470675587e-01).as_meters(),
            },
            velocity: Vector {
                x: Distance::from_aus(1.020926560027260e-03).as_meters() / SECONDS_IN_DAY,
                y: Distance::from_aus(2.974039863467131e-03).as_meters() / SECONDS_IN_DAY,
                z: Distance::from_aus(-8.451314801496931e-05).as_meters() / SECONDS_IN_DAY,
            },
            mass: Mass::from_kgs(1.024e26),
            name: "Neptune",
            user_data: user_data_factory()
        });

        space
    }
}
