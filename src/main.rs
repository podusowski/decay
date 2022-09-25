use algebra::Vector;

mod algebra;
mod physics;
mod units;
mod ephemeris;

use chrono::{Duration, Utc};
use physics::*;
use units::Distance;

use bevy::prelude::*;

impl Vector {
    // This is wrong and ugly in so many ways. Ultimate goal is to cleanup
    // all the units so they are safe and Bevy compatible.

    /// Converts AUs into meters, assuming the values are AUs at first place.
    fn aus_to_meters(self) -> Self {
        Self {
            x: Distance::from_aus(self.x).as_meters(),
            y: Distance::from_aus(self.y).as_meters(),
            z: Distance::from_aus(self.z).as_meters(),
        }
    }

    fn km_per_second_to_meters_per_second(self) -> Self {
        Self {
            x: self.x * 1000.,
            y: self.y * 1000.,
            z: self.z * 1000.,
        }
    }
}

async fn fetch_body(body: &rhorizons::MajorBody) -> Body {
    // TODO: These should be constructed only once.
    let start_time = Utc::now() - Duration::days(1);
    let stop_time = Utc::now();

    let vectors = rhorizons::ephemeris(body.id, start_time, stop_time).await;
    Body {
        name: body.name.clone(), // TODO: Try getting rid of it.
        mass: physics::Mass::new::<physics::kilogram>(0.),
        position: Vector {
            x: vectors[0].position[0] as f64,
            y: vectors[0].position[1] as f64,
            z: vectors[0].position[2] as f64,
        },
        velocity: Vector {
            x: vectors[0].velocity[0] as f64,
            y: vectors[0].velocity[1] as f64,
            z: vectors[0].velocity[2] as f64,
        },
    }
}

fn fetch_ephemeris() -> Vec<Body> {
    // `rhorizons` crate is asynchronous, but Bevy isn't.
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            info!("Getting Solar System bodies from NASA JPL Horizons.");
            // TODO: unuglyfy this!
            let bodies = rhorizons::major_bodies().await;
            let major_bodies = bodies
                .iter()
                .filter(|body| ["Mercury"].contains(&body.name.as_str()));

            let mut bodies = Vec::new();
            for major_body in major_bodies {
                bodies.push(fetch_body(major_body).await);
            }
            bodies
        })
}

fn ensure_ephemeris() {
    for body in fetch_ephemeris() {
        eprintln!("{:?}", body);
    }
}

fn main() {
    ensure_ephemeris();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(ephemeris::create_solar_system)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_system(physics::newtonian_gravity)
        .add_system(physics::move_bodies)
        .run();
}
