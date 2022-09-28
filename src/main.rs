mod algebra;
mod ephemeris;
mod knowledge;
mod physics;
mod units;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(ephemeris::spawn_solar_system)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_system(physics::newtonian_gravity)
        .run();
}
