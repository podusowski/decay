use bevy::prelude::*;

use crate::physics::Body;

#[derive(Component)]
struct Name(String);

pub fn create_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let file = std::fs::File::open("ephemeris.yaml").expect("could not open ephemeris file");
    let bodies: Vec<Body> = serde_yaml::from_reader(file).expect("could not parse ephemeris file");

    for body in bodies {
        commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 30000000000.0,
                    subdivisions: 50,
                })),
                material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                ..default()
            })
            .insert(Body {
                position: body.position.aus_to_meters(),
                velocity: body.velocity.km_per_second_to_meters_per_second(),
                mass: body.mass,
                name: body.name.clone(),
            })
            .insert(Name(body.name.clone()));
    }

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, -2.5, 5000000000000.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}
