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

    loop {
        space.tick(std::time::Duration::from_secs(1));
        println!("{:?}", space);
    }
}
