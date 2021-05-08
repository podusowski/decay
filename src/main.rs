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
        self.view_transform = translate([(1280.0 / 2.0) - x, (720.0 / 2.0) - y]);
    }

    fn cast(&self, position: algebra::Vector) -> (f64, f64) {
        (
            units::Distance::from_meters(position.x).as_au() * self.au_as_pixels,
            units::Distance::from_meters(position.y).as_au() * self.au_as_pixels,
        )
    }

    fn reverse_cast(&self, position: (f64, f64)) -> Vector {
        println!("{:?}", self.view_transform);
        let x_offset = self.view_transform[0][2];
        let y_offset = self.view_transform[1][2];
        Vector {
            x: units::Distance::from_aus((position.0 - x_offset) / self.au_as_pixels).as_meters(),
            y: units::Distance::from_aus((position.1 - y_offset) / self.au_as_pixels).as_meters(),
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
    let mut selected_body: usize = 0;
    println!("Space: {:?}", space);

    let mut window: PistonWindow = WindowSettings::new("decay", [1280, 720])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs = window.load_font("./FiraSans-Regular.ttf").unwrap();

    let mut observer = Observer::default();
    let the_big_bang_instant = space.time;
    let mut mouse_cursor = (0.0, 0.0);

    while let Some(event) = window.next() {
        if let Event::Input(Input::Move(Motion::MouseScroll(zoom_amount)), _) = event {
            observer.zoom_in_out(zoom_amount[1]);
        };

        if let Event::Input(Input::Move(Motion::MouseCursor(cursor)), _) = event {
            mouse_cursor = (cursor[0], cursor[1]);
        }

        if let Event::Input(
            Input::Button(ButtonArgs {
                state: ButtonState::Press,
                button: Button::Mouse(MouseButton::Left),
                scancode: _,
            }),
            _,
        ) = event
        {
            let position = observer.reverse_cast(mouse_cursor);
            let body = space.body_at(position);
            println!(
                "click {:?}, position: {:?}, body: {:?}",
                mouse_cursor, position, body
            );
            if let Some(body) = body {
                selected_body = body;
            }
        };

        if let Event::Loop(Loop::Update(_)) = event {
            space.tick(chrono::Duration::hours(1));
            observer.center_at(space.bodies[selected_body].position);
        }

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
                format!("T = {}", space.time).as_str(),
                &mut glyphs,
                context.transform.trans(10.0, 10.0),
                graphics,
            )
            .unwrap();

            glyphs.factory.encoder.flush(device);
        });
    }
}
