use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uom::si::mass::gram;
pub use uom::si::{f64::Mass, mass::kilogram};

use crate::algebra::Vector;

const G: f64 = 6.67408e-11f64;

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct Body {
    pub name: String,
    //#[serde(with = "mass_serializer")]
    pub mass: Mass,
    pub position: Vector,
    pub velocity: Vector,
}

impl Body {
    fn update_velocity(&mut self, time: f64, force: Vector) {
        let acceleration = force / self.mass.get::<gram>();
        self.velocity = self.velocity + acceleration * time;
    }

    /// Gravity force between this and other body.
    fn gravity_force(&self, other: &Body) -> Vector {
        // Pauli exclusion principle FTW!
        if self.position == other.position {
            Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            let offset = self.position - other.position;
            -G * ((self.mass.get::<kilogram>() * other.mass.get::<kilogram>())
                / offset.length().powi(2))
                * offset.normalized()
        }
    }
}

/// Bevy system which simulates newtonian physics for all entities with `Body`
/// component. Computed positions are then written into `Transform` component.
pub fn newtonian_gravity(time: Res<Time>, mut query: Query<(&mut Body, &mut Transform)>) {
    let time = time.delta_seconds_f64() * crate::time::TIME_SCALE;

    // Compute velocities.
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut body1, _), (mut body2, _)]) = combinations.fetch_next() {
        let force = body1.gravity_force(&*body2) * 0.001;
        body1.update_velocity(time, force);
        body2.update_velocity(time, -force);
    }

    // Update positions with previously calculated velocities.
    for (mut body, mut transform) in query.iter_mut() {
        let offset_ensued_from_velocity = body.velocity * time as f64;
        body.position = body.position + offset_ensued_from_velocity;

        // Synchronize internal (physics related) state with Bevy's metadata
        // for the renderer. To be clarified whether it would be better if
        // physics code have been operating on `Transform` component directly.
        *transform = Transform::from_xyz(
            body.position.x as f32,
            body.position.y as f32,
            body.position.z as f32,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics;
    use approx::assert_abs_diff_eq;
    use std::time::Duration;

    fn rewind_time(world: &mut World, duration: Duration) {
        let mut time = world.resource_mut::<Time>();
        let last_update = time.last_update().unwrap();
        time.update_with_instant(last_update + duration);
    }

    #[test]
    fn one_body_stays_in_place() {
        let mut app = App::new();

        app.add_system(newtonian_gravity);

        let mut time = Time::default();
        time.update();
        app.world.insert_resource(time);

        let id = app
            .world
            .spawn()
            .insert(Body {
                position: Vector::default(),
                velocity: Vector::default(),
                mass: Mass::new::<physics::kilogram>(1.0),
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

        let mut time = Time::default();
        time.update();
        app.world.insert_resource(time);

        let id1 = app
            .world
            .spawn()
            .insert(Body {
                position: Vector::default(),
                velocity: Vector::default(),
                mass: Mass::new::<physics::kilogram>(1.0),
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
                mass: Mass::new::<physics::kilogram>(1.0),
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

        // TODO: Check Transform component instead of Body::position!!
        assert_abs_diff_eq!(
            G * crate::time::TIME_SCALE * 1000.,
            app.world.get::<Body>(id1).unwrap().position.x,
            epsilon = 0.01
        );
    }
}
