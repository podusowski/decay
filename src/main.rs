use algebra::Vector;

mod algebra;
mod physics;
mod units;
mod ephemeris;
mod physical_properties;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(ephemeris::create_solar_system)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_system(physics::newtonian_gravity)
        .add_system(physics::move_bodies)
        .run();
}
