use crate::physics::Body;
use crate::{algebra::Vector, units::Distance, units::Mass};
use chrono::prelude::*;

const SECONDS_IN_DAY: f64 = 24.0 * 60.0 * 60.0;

/// Taken from JPL's HORIZONS for A.D. 2016-Oct-15 00:00:00.0000 TDB
pub fn solar_system() -> (DateTime<Utc>, Vec<Body>) {
    let time = Utc.ymd(2016, 10, 15).and_hms(0, 0, 0);
    let mut bodies = Vec::default();

    bodies.push(Body {
        position: Vector::default(),
        velocity: Vector::default(),
        mass: Mass::from_kgs(1988500e24),
        name: "Sun".to_string(),
    });

    bodies.push(Body {
        position: Vector {
            x: -3.610946582889994E-01,
            y: 7.655753687572452E-02,
            z: 3.938313941762204E-02,
        },
        velocity: Vector {
            x: -1.166921930880622E-02,
            y: -2.631562924335937E-02,
            z: -1.079745298798429E-03,
        },
        mass: Mass::from_kgs(3.302e23),
        name: "Mercury".to_string(),
    });

    bodies.push(Body {
        position: Vector {
            x: 1.973338103014433E-01,
            y: -7.001287841606206E-01,
            z: -2.098736267890693E-02,
        },
        velocity: Vector {
            x: 1.933209186041313E-02,
            y: 5.418163683984627E-03,
            z: -1.041291312991296E-03,
        },
        mass: Mass::from_kgs(48.685e23),
        name: "Venus".to_string(),
    });

    bodies.push(Body {
        position: Vector {
            x: 9.255531850624332E-01,
            y: 3.708906496672169E-01,
            z: -1.740719250795340E-05,
        },
        velocity: Vector {
            x: -6.678671400247983E-03,
            y: 1.589848110988471E-02,
            z: 5.510397457798760E-08,
        },
        mass: Mass::from_kgs(5.97219e24),
        name: "Earth".to_string(),
    });

    bodies.push(Body {
        position: Vector {
            x: 9.279584884887349E-01,
            y: 3.711104444050654E-01,
            z: -1.037121134101948E-04,
        },
        velocity: Vector {
            x: -6.761640228117041E-03,
            y: 1.651990549601004E-02,
            z: -5.110814081371464E-05,
        },
        mass: Mass::from_kgs(7.349e22),
        name: "Moon".to_string(),
    });

    bodies.push(Body {
        position: Vector {
            x: 1.158495532744392E+00,
            y: -7.536556465639087E-01,
            z: -4.422526991966770E-02,
        },
        velocity: Vector {
            x: 8.162919959786599E-03,
            y: 1.292885802502735E-02,
            z: 7.059671810539333E-05,
        },
        mass: Mass::from_kgs(6.4171e23),
        name: "Mars".to_string(),
    });

    bodies.push(Body {
        position: Vector {
            x: -5.432121324842138E+00,
            y: -4.569844296390864E-01,
            z: 1.234480663921852E-01,
        },
        velocity: Vector {
            x: 5.420082489396598E-04,
            y: -7.169749225100362E-03,
            z: 1.762648657670620E-05,
        },
        mass: Mass::from_kgs(189818722e19),
        name: "Jupiter".to_string(),
    });

    bodies.push(Body {
        position: Vector {
            x: -2.270616615877762e+00,
            y: -9.778225136344251e+00,
            z: 2.603431483798866e-01,
        },
        velocity: Vector {
            x: 5.130339671553237E-03,
            y: -1.285344196202056E-03,
            z: -1.815947060563043E-04,
        },
        mass: Mass::from_kgs(5.6834E26),
        name: "Saturn".to_string(),
    });

    bodies.push(Body {
        position: Vector {
            x: 2.825736958467999e+01,
            y: -9.925704611354977e+00,
            z: -4.468121470675587e-01,
        },
        velocity: Vector {
            x: 1.020926560027260e-03,
            y: 2.974039863467131e-03,
            z: -8.451314801496931e-05,
        },
        mass: Mass::from_kgs(1.024e26),
        name: "Neptune".to_string(),
    });

    (time, bodies)
}
