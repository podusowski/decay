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

fn move_single(time: f64, force: Vector, body: &mut Body) {
    let acceleration = force / body.mass().as_kgs();
    let offset_ensued_from_velocity = body.velocity * time as f64;
    let offset_ensued_from_acceleration = acceleration * time.powf(2.) as f64 / 2.0;

    body.velocity = acceleration * time + body.velocity;
    eprintln!("vel {:?}", body.velocity);
    //body.position = body.position + offset_ensued_from_acceleration + offset_ensued_from_velocity;
}

fn newtownian_gravity(time: Res<Time>, mut query: Query<(&mut Body, &mut Transform)>) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut body1, mut transform1), (mut body2, mut transform2)]) =
        combinations.fetch_next()
    {
        let time = time.delta_seconds_f64() * 1000000.;
        let force = body1.newtonian_gravity(&*body2);

        move_single(time, force, &mut body1);
        move_single(time, -force, &mut body2);

        *transform1 = Transform::from_xyz(
            body1.position.x as f32,
            body1.position.y as f32,
            body1.position.z as f32,
        );

        *transform2 = Transform::from_xyz(
            body2.position.x as f32,
            body2.position.y as f32,
            body2.position.z as f32,
        );
    }
}

fn move_bodies(time: Res<Time>, mut query: Query<&mut Body>) {
    let time = time.delta_seconds_f64() * 1000000.;
    for mut body in query.iter_mut() {
        let offset_ensued_from_velocity = body.velocity * time as f64;
        body.position = body.position + offset_ensued_from_velocity;
    }
    eprintln!("dupa {}", time);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_solar_system)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_system(newtownian_gravity)
        .add_system(move_bodies)
        .run();
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use approx::assert_abs_diff_eq;

    use super::Body;
    use super::*;

    fn rewind_time(world: &mut World, duration: Duration) {
        let mut time = world.resource_mut::<Time>();
        let last_update = time.last_update().unwrap();
        time.update_with_instant(last_update + duration);
    }

    #[test]
    fn one_body_stays_in_place() {
        let mut app = App::new();

        app.add_system(newtownian_gravity);
        app.add_system(move_bodies);

        let mut time = Time::default();
        time.update();
        app.world.insert_resource(time);

        let id = app
            .world
            .spawn()
            .insert(Body {
                position: Vector::default(),
                velocity: Vector::default(),
                mass: Mass::from_kgs(1.0),
                name: "Earth".into(),
            })
            .id();

        app.update();

        // See if position is still the same.
        assert_eq!(
            Vector::default(),
            app.world.get::<Body>(id).unwrap().position
        );

        // Now let's see if position is still the same after another second.
        rewind_time(&mut app.world, Duration::from_secs(1));
        app.update();

        assert_eq!(
            Vector::default(),
            app.world.get::<Body>(id).unwrap().position
        );
    }

    #[test]
    fn two_bodies_fly_towards_each_other() {
        let mut app = App::new();

        app.add_system(newtownian_gravity);
        app.add_system(move_bodies);

        let mut time = Time::default();
        time.update();
        app.world.insert_resource(time);

        let id1 = app
            .world
            .spawn()
            .insert(Body {
                position: Vector::default(),
                velocity: Vector::default(),
                mass: Mass::from_kgs(1.0),
                name: "first".into(),
            })
            .id();

        let id2 = app
            .world
            .spawn()
            .insert(Body {
                position: Vector {
                    x: 1.,
                    ..Default::default()
                },
                velocity: Vector::default(),
                mass: Mass::from_kgs(1.0),
                name: "second".into(),
            })
            .id();

        rewind_time(&mut app.world, Duration::from_secs(1));
        app.update();
        rewind_time(&mut app.world, Duration::from_secs(1));
        app.update();

        // Distance between the two, their mass product and square of distance, all equals 1.
        // This gives a gravity force equal to G. With the mass of 1, such force will give
        // the acceleration of G [m per sec per sec]. After one second such acceleration should
        // give the velocity of G.
        assert_abs_diff_eq!(G, app.world.get::<Body>(id1).unwrap().velocity.x);

        // For both bodies.
        assert_abs_diff_eq!(-G, app.world.get::<Body>(id2).unwrap().velocity.x);

        // Distance traveled should be:
        // a * t ^ 2 / 2
        // G * 1 ^ 2 / 2
        // G / 2
        //assert_abs_diff_eq!(G / 2.0, space.bodies[0].position.x);
    }
}
