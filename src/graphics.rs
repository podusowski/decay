use piston_window::math::translate;
use piston_window::types::Matrix2d;
use piston_window::*;
use piston_window::{text, Context, G2d, Glyphs};

use crate::algebra::{self, Vector};
use crate::physics::{Body, Space};
use crate::units;

pub struct Observer {
    pub view_transform: Matrix2d<f64>,

    /// How many pixels has one astronomical unit.
    au_as_pixels: f64,

    /// Need to track current mouse position as Piston doesn't do that.
    pub mouse_cursor: (f64, f64),

    /// Index of the body the player is looking at.
    pub selected_body: usize,
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
    pub fn zoom_in_out(&mut self, amount: f64) {
        self.au_as_pixels = (self.au_as_pixels + amount).max(1.0);
    }

    pub fn ship_wide_zoom(&mut self) {
        self.au_as_pixels = 10000.0;
    }

    pub fn system_wide_zoom(&mut self) {
        self.au_as_pixels = SYSTEM_WIDE_ZOOM;
    }

    pub fn look_at(&mut self, position: algebra::Vector) {
        let (x, y) = self.to_screen_coords(position);
        self.view_transform = translate([(1280.0 / 2.0) - x, (720.0 / 2.0) - y]);
    }

    pub fn to_screen_coords(&self, position: algebra::Vector) -> (f64, f64) {
        (
            units::Distance::from_meters(position.x).as_au() * self.au_as_pixels,
            units::Distance::from_meters(position.y).as_au() * self.au_as_pixels,
        )
    }

    pub fn to_world_coords(&self, position: (f64, f64)) -> Vector {
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

pub fn draw_body(
    body: &Body,
    observer: &Observer,
    context: &Context,
    graphics: &mut G2d,
    glyphs: &mut Glyphs,
) {
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
        glyphs,
        context
            .transform
            .trans(x + 10.0, y)
            .append_transform(observer.view_transform),
        graphics,
    )
    .unwrap();
}

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
