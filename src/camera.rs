use crate::physics::Body;
use bevy::{input::mouse::MouseWheel, prelude::*};
use std::ops::DerefMut;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, -2.5, 5000000000000.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}

/// Zoom in/out on mouse wheel movement.
pub fn zoom_in_out(
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

pub struct SelectedBody {
    pub entity: Entity,
}

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
