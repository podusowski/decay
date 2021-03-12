extern crate piston_window;
use piston_window::*;

mod physics;
mod algebra;

use physics::*;
use algebra::Vector;

fn main() {
    let mut space = physics::Space::default();

    space.bodies.push(Body {
        position: Vector { x: 400.0, y: 300.0 },
        velocity: Vector { x: 0.0, y: -10.0 },
        mass: 1200000.0,
    });

    space.bodies.push(Body {
        position: Vector { x: 300.0, y: 300.0 },
        velocity: Vector { x: 0.0, y: 10.0 },
        mass: 1200000.0,
    });

    let mut window: PistonWindow = WindowSettings::new("decay", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs = window.load_font("./FiraSans-Regular.ttf").unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            for body in &space.bodies {
                ellipse(
                    [1.0; 4],
                    [body.position.x, body.position.y, 10.0, 10.0],
                    context.transform,
                    graphics,
                );
                text(
                    [0.7; 4],
                    10,
                    "Hello",
                    &mut glyphs,
                    context.transform.trans(body.position.x + 10.0, body.position.y),
                    graphics,
                )
                .unwrap();
            }
            glyphs.factory.encoder.flush(device);

            space.tick(std::time::Duration::from_millis(10));
        });
    }
}
