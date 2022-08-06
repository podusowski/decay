use std::sync::{Arc, Mutex, RwLock};

use algebra::Vector;

mod algebra;
mod ephemeris;
mod physics;
mod units;

use physics::*;

use crate::units::Distance;

use bevy::prelude::*;

#[derive(Component)]
struct Body;

#[derive(Component)]
struct Name(String);

fn create_solar_system(mut commands: Commands) {
    let space = Space::<()>::solar_system(|| ());

    for body in space.bodies {
        commands.spawn().insert(Body).insert(Name(body.name.into()));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_solar_system)
        .run();
}
