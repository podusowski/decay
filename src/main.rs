mod physics;
use physics::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
extern crate piston_window;
use piston_window::*;

fn main() {
    println!("Bang!");

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

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);
            for body in &space.bodies {
                ellipse(
                    [1.0; 4],
                    [body.position.x, body.position.y, 10.0, 10.0],
                    context.transform,
                    graphics,
                );
            }

            println!("{:?}", space);
            space.tick(std::time::Duration::from_millis(10));
        });
    }
}
