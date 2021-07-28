extern crate piston_window;
use algebra::Vector;
use piston_window::*;

mod algebra;
mod ephemeris;
mod graphics;
mod physics;
mod units;

use physics::*;

use crate::graphics::Observer;
use crate::units::Distance;

fn handle_event(event: &Event, observer: &mut graphics::Observer, space: &mut Space) {
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

    let mut window: PistonWindow = WindowSettings::new("Decay", [1280, 720])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs = window.load_font("./Rajdhani-Light.ttf").unwrap();

    let mut observer = Observer::default();

    while let Some(event) = window.next() {
        handle_event(&event, &mut observer, &mut space);

        window.draw_2d(&event, |context, graphics, device| {
            clear([0.0; 4], graphics);

            for body in &space.bodies {
                graphics::draw_body(body, &observer, &context, graphics, &mut glyphs);
            }

            for ship in &space.ships {
                graphics::draw_ship(ship, &observer, &context, graphics, &mut glyphs);
            }

            graphics::draw_statusbar(&space, &context, graphics, &mut glyphs);

            glyphs.factory.encoder.flush(device);
        });
    }
}
