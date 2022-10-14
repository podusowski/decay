mod algebra;
mod ephemeris;
mod knowledge;
mod physics;
mod time;

use std::ops::DerefMut;

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

mod camera {
    use super::*;

    /// Move camera so it's above selected body, but keep original Z element.
    #[allow(clippy::type_complexity)]
    pub fn follow_selected_body(
        mut selected_body: ResMut<Option<SelectedBody>>,
        mut query: ParamSet<(
            Query<&mut Transform, With<Camera3d>>,
            Query<&Transform, With<Body>>,
        )>,
    ) {
        if let Some(ref mut selected_body) = selected_body.deref_mut() {
            let body_translation = {
                let p1 = query.p1();
                // This unwrap will fail only if entity is deleted.
                let body = p1.get(selected_body.entity).unwrap();
                body.translation
            };

            let mut p0 = query.p0();
            // Single camera is expected.
            let camera_translation = &mut p0.single_mut().translation;
            let camera_z = camera_translation.z;
            *camera_translation = body_translation;
            camera_translation.z = camera_z;
        }
    }
}

pub struct SelectedBody {
    entity: Entity,
}

fn bodies_ui(
    mut egui_context: ResMut<EguiContext>,
    mut selected_body: ResMut<Option<SelectedBody>>,
    bodies: Query<(Entity, &Body)>,
) {
    // TODO: There are allocations everywhere here!

    if let Some(ref mut selected_body) = selected_body.deref_mut() {
        // This unwrap will fail only if entity is deleted.
        let (_, body) = bodies.get(selected_body.entity).unwrap();

        egui::Window::new("Bodies").show(egui_context.ctx_mut(), |ui| {
            egui::ComboBox::from_id_source("selected_body")
                .selected_text(body.name.to_owned())
                .show_ui(ui, |ui| {
                    for (entity, body) in bodies.iter() {
                        ui.selectable_value(
                            &mut selected_body.entity,
                            entity,
                            body.name.to_owned(),
                        );
                    }
                });
        });
    }
}

/// Shows window with the current world time.
fn clock(mut egui_context: ResMut<EguiContext>, world_time: Res<time::WorldTime>) {
    egui::Window::new("Time").show(egui_context.ctx_mut(), |ui| {
        ui.label(world_time.now().format("%d/%m/%Y %H:%M").to_string());
    });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Solar System".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(time::WorldTimePlugin)
        .add_system(clock)
        .add_startup_system(spawn_camera)
        .add_system(camera::follow_selected_body)
        .add_system(zoom_in_out)
        .insert_resource(Option::<SelectedBody>::None)
        .add_startup_system(ephemeris::spawn_solar_system)
        // Body select
        .add_system(bodies_ui)
        // Others
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_system(physics::newtonian_gravity)
        .run();
}
