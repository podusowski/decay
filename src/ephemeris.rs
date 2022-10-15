use bevy::prelude::*;
use chrono::{Duration, Utc};

use crate::{
    algebra::Vector,
    knowledge::{self, mass_of},
    physics::{self, Body},
    SelectedBody,
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
            mass_of(&body.name).unwrap_or_else(|| panic!("no mass for '{}'", body.name)),
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
            let major_bodies = bodies.iter().filter(|body| {
                [
                    "Sun", "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus",
                    "Neptune",
                ]
                .contains(&body.name.as_str())
            });

            let mut bodies = Vec::new();
            for major_body in major_bodies {
                bodies.push(fetch_body(major_body).await);
            }
            bodies
        })
}

mod cache {
    use super::*;

    type Error = Box<dyn std::error::Error>;
    const PATH: &str = "ephemeris.yaml";

    pub(super) fn read() -> Result<Vec<Body>, Error> {
        let f = std::fs::File::open(PATH)?;
        Ok(serde_yaml::from_reader(f)?)
    }

    pub(super) fn write(ephemeris: &Vec<Body>) -> Result<(), Error> {
        let f = std::fs::File::create(PATH)?;
        serde_yaml::to_writer(f, ephemeris)?;
        Ok(())
    }
}

#[derive(Component)]
struct Name(String);

pub fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut selected_body: ResMut<Option<SelectedBody>>,
) {
    let bodies = cache::read().unwrap_or_else(|_| fetch_ephemeris());
    let _ = cache::write(&bodies);

    info!("State of the world:");

    for body in bodies {
        info!("{:?}", body);
        let id = commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: knowledge::about(&body.name)
                        .expect("knowledge of this body")
                        .radius
                        .map_or(10000000000., |r| r * 45_000.),
                    subdivisions: 50,
                })),
                material: materials.add(
                    knowledge::about(&body.name)
                        .expect("knowledge of this body")
                        .color
                        .into(),
                ),
                ..default()
            })
            .insert(Body {
                position: body.position * 1000.,
                velocity: body.velocity,
                mass: body.mass,
                name: body.name.clone(),
            })
            .insert(Name(body.name.clone()))
            .id();

        // Select Sun by default.
        if body.name == "Sun" {
            *selected_body = Some(SelectedBody { entity: id });
        }
    }
}
