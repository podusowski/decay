#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn normalized(&self) -> Self {
        self / self.length()
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl std::ops::Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub struct ForceVector {
    x: usize,
    y: usize,
}

pub struct Velocity {
    x: usize,
    y: usize,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl std::ops::Add for ForceVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Default for ForceVector {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn newtonian_gravitation(m1: f64, m2: f64, distance: f64) -> f64 {
    const G: f64 = 6.67408e-11f64;
    G * (m1 + m2) / distance.powf(2.0)
}

#[derive(Debug)]
pub struct Body {
    pub position: Vector,
    pub velocity: Vector,
    pub mass: f64,
}

impl Body {
    pub fn gravity_force(&self, rhs: &Body) -> Vector {
        const G: f64 = 6.67408e-11f64;
        let offset = &self.position - &rhs.position;
        -G * ((self.mass * rhs.mass) / offset.length()) * offset.normalized()
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
            .map(|other| body.gravity_force(&other))
            .fold(Vector::default(), std::ops::Add::add)
    }

    pub fn tick(&mut self, delta_time: std::time::Duration) {
        for i in 0..self.bodies.len() {
            // A hack to trick the borrow checker.
            let body = &self.bodies[i];
            let force = self.cumulative_force(body);
            let body = &mut self.bodies[i];
            body.velocity = body.velocity + force * delta_time.as_secs_f64();

            // Pretty sure the World doesn't do it in a single loop
            body.position = body.position + body.velocity * delta_time.as_secs_f64();
        }
    }
}
