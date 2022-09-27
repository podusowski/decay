use bevy::prelude::*;
use chrono::{Duration, Utc};

use crate::{
    algebra::Vector,
    physical_properties::mass_of,
    physics::{self, Body},
};

async fn fetch_body(body: &rhorizons::MajorBody) -> Body {
    info!("Getting info about '{}'", body.name);

    // TODO: These should be constructed only once.
    let start_time = Utc::now() - Duration::days(1);
    let stop_time = Utc::now();

    let vectors = rhorizons::ephemeris(body.id, start_time, stop_time).await;
    Body {
        name: body.name.clone(), // TODO: Try getting rid of it.
        mass: physics::Mass::new::<physics::kilogram>(
            mass_of(&body.name).expect(&format!("no mass for '{}'", body.name)),
        ),
        position: Vector {
            x: vectors[0].position[0] as f64,
            y: vectors[0].position[1] as f64,
            z: vectors[0].position[2] as f64,
        },
        velocity: Vector {
            x: vectors[0].velocity[0] as f64,
            y: vectors[0].velocity[1] as f64,
            z: vectors[0].velocity[2] as f64,
        },
    }
}

fn fetch_ephemeris() -> Vec<Body> {
    // `rhorizons` crate is asynchronous, but Bevy isn't.
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            info!("Getting Solar System bodies from NASA JPL Horizons.");
            // TODO: unuglyfy this!
            let bodies = rhorizons::major_bodies().await;
            let major_bodies = bodies
                .iter()
                .filter(|body| ["Sun", "Mercury", "Venus", "Earth"].contains(&body.name.as_str()));

            let mut bodies = Vec::new();
            for major_body in major_bodies {
                bodies.push(fetch_body(major_body).await);
            }
            bodies
        })
}

fn ensure_ephemeris() {
    for body in fetch_ephemeris() {
        eprintln!("{:?}", body);
    }
}

#[derive(Component)]
struct Name(String);

pub fn create_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //let file = std::fs::File::open("ephemeris.yaml").expect("could not open ephemeris file");
    //let bodies: Vec<Body> = serde_yaml::from_reader(file).expect("could not parse ephemeris file");
    let bodies = fetch_ephemeris();

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
