mod physics;
use physics::*;

fn main() {
    println!("Bang!");

    let mut space = physics::Space::default();

    space.bodies.push(Body {
        position: Vector { x: 0.0, y: 0.0 },
        velocity: Vector { x: 0.0, y: 0.0 },
        mass: 100.0,
    });

    space.bodies.push(Body {
        position: Vector { x: 0.0, y: 100.0 },
        velocity: Vector { x: 0.0, y: 0.0 },
        mass: 100.0,
    });

    for _ in 0..50{
        println!("{:?}", space);
        space.tick(std::time::Duration::from_secs(1));
    }
}
