mod algebra;
mod ephemeris;
mod knowledge;
mod physics;
mod units;

use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_egui::{EguiContext, EguiPlugin};

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

    use chrono::{Utc, DateTime};

    use super::*;

    // TODO: This shouldn't be public
    pub const TIME_SCALE: f64 = 1000000000.;

    pub struct WorldTime {
        initial_time: chrono::DateTime<Utc>,
        time: chrono::DateTime<Utc>,
    }

    impl WorldTime {
        // TODO: Initial time should come externally
        fn new() -> Self {
            Self {
                initial_time: Utc::now(),
                time: Utc::now(),
            }
        }

        pub fn now(&self) -> DateTime<Utc>{
            self.time
        }
    }

    fn world_time(time: Res<Time>, mut world_time: ResMut<WorldTime>) {
        eprintln!("Bevy time: {:?}", time.time_since_startup());
        // TODO: The time is broken.
        let world_duration_since_startup = (time.seconds_since_startup() * TIME_SCALE / 3600.);
        world_time.time = 
            world_time.initial_time
                + chrono::Duration::seconds(world_duration_since_startup as i64)
        ;

        eprintln!("World time: {:?}", world_time.time);
    }

    pub struct WorldTimePlugin;

    impl Plugin for WorldTimePlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(WorldTime::new()).add_system(world_time);
        }
    }
}

fn clock(mut egui_context: ResMut<EguiContext>, world_time: Res<time::WorldTime>) {
    egui::Window::new("Time").show(egui_context.ctx_mut(), |ui| {
        ui.label(world_time.now().format("%d/%m/%Y %H:%M").to_string());
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(time::WorldTimePlugin)
        .add_system(clock)
        .add_startup_system(spawn_camera)
        .add_system(zoom_in_out)
        .add_startup_system(ephemeris::spawn_solar_system)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_system(physics::newtonian_gravity)
        .run();
}
