use std::sync::{Arc, Mutex, RwLock};

use algebra::Vector;

mod algebra;
mod ephemeris;
mod physics;
mod units;

use physics::*;
use units::Mass;

use crate::units::Distance;

use bevy::prelude::*;

#[derive(Component)]
struct Body {
    pub position: Vector,
    pub velocity: Vector,
    pub mass: Mass,
}

#[derive(Component)]
struct Name(String);

fn create_solar_system(mut commands: Commands) {
    let space = Space::<()>::solar_system(|| ());

    for body in space.bodies {
        commands
            .spawn()
            .insert(Body {
                position: body.position,
                velocity: body.velocity,
                mass: body.mass,
            })
            .insert(Name(body.name.into()));
    }
}

fn newtownian_movement(query: Query<&Body>) {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_solar_system)
        .run();
}
