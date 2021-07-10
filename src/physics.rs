use crate::algebra::Vector;
use crate::units::Mass;

// Object having a mass and position in space.
trait MassObject {
    fn mass(&self) -> Mass;
    fn position(&self) -> Vector;

    fn newtonian_gravity(&self, other: &impl MassObject) -> Vector {
        // Pauli exclusion principle FTW!
        if self.position() == other.position() {
            Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            let offset = self.position() - other.position();
            -G * ((self.mass().as_kgs() * other.mass().as_kgs()) / offset.length().powi(2))
                * offset.normalized()
        }
    }
}

#[derive(Debug)]
pub struct Body {
    pub position: Vector,
    pub velocity: Vector,
    pub mass: Mass,
    pub name: &'static str,
}

impl MassObject for Body {
    fn mass(&self) -> Mass {
        self.mass
    }

    fn position(&self) -> Vector {
        self.position
    }
}

/// Ships are objects too, but unlike regular bodies they can move by their own.
#[derive(Debug)]
pub struct Ship {
    pub position: Vector,
    pub velocity: Vector,
    pub thrust: Vector,
    pub name: &'static str,
}

impl MassObject for Ship {
    fn mass(&self) -> Mass {
        Mass::from_kgs(10000.0)
    }

    fn position(&self) -> Vector {
        self.position
    }
}

const G: f64 = 6.67408e-11f64;

#[derive(Debug)]
pub struct Space /* perhaps time some day... */ {
    pub time: chrono::DateTime<chrono::Utc>,
    pub bodies: Vec<Body>,
    pub ships: Vec<Ship>,
}

impl Default for Space {
    fn default() -> Self {
        Space {
            time: chrono::Utc::now(),
            bodies: Default::default(),
            ships: Default::default(),
        }
    }
}

impl Space {
    fn cumulative_gravity_force(&self, body: &impl MassObject) -> Vector {
        self.bodies
            .iter()
            .map(|other| body.newtonian_gravity(other))
            .fold(Vector::default(), std::ops::Add::add)
    }

    pub fn tick(&mut self, delta_time: chrono::Duration) {
        // Need to use this barbaric loop to trick the borrow checker a bit.
        for i in 0..self.bodies.len() {
            let body = &self.bodies[i];
            let force = self.cumulative_gravity_force(body);
            let acceleration = force / body.mass.as_kgs();

            // Calculate this before we store the new velocity.
            let offset_ensued_from_velocity = body.velocity * delta_time.num_seconds() as f64;
            let offset_ensued_from_acceleration =
                acceleration * delta_time.num_seconds().pow(2) as f64 / 2.0;

            let body = &mut self.bodies[i];

            body.velocity = acceleration * delta_time.num_seconds() as f64 + body.velocity;
            body.position =
                body.position + offset_ensued_from_velocity + offset_ensued_from_acceleration;
        }

        // Now update velocities and positions of ships.
        for i in 0..self.ships.len() {
            let ship = &self.ships[i];
            let force = self.cumulative_gravity_force(ship) + ship.thrust;
            let acceleration = force / ship.mass().as_kgs();

            let offset_ensued_from_velocity = ship.velocity * delta_time.num_seconds() as f64;
            let offset_ensued_from_acceleration =
                acceleration * delta_time.num_seconds().pow(2) as f64 / 2.0;

            let ship = &mut self.ships[i];

            ship.velocity = acceleration * delta_time.num_seconds() as f64 + ship.velocity;
            ship.position =
                ship.position + offset_ensued_from_velocity + offset_ensued_from_acceleration;
        }

        self.time = self.time.checked_add_signed(delta_time).unwrap();
    }

    pub fn body_at(&self, position: Vector) -> Option<usize> {
        for i in 0..self.bodies.len() {
            println!("dist: {:?}", (self.bodies[i].position - position).length());
            if (self.bodies[i].position - position).length() < 10e10 {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::Mass;
    use approx::assert_abs_diff_eq;
    use chrono::Duration;

    #[test]
    fn one_body_stays_in_place() {
        let mut space = Space::default();
        space.bodies.push(Body {
            position: Vector::default(),
            velocity: Vector::default(),
            mass: Mass::from_kgs(1.0),
            name: "Earth",
        });
        space.tick(Duration::seconds(1));
        assert_eq!(Vector::default(), space.bodies[0].position);
        assert_eq!(Vector::default(), space.bodies[0].velocity);
    }

    #[test]
    fn two_bodies_fly_towards_each_other() {
        let mut space = Space::default();
        space.bodies.push(Body {
            position: Vector::default(),
            velocity: Vector::default(),
            mass: Mass::from_kgs(1.0),
            name: "Earth",
        });
        space.bodies.push(Body {
            position: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: Vector::default(),
            mass: Mass::from_kgs(1.0),
            name: "Earth",
        });

        space.tick(Duration::seconds(1));

        // Distance between the two, their mass product and square of distance, all equals 1.
        // This gives a gravity force equal to G. With the mass of 1, such force will give
        // the acceleration of G [m per sec per sec]. After one second such acceleration should
        // give the velocity of G.
        assert_abs_diff_eq!(G, space.bodies[0].velocity.x);

        // For both bodies.
        assert_abs_diff_eq!(-G, space.bodies[1].velocity.x);

        // Distance traveled should be:
        // a * t ^ 2 / 2
        // G * 1 ^ 2 / 2
        // G / 2
        assert_abs_diff_eq!(G / 2.0, space.bodies[0].position.x);
    }
}
