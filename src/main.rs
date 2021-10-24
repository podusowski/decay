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
use rg3d::engine::framework::{Framework, GameEngine, GameState};
use rg3d::engine::resource_manager::{MaterialSearchOptions, ResourceManager};
use rg3d::engine::Engine;
use rg3d::event::{ElementState, VirtualKeyCode, WindowEvent};
use rg3d::event_loop::ControlFlow;
use rg3d::scene::base::BaseBuilder;
use rg3d::scene::camera::CameraBuilder;
use rg3d::scene::node::Node;
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

enum Zooming {
    In,
    Out,
}

impl Zooming {
    fn multiplier(&self) -> f32 {
        let amount = 1.0;
        match self {
            Self::In => amount,
            Self::Out => -amount,
        }
    }
}

struct Decay {
    space: Space,
    scene: Handle<Scene>,
    camera: Handle<Node>,
    zooming: Option<Zooming>,
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

        let (scene, camera) =
            rg3d::core::futures::executor::block_on(create_scene(&space, &engine.resource_manager));

        Self {
            space: space,
            scene: engine.scenes.add(scene),
            camera: camera,
            zooming: None,
        }
    }

    fn on_tick(&mut self, engine: &mut GameEngine, _dt: f32, _: &mut ControlFlow) {
        let scene = &mut engine.scenes[self.scene];

        if let Some(zooming) = &self.zooming {
            scene.graph[self.camera]
                .local_transform_mut()
                .offset(Vector3::new(0.0, 0.0, zooming.multiplier()));
        }
    }

    fn on_window_event(&mut self, _engine: &mut GameEngine, event: WindowEvent) {
        if let WindowEvent::KeyboardInput { input, .. } = event {
            self.zooming = match (input.state, input.virtual_keycode) {
                (ElementState::Pressed, Some(VirtualKeyCode::W)) => Some(Zooming::In),
                (ElementState::Pressed, Some(VirtualKeyCode::S)) => Some(Zooming::Out),
                _ => None,
            }
        }
    }
}

async fn create_scene(space: &Space, resource_manager: &ResourceManager) -> (Scene, Handle<Node>) {
    let mut scene = Scene::new();

    scene.ambient_lighting_color = Color::opaque(200, 200, 200);

    let camera = CameraBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 0.0, -50.0))
                .build(),
        ),
    )
    .build(&mut scene.graph);

    let planet = resource_manager
        .request_model("data/ball.fbx", MaterialSearchOptions::RecursiveUp)
        .await;
    let planet = planet.unwrap();

    for body in &space.bodies {
        let scale = 0.005;
        let planet = planet.instantiate_geometry(&mut scene);
        scene.graph[planet]
            .local_transform_mut()
            .set_position(Vector3::new(
                Distance::from_meters(body.position().x).as_au() as f32,
                Distance::from_meters(body.position().y).as_au() as f32,
                Distance::from_meters(body.position().z).as_au() as f32,
            ))
            .set_scale(Vector3::new(scale, scale, scale));
    }

    (scene, camera)
}

fn main() {
    Framework::<Decay>::new().unwrap().run();
}
