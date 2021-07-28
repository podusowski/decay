use piston_window::*;
use piston_window::{text, Context, G2d, Glyphs};

use crate::physics::Space;

pub fn draw_statusbar(space: &Space, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
    text(
        [0.7; 4],
        16,
        format!("{}", space.time).as_str(),
        glyphs,
        context.transform.trans(10.0, 20.0),
        graphics,
    )
    .unwrap();

    let ship = &space.ships[0];
    text(
        [0.7; 4],
        12,
        format!("{} thrust: {:?}", ship.name, ship.thrust).as_str(),
        glyphs,
        context.transform.trans(300.0, 20.0),
        graphics,
    )
    .unwrap();
}
