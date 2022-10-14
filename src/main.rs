mod algebra;
mod ephemeris;
mod knowledge;
mod physics;
mod time;

use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_egui::{EguiContext, EguiPlugin};
use physics::Body;

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

fn bodies_ui(mut egui_context: ResMut<EguiContext>, bodies: Query<(Entity, &Body)>) {
    // TODO: There are allocations everywhere here!

    let (mut selected_entity, selected_body) = bodies.iter().last().unwrap();

    egui::Window::new("Bodies").show(egui_context.ctx_mut(), |ui| {
        egui::ComboBox::from_id_source("selected_body")
            .selected_text(selected_body.name.to_owned())
            .show_ui(ui, |ui| {
                for (entity, body) in bodies.iter() {
                    ui.selectable_value(&mut selected_entity, entity, body.name.to_owned());
                }
            });
    });
}

/// Shows window with the current world time.
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
        .add_system(bodies_ui)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_system(physics::newtonian_gravity)
        .run();
}
