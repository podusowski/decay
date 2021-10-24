extern crate piston_window;
use algebra::Vector;
use piston_window::*;

mod algebra;
mod ephemeris;
mod graphics;
mod physics;
mod units;

use physics::*;
use rg3d::core::algebra::{Vector2, Vector3};
use rg3d::core::color::Color;
use rg3d::core::math::Rect;
use rg3d::core::pool::Handle;
use rg3d::engine::framework::{Framework, GameState};
use rg3d::engine::resource_manager::{MaterialSearchOptions, ResourceManager};
use rg3d::scene::base::BaseBuilder;
use rg3d::scene::camera::CameraBuilder;
use rg3d::scene::transform::TransformBuilder;
use rg3d::scene::Scene;

use crate::graphics::{Frame, Observer};
use crate::units::Distance;

fn handle_event(event: &Event, observer: &mut graphics::Observer, space: &mut Space) {
    if let Event::Input(Input::Move(Motion::MouseScroll(zoom_amount)), _) = event {
        observer.zoom_in_out(zoom_amount[1]);
    };

    // Record position of the mouse to know where the click happened later on.
    if let Event::Input(Input::Move(Motion::MouseCursor(cursor)), _) = event {
        observer.mouse_cursor = (cursor[0], cursor[1]);
    }

    // Handle clicks.
    if let Event::Input(
        Input::Button(ButtonArgs {
            state: ButtonState::Press,
            button: Button::Mouse(MouseButton::Left),
            scancode: _,
        }),
        _,
    ) = event
    {
        let position = observer.to_world_coords(observer.mouse_cursor);
        let body = space.body_at(position);
        println!(
            "click {:?}, position: {:?}, body: {:?}",
            observer.mouse_cursor, position, body
        );
        if let Some(body) = body {
            observer.selected_body = body;
        }
    };

    if let Event::Input(
        Input::Button(ButtonArgs {
            state: ButtonState::Press,
            button: Button::Keyboard(Key::Q),
            scancode: _,
        }),
        _,
    ) = event
    {
        observer.ship_wide_zoom();
    };

    if let Event::Input(
        Input::Button(ButtonArgs {
            state: ButtonState::Press,
            button: Button::Keyboard(Key::W),
            scancode: _,
        }),
        _,
    ) = event
    {
        observer.system_wide_zoom();
    };

    if let Event::Input(
        Input::Button(ButtonArgs {
            state: ButtonState::Press,
            button: Button::Keyboard(Key::E),
            scancode: _,
        }),
        _,
    ) = event
    {
        let ship = &mut space.ships[0];
        ship.thrust = ship.thrust
            + Vector {
                x: 0.0,
                y: -10.0,
                z: 0.0,
            };
    };

    if let Event::Input(
        Input::Button(ButtonArgs {
            state: ButtonState::Press,
            button: Button::Keyboard(Key::R),
            scancode: _,
        }),
        _,
    ) = event
    {
        let ship = &mut space.ships[0];
        ship.thrust = Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    };

    if let Event::Loop(Loop::Update(_)) = event {
        space.tick(chrono::Duration::hours(1));
        observer.look_at(space.bodies[observer.selected_body].position);
    }
}

struct Decay {
    space: Space,
    scene: Handle<Scene>,
}

impl GameState for Decay {
    fn init(engine: &mut rg3d::engine::framework::GameEngine) -> Self
    where
        Self: Sized,
    {
        let mut space = Space::solar_system();
        space.ships.push(Ship {
            position: Vector {
                x: Distance::from_aus(1.0).as_meters(),
                y: Distance::from_aus(1.0).as_meters(),
                z: Default::default(),
            },
            velocity: Default::default(),
            thrust: Default::default(),
            name: "Rocinante",
        });

        println!("Space: {:?}", space);

        let scene =
            rg3d::core::futures::executor::block_on(create_scene(&space, &engine.resource_manager));

        Self {
            space: space,
            scene: engine.scenes.add(scene),
        }
    }
}

async fn create_scene(space: &Space, resource_manager: &ResourceManager) -> Scene {
    let mut scene = Scene::new();

    scene.ambient_lighting_color = Color::opaque(200, 200, 200);

    let camera = CameraBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 0.0, -1200.0))
                .build(),
        ),
    )
    .build(&mut scene.graph);

    for body in &space.bodies {
        let planet = resource_manager
            .request_model("data/ball.fbx", MaterialSearchOptions::RecursiveUp)
            .await;

        let planet = planet.unwrap().instantiate_geometry(&mut scene);

        scene.graph[planet]
            .local_transform_mut()
            .set_position(Vector3::new(
                body.position().x as f32,
                body.position().y as f32,
                body.position().z as f32,
            ));
    }

    scene
}

fn main() {
    Framework::<Decay>::new().unwrap().run();
}
