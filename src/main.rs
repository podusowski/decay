extern crate piston_window;
use piston_window::math::translate;
use piston_window::*;

mod algebra;
mod ephemeris;
mod physics;
mod units;

use physics::*;

fn main() {
    let mut space = Space::solar_system();
    println!("Space: {:?}", space);

    let mut window: PistonWindow = WindowSettings::new("decay", [1280, 720])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs = window.load_font("./FiraSans-Regular.ttf").unwrap();

    let view_transform = translate([400.0, 400.0]);
    let mut au_as_pixels = 20.0;
    let the_big_bang_instant = space.time;

    while let Some(event) = window.next() {
        if let Event::Input(Input::Move(Motion::MouseScroll(zoom_amount)), _) = event {
            println!("{:?}", zoom_amount);
            au_as_pixels = (au_as_pixels + zoom_amount[1]).max(1.0);
        };

        window.draw_2d(&event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            for body in &space.bodies {
                let x = units::Distance::from_meters(body.position.x).as_au() * au_as_pixels;
                let y = units::Distance::from_meters(body.position.y).as_au() * au_as_pixels;

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
        });
    }
}
