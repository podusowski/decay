use std::sync::{Arc, Mutex, RwLock};

use algebra::Vector;

mod algebra;
mod ephemeris;
mod physics;
mod units;

use physics::*;

use crate::units::Distance;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
