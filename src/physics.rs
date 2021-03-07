use std::{iter::Sum, time, usize};

struct Newton(usize);

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

pub struct Body {
    x: usize,
    y: usize,
    velocity: Velocity,
    mass: usize,
}

impl Body {
    pub fn gravity_force(&self, other: &Body) -> ForceVector {
        ForceVector {
            x: 0,
            y: 0,
        }
    }
}

pub struct Space /* perhaps time some day... */ {
    bodies: Vec<Body>,
}

impl Default for Space {
    fn default() -> Self {
        Space {
            bodies: Vec::default(),
        }
    }
}

impl Space {
    pub fn tick(&mut self, delta_time: std::time::Duration) {
        println!("Tick!");
        for body in &self.bodies {
            let cumulative_force: ForceVector = self
                .bodies
                .iter()
                .map(|other| body.gravity_force(&other))
                .fold(ForceVector::default(), std::ops::Add::add);
        }
    }
}
