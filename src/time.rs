use bevy::{prelude::*, time::Time};
use chrono::{DateTime, Utc};

// TODO: This shouldn't be public
pub const TIME_SCALE: f64 = 1000000000.;

pub struct WorldTime {
    initial_time: chrono::DateTime<Utc>,
    time: chrono::DateTime<Utc>,
}

impl WorldTime {
    fn new() -> Self {
        // TODO: Eventually initial time should come as a parameter.
        let now = Utc::now();
        Self {
            initial_time: now,
            time: now,
        }
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.time
    }
}

fn world_time(time: Res<Time>, mut world_time: ResMut<WorldTime>) {
    let world_duration_since_startup = time.seconds_since_startup() * TIME_SCALE / 1000.;
    world_time.time =
        world_time.initial_time + chrono::Duration::seconds(world_duration_since_startup as i64);
}

pub struct WorldTimePlugin;

impl Plugin for WorldTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldTime::new()).add_system(world_time);
    }
}
