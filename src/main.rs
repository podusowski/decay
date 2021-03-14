extern crate piston_window;
use piston_window::*; 
use piston_window::types::Matrix2d;
use piston_window::math::translate;

mod algebra;
mod ephemeris;
mod physics;

use physics::*;

struct Distance(f64);

impl Distance {
    fn from_meters(meters: f64) -> Self {
        Distance(meters)
    }

    fn as_meters(&self) -> f64 {
        self.0
    }

    fn as_au(&self) -> f64 {
        self.0 / 597870700.0
    }
}

fn main() {
    let mut space = Space::solar_system();
    println!("Space: {:?}", space);

    let mut window: PistonWindow = WindowSettings::new("decay", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs = window.load_font("./FiraSans-Regular.ttf").unwrap();

    let view_transform = translate([400.0, 400.0]);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            for body in &space.bodies {
                let x = Distance::from_meters(body.position.x).as_au() * 100.0;
                let y = Distance::from_meters(body.position.y).as_au() * 100.0;

                ellipse(
                    [1.0; 4],
                    [x, y, 10.0, 10.0],
                    context.transform.append_transform(view_transform),
                    graphics,
                );

                text(
                    [0.7; 4],
                    10,
                    body.name,
                    &mut glyphs,
                    context
                        .transform
                        .trans(x + 10.0, y)
                        .append_transform(view_transform),
                    graphics,
                )
                .unwrap();
            }
            glyphs.factory.encoder.flush(device);

            space.tick(std::time::Duration::from_millis(100));
            println!("Space: {:?}", space);
        });
    }
}
