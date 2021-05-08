extern crate piston_window;
use algebra::Vector;
use piston_window::*;
use piston_window::{math::translate, types::Matrix2d};

mod algebra;
mod ephemeris;
mod physics;
mod units;

use physics::*;

struct Observer {
    view_transform: Matrix2d<f64>,
    au_as_pixels: f64,
}

impl Observer {
    fn zoom_in_out(&mut self, amount: f64) {
        self.au_as_pixels = (self.au_as_pixels + amount).max(1.0);
    }

    fn center_at(&mut self, position: algebra::Vector) {
        let (x, y) = self.cast(position);
        self.view_transform = translate([x + 1280.0 / 2.0, y + 720.0 / 2.0]);
    }

    fn cast(&self, position: algebra::Vector) -> (f64, f64) {
        (
            units::Distance::from_meters(position.x).as_au() * self.au_as_pixels,
            units::Distance::from_meters(position.y).as_au() * self.au_as_pixels,
        )
    }

    fn reverse_cast(&self, position: (f64, f64)) -> Vector {
        Vector {
            x: units::Distance::from_aus(position.0 / self.au_as_pixels).as_meters(),
            y: units::Distance::from_aus(position.1 / self.au_as_pixels).as_meters(),
            z: Default::default(),
        }
    }
}

impl Default for Observer {
    fn default() -> Self {
        Observer {
            view_transform: Default::default(),
            au_as_pixels: 20.0,
        }
    }
}

fn main() {
    let mut space = Space::solar_system();
    println!("Space: {:?}", space);

    let mut window: PistonWindow = WindowSettings::new("decay", [1280, 720])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs = window.load_font("./FiraSans-Regular.ttf").unwrap();

    let mut observer = Observer::default();
    let the_big_bang_instant = space.time;

    while let Some(event) = window.next() {
        if let Event::Input(Input::Move(Motion::MouseScroll(zoom_amount)), _) = event {
            observer.zoom_in_out(zoom_amount[1]);
        };

        if let Event::Input(
            Input::Button(ButtonArgs {
                state: _,
                button: Button::Mouse(MouseButton::Left),
                scancode: _,
            }),
            _,
        ) = event
        {
            println!("click");
        };

        window.draw_2d(&event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            for body in &space.bodies {
                let (x, y) = observer.cast(body.position);

                ellipse(
                    [1.0; 4],
                    [x, y, 10.0, 10.0],
                    context.transform.append_transform(observer.view_transform),
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
                        .append_transform(observer.view_transform),
                    graphics,
                )
                .unwrap();
            }

            text(
                [0.7; 4],
                12,
                format!(
                    "T = {}",
                    space.time.duration_since(the_big_bang_instant).as_secs()
                )
                .as_str(),
                &mut glyphs,
                context.transform.trans(10.0, 10.0),
                graphics,
            )
            .unwrap();

            glyphs.factory.encoder.flush(device);
            space.tick(std::time::Duration::from_secs(3600));
            observer.center_at(space.bodies[0].position);
        });
    }
}
