mod algebra;
mod camera;
mod ephemeris;
mod knowledge;
mod physics;
mod time;

use std::ops::DerefMut;

use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiPlugin};
use camera::SelectedBody;
use physics::Body;

fn bodies_ui(
    mut egui_context: ResMut<EguiContext>,
    mut selected_body: ResMut<Option<camera::SelectedBody>>,
    bodies: Query<(Entity, &Body)>,
) {
    // TODO: There are allocations everywhere here!

    if let Some(ref mut selected_body) = selected_body.deref_mut() {
        // This unwrap will fail only if entity is deleted.
        let (_, body) = bodies.get(selected_body.entity).unwrap();

        egui::Window::new("Bodies")
            .collapsible(false)
            .resizable(false)
            .show(egui_context.ctx_mut(), |ui| {
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
    egui::Window::new("Time")
        .collapsible(false)
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
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
        // Camera and lights.
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_startup_system(camera::spawn_camera)
        .add_system(camera::follow_selected_body)
        .add_system(camera::zoom_in_out)
        .insert_resource(Option::<SelectedBody>::None)
        // User interface.
        .add_system(bodies_ui)
        .add_system(clock)
        // Bodies and movement.
        .add_startup_system(ephemeris::spawn_solar_system)
        .add_system(physics::newtonian_gravity)
        .run();
}
