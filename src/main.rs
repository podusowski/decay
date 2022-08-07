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

impl MassObject for Body {
    fn mass(&self) -> Mass {
        self.mass
    }

    fn position(&self) -> Vector {
        self.position
    }
}

/// The force that all other bodies act on this body.
#[derive(Component, Default)]
struct GravitationalForce(Vector);

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
            .insert(GravitationalForce::default())
            .insert(Name(body.name.into()));
    }
}

/// Calculates gravitational forces of all bodies. The forces can be then used
/// by other system to calculate the movements.
fn gravitational_force(mut forces: Query<(&mut GravitationalForce, &Body)>, query: Query<&Body>) {
    for (mut force, body) in forces.iter_mut() {
        force.0 = query
            .iter()
            .map(|other| body.newtonian_gravity(other))
            .fold(Vector::default(), std::ops::Add::add)
    }
}

fn newtownian_movement(time: Res<Time>, query: Query<&Body>) {
    for body in &query {
        //let force = ;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_solar_system)
        .add_system(gravitational_force)
        .run();
}
