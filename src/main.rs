mod algebra;
mod ephemeris;
mod knowledge;
mod physics;
mod units;

use bevy::{input::mouse::MouseWheel, prelude::*};

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, -2.5, 5000000000000.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}

fn zoom_in_out(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
) {
    let min_z = 1000000000000.;
    for ev in mouse_wheel.iter() {
        for mut transform in cameras.iter_mut() {
            transform.translation += Vec3::new(0., 0., -1000000000000. * ev.y);
            if transform.translation.z < min_z {
                transform.translation.z = min_z;
            }
        }
    }
}

mod time {
    use std::time::Instant;

    use super::*;

    // TODO: This shouldn't be public
    pub const TIME_SCALE: f64 = 1000000000.;

    struct WorldTime {
        initial_time: std::time::Instant,
        time: Option<std::time::Instant>,
    }

    impl WorldTime {
        // TODO: Initial time should come externally
        fn new() -> Self {
            Self {
                initial_time: std::time::Instant::now(),
                time: None,
            }
        }

        fn now(&self) -> Instant {
            self.time.unwrap()
        }
    }

    fn world_time(time: Res<Time>, mut world_time: ResMut<WorldTime>) {
        eprintln!("Bevy time: {:?}", time.time_since_startup());
        let world_duration_since_startup = time.time_since_startup() * TIME_SCALE as u32;
        world_time.time = Some(world_time.initial_time + world_duration_since_startup);

        eprintln!("World time: {:?}", world_time.time);
    }

    pub struct WorldTimePlugin;

    impl Plugin for WorldTimePlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(WorldTime::new()).add_system(world_time);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(time::WorldTimePlugin)
        .add_startup_system(spawn_camera)
        .add_system(zoom_in_out)
        .add_startup_system(ephemeris::spawn_solar_system)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_system(physics::newtonian_gravity)
        .run();
}
