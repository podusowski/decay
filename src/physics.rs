use crate::algebra::Vector;

#[derive(Debug)]
pub struct Body {
    pub position: Vector,
    pub velocity: Vector,
    pub mass: f64,
}

impl Body {
    pub fn newtonian_gravity(&self, other: &Body) -> Vector {
        const G: f64 = 6.67408e-11f64;
        let offset = &self.position - &other.position;
        -G * ((self.mass * other.mass) / offset.length()) * offset.normalized()
    }
}

#[derive(Debug)]
pub struct Space /* perhaps time some day... */ {
    pub bodies: Vec<Body>,
}

impl Default for Space {
    fn default() -> Self {
        Space {
            bodies: Vec::default(),
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
            let body = &mut self.bodies[i];
            body.velocity = body.velocity + force * delta_time.as_secs_f64();

            // Pretty sure the World doesn't do it in a single loop
            body.position = body.position + body.velocity * delta_time.as_secs_f64();
        }
    }
}
