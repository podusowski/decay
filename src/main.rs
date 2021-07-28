extern crate piston_window;
use algebra::Vector;
use piston_window::*;
use piston_window::{math::translate, types::Matrix2d};

mod algebra;
mod ephemeris;
mod physics;
mod units;

use physics::*;

use crate::units::Distance;

struct Observer {
    view_transform: Matrix2d<f64>,

    /// How many pixels has one astronomical unit.
    au_as_pixels: f64,

    // Need to track current mouse position as Piston doesn't do that.
    mouse_cursor: (f64, f64),

    // Index of the body the player is looking at.
    selected_body: usize,
}

const SYSTEM_WIDE_ZOOM: f64 = 20.0;

impl Default for Observer {
    fn default() -> Self {
        Observer {
            view_transform: Default::default(),
            au_as_pixels: SYSTEM_WIDE_ZOOM,
            mouse_cursor: Default::default(),
            selected_body: 0,
        }
    }
}

impl Observer {
    /// Zoom in or out, depending on `amount` sign.
    fn zoom_in_out(&mut self, amount: f64) {
        self.au_as_pixels = (self.au_as_pixels + amount).max(1.0);
    }

    fn ship_wide_zoom(&mut self) {
        self.au_as_pixels = 10000.0;
    }

    fn system_wide_zoom(&mut self) {
        self.au_as_pixels = SYSTEM_WIDE_ZOOM;
    }

    fn look_at(&mut self, position: algebra::Vector) {
        let (x, y) = self.to_screen_coords(position);
        self.view_transform = translate([(1280.0 / 2.0) - x, (720.0 / 2.0) - y]);
    }

    fn to_screen_coords(&self, position: algebra::Vector) -> (f64, f64) {
        (
            units::Distance::from_meters(position.x).as_au() * self.au_as_pixels,
            units::Distance::from_meters(position.y).as_au() * self.au_as_pixels,
        )
    }

    fn to_world_coords(&self, position: (f64, f64)) -> Vector {
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

fn handle_event(event: &Event, observer: &mut Observer, space: &mut Space) {
    if let Event::Input(Input::Move(Motion::MouseScroll(zoom_amount)), _) = event {
        observer.zoom_in_out(zoom_amount[1]);
    };

    // Record position of the mouse to know where the click happened later on.
    if let Event::Input(Input::Move(Motion::MouseCursor(cursor)), _) = event {
        observer.mouse_cursor = (cursor[0], cursor[1]);
    }

    // Handle clicks.
    if let Event::Input(
        Input::Button(ButtonArgs {
            state: ButtonState::Press,
            button: Button::Mouse(MouseButton::Left),
            scancode: _,
        }),
        _,
    ) = event
    {
        let position = observer.to_world_coords(observer.mouse_cursor);
        let body = space.body_at(position);
        println!(
            "click {:?}, position: {:?}, body: {:?}",
            observer.mouse_cursor, position, body
        );
        if let Some(body) = body {
            observer.selected_body = body;
        }
    };

    if let Event::Input(
        Input::Button(ButtonArgs {
            state: ButtonState::Press,
            button: Button::Keyboard(Key::Q),
            scancode: _,
        }),
        _,
    ) = event
    {
        observer.ship_wide_zoom();
    };

    if let Event::Input(
        Input::Button(ButtonArgs {
            state: ButtonState::Press,
            button: Button::Keyboard(Key::W),
            scancode: _,
        }),
        _,
    ) = event
    {
        observer.system_wide_zoom();
    };

    if let Event::Input(
        Input::Button(ButtonArgs {
            state: ButtonState::Press,
            button: Button::Keyboard(Key::E),
            scancode: _,
        }),
        _,
    ) = event
    {
        let ship = &mut space.ships[0];
        ship.thrust = ship.thrust
            + Vector {
                x: 0.0,
                y: -10.0,
                z: 0.0,
            };
    };

    if let Event::Input(
        Input::Button(ButtonArgs {
            state: ButtonState::Press,
            button: Button::Keyboard(Key::R),
            scancode: _,
        }),
        _,
    ) = event
    {
        let ship = &mut space.ships[0];
        ship.thrust = Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    };

    if let Event::Loop(Loop::Update(_)) = event {
        space.tick(chrono::Duration::hours(1));
        observer.look_at(space.bodies[observer.selected_body].position);
    }
}

fn main() {
    let mut space = Space::solar_system();

    space.ships.push(Ship {
        position: Vector {
            x: Distance::from_aus(1.0).as_meters(),
            y: Distance::from_aus(1.0).as_meters(),
            z: Default::default(),
        },
        velocity: Default::default(),
        thrust: Default::default(),
        name: "Rocinante",
    });

    println!("Space: {:?}", space);

    let mut window: PistonWindow = WindowSettings::new("decay", [1280, 720])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs = window.load_font("./FiraSans-Regular.ttf").unwrap();

    let mut observer = Observer::default();

    while let Some(event) = window.next() {
        handle_event(&event, &mut observer, &mut space);

        window.draw_2d(&event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            for body in &space.bodies {
                let (x, y) = observer.to_screen_coords(body.position);

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

            // Draw ships.
            for ship in &space.ships {
                let (x, y) = observer.to_screen_coords(ship.position);

                ellipse(
                    [1.0; 4],
                    [x, y, 10.0, 10.0],
                    context.transform.append_transform(observer.view_transform),
                    graphics,
                );

                text(
                    [0.7; 4],
                    10,
                    ship.name,
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

            let ship = &space.ships[0];
            text(
                [0.7; 4],
                12,
                format!("{} thrust: {:?}", ship.name, ship.thrust).as_str(),
                &mut glyphs,
                context.transform.trans(300.0, 10.0),
                graphics,
            )
            .unwrap();

            glyphs.factory.encoder.flush(device);
        });
    }
}
