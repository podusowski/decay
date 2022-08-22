use algebra::Vector;

mod algebra;
mod physics;
mod units;

use physics::*;
use units::Distance;

use bevy::prelude::*;
use uom::si::mass::gram;

#[derive(Component)]
struct Name(String);

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

const TIME_SCALE: f64 = 1000000000.;

fn create_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let file = std::fs::File::open("ephemeris.yaml").expect("could not open ephemeris file");
    let bodies: Vec<Body> = serde_yaml::from_reader(file).expect("could not parse ephemeris file");

    for body in bodies {
        commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 30000000000.0,
                    subdivisions: 50,
                })),
                material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                ..default()
            })
            .insert(Body {
                position: body.position.aus_to_meters(),
                velocity: body.velocity.km_per_second_to_meters_per_second(),
                mass: body.mass,
                name: body.name.clone(),
            })
            .insert(Name(body.name.clone()));
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

fn move_single(time: f64, force: Vector, body: &mut Body) {
    let acceleration = force / body.mass().get::<gram>();
    body.velocity = acceleration * time + body.velocity;
}

fn newtonian_gravity(time: Res<Time>, mut query: Query<(&mut Body, &mut Transform)>) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut body1, mut transform1), (mut body2, mut transform2)]) =
        combinations.fetch_next()
    {
        let time = time.delta_seconds_f64() * TIME_SCALE;
        let force = body1.newtonian_gravity(&*body2) * 0.001;

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
    let time = time.delta_seconds_f64() * TIME_SCALE;
    for mut body in query.iter_mut() {
        let offset_ensued_from_velocity = body.velocity * time as f64;
        body.position = body.position + offset_ensued_from_velocity;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_solar_system)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_system(newtonian_gravity)
        .add_system(move_bodies)
        .run();
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use approx::assert_abs_diff_eq;

    use crate::units::Mass;

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

        app.add_system(newtonian_gravity);
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

        app.add_system(newtonian_gravity);
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
            .insert(Transform::default())
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
            .insert(Transform::default())
            .id();

        rewind_time(&mut app.world, Duration::from_secs(1));
        app.update();

        // Distance between the two, their mass product and square of distance, all equals 1.
        // This gives a gravity force equal to G. With the mass of 1, such force will give
        // the acceleration of G [m per sec per sec]. After one second such acceleration should
        // give the velocity of G.
        assert_abs_diff_eq!(
            G,
            app.world.get::<Body>(id1).unwrap().velocity.x,
            epsilon = 0.01
        );

        // For both bodies.
        assert_abs_diff_eq!(
            -G,
            app.world.get::<Body>(id2).unwrap().velocity.x,
            epsilon = 0.01
        );

        // Distance traveled should be:
        // a * t ^ 2 / 2
        // G * 1 ^ 2 / 2
        // G / 2
        // TODO: Check Transform component instead of Body::position!!
        assert_abs_diff_eq!(
            G / 2.0,
            app.world.get::<Body>(id1).unwrap().position.x,
            epsilon = 0.01
        );
    }
}
