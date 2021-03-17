use crate::algebra::Vector;

pub struct Distance(f64);

impl Distance {
    const METERS_IN_AU: f64 = 149597870700.0;

    pub fn from_meters(meters: f64) -> Self {
        Distance(meters)
    }

    pub fn from_aus(aus: f64) -> Self {
        Distance(aus * Self::METERS_IN_AU)
    }

    pub fn as_meters(&self) -> f64 {
        self.0
    }

    pub fn as_au(&self) -> f64 {
        self.0 / Self::METERS_IN_AU
    }
}

#[derive(Debug)]
pub struct Mass(f64);

impl Mass {
    pub fn from_kgs(kgs: f64) -> Mass {
        Mass(kgs * 1000.0)
    }

    pub fn as_kgs(&self) -> f64 {
        self.0 / 1000.0
    }
}

#[derive(Debug)]
pub struct Body {
    pub position: Vector,
    pub velocity: Vector,
    pub mass: Mass,
    pub name: &'static str,
}

const G: f64 = 6.67408e-11f64;

impl Body {
    pub fn newtonian_gravity(&self, other: &Body) -> Vector {
        let offset = &self.position - &other.position;
        -G * ((self.mass.as_kgs() * other.mass.as_kgs()) / offset.length().powi(2))
            * offset.normalized()
    }
}

#[derive(Debug)]
pub struct Space /* perhaps time some day... */ {
    pub bodies: Vec<Body>,
    pub time: std::time::Instant,
}

impl Default for Space {
    fn default() -> Self {
        Space {
            bodies: Vec::default(),
            time: std::time::Instant::now(),
        }
    }
}

impl Space {
    fn cumulative_force(&self, body: &Body) -> Vector {
        self.bodies
            .iter()
            .filter(|&other| !std::ptr::eq(body, other))
            .map(|other| body.newtonian_gravity(&other))
            .fold(Vector::default(), std::ops::Add::add)
    }

    pub fn tick(&mut self, delta_time: std::time::Duration) {
        // Need to use this barbaric loop to trick the borrow checker a bit.
        for i in 0..self.bodies.len() {
            let body = &self.bodies[i];
            let force = self.cumulative_force(body);
            let acceleration = force / body.mass.as_kgs();

            // Calculate this before we store the new velocity.
            let offset_ensued_from_velocity = body.velocity * delta_time.as_secs_f64();
            let offset_ensued_from_acceleration =
                acceleration * delta_time.as_secs_f64().powi(2) / 2.0;

            let body = &mut self.bodies[i];

            body.velocity = acceleration * delta_time.as_secs_f64() + body.velocity;
            body.position =
                body.position + offset_ensued_from_velocity + offset_ensued_from_acceleration;
        }

        self.time += delta_time;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn one_body_stays_in_place() {
        let mut space = Space::default();
        space.bodies.push(Body {
            position: Vector::default(),
            velocity: Vector::default(),
            mass: Mass::from_kgs(1.0),
            name: "Earth"
        });
        space.tick(std::time::Duration::from_secs(1));
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
            name: "Earth"
        });
        space.bodies.push(Body {
            position: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: Vector::default(),
            mass: Mass::from_kgs(1.0),
            name: "Earth"
        });

        space.tick(std::time::Duration::from_secs(1));

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
