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

fn create_solar_system(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let space = Space::<()>::solar_system(|| ());

    for body in space.bodies {
        //let ball = ass.load("ball.glb");

        commands
            .spawn()
            //.insert_bundle(SceneBundle {
            //    scene: ball,
            //    ..Default::default()
            //})
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere{ radius: 30000000000.0, subdivisions: 50 })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform::from_xyz(body.position.x as f32, body.position.y as f32, body.position.z as f32),
                ..default()
            })
            .insert(Body {
                position: body.position,
                velocity: body.velocity,
                mass: body.mass,
            })
            .insert(GravitationalForce::default())
            .insert(Name(body.name.into()));

        //commands.spawn_bundle(PbrBundle {
        //    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        //    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        //    transform: Transform::from_xyz(0.0, 0.5, 0.0),
        //    ..default()
        //});
    }

    //commands.spawn_bundle(PointLightBundle {
    //    point_light: PointLight {
    //        intensity: 1500.0,
    //        shadows_enabled: true,
    //        ..default()
    //    },
    //    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //    ..default()
    //});

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, -2.5, 5000000000000.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_system(gravitational_force)
        .run();
}
