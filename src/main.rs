use algebra::Vector;

mod algebra;
mod ephemeris;
mod physics;
mod units;

use physics::*;
use units::Mass;

use bevy::prelude::*;

#[derive(Component)]
struct Body {
    pub position: Vector,
    pub velocity: Vector,
    pub mass: Mass,
    pub name: String,
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let space = Space::<()>::solar_system(|| ());

    for body in space.bodies {
        commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 30000000000.0,
                    subdivisions: 50,
                })),
                material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                transform: Transform::from_xyz(
                    body.position.x as f32,
                    body.position.y as f32,
                    body.position.z as f32,
                ),
                ..default()
            })
            .insert(Body {
                position: body.position,
                velocity: body.velocity,
                mass: body.mass,
                name: body.name.into(),
            })
            .insert(GravitationalForce::default())
            .insert(Name(body.name.into()));
    }

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, -2.5, 5000000000000.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
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

fn newtownian_gravity(time: Res<Time>, mut query: Query<(&mut Body, &mut Transform)>) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut first, mut first_transform), (second, seocnd_transform)]) =
        combinations.fetch_next()
    {
        let time = time.delta_seconds_f64() * 1000000.;
        let force = first.newtonian_gravity(&*second);

        let acceleration_of_first = force / first.mass().as_kgs();
        let offset_ensued_from_velocity = first.velocity * time as f64;
        let offset_ensued_from_acceleration = acceleration_of_first * time.powf(2.) as f64 / 2.0;

        first.velocity = acceleration_of_first * time + first.velocity;

        if first.name == "Mercury" {
            //eprintln!("force {:?}", force);
            //eprintln!("velocity {:?}", first.velocity);
        }

        first.position =
            first.position + offset_ensued_from_acceleration + offset_ensued_from_velocity;

        let new_t = Transform::from_xyz(
            first.position.x as f32,
            first.position.y as f32,
            first.position.z as f32,
        );

        if first.name == "Mercury" {
            //eprintln!("{:?} -> {:?}", first_transform, new_t);
        }

        *first_transform = new_t;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_solar_system)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_system(newtownian_gravity)
        .run();
}
