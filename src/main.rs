use std::sync::{Arc, Mutex, RwLock};

use algebra::Vector;

mod algebra;
mod ephemeris;
mod frameworks;
mod physics;
mod units;

use frameworks::{Framework, GameState};
use physics::*;
use rg3d::core::algebra::{Matrix4, Vector2, Vector3};
use rg3d::core::color::Color;
use rg3d::core::instant::Instant;
use rg3d::core::math::Rect;
use rg3d::core::pool::Handle;
use rg3d::engine::error::EngineError;
use rg3d::engine::framework::{GameEngine, UiNode};
use rg3d::engine::resource_manager::{MaterialSearchOptions, ResourceManager};
use rg3d::engine::Engine;
use rg3d::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use rg3d::event_loop::{ControlFlow, EventLoop};
use rg3d::gui::message::{MessageDirection, TextMessage};
use rg3d::gui::node::StubNode;
use rg3d::gui::text::TextBuilder;
use rg3d::gui::widget::WidgetBuilder;
use rg3d::gui::UserInterface;
use rg3d::material::shader::SamplerFallback;
use rg3d::material::{Material, PropertyValue};
use rg3d::resource::texture::Texture;
use rg3d::scene::base::BaseBuilder;
use rg3d::scene::camera::CameraBuilder;
use rg3d::scene::mesh::surface::{SurfaceBuilder, SurfaceData};
use rg3d::scene::mesh::{MeshBuilder, RenderPath};
use rg3d::scene::node::Node;
use rg3d::scene::transform::TransformBuilder;
use rg3d::scene::Scene;
use rg3d::utils::log::{Log, MessageKind};
use rg3d::utils::translate_event;
use rg3d::window::WindowBuilder;

use crate::units::Distance;

//fn handle_event(event: &Event, observer: &mut graphics::Observer, space: &mut Space) {
//    if let Event::Input(Input::Move(Motion::MouseScroll(zoom_amount)), _) = event {
//        observer.zoom_in_out(zoom_amount[1]);
//    };
//
//    // Record position of the mouse to know where the click happened later on.
//    if let Event::Input(Input::Move(Motion::MouseCursor(cursor)), _) = event {
//        observer.mouse_cursor = (cursor[0], cursor[1]);
//    }
//
//    // Handle clicks.
//    if let Event::Input(
//        Input::Button(ButtonArgs {
//            state: ButtonState::Press,
//            button: Button::Mouse(MouseButton::Left),
//            scancode: _,
//        }),
//        _,
//    ) = event
//    {
//        let position = observer.to_world_coords(observer.mouse_cursor);
//        let body = space.body_at(position);
//        println!(
//            "click {:?}, position: {:?}, body: {:?}",
//            observer.mouse_cursor, position, body
//        );
//        if let Some(body) = body {
//            observer.selected_body = body;
//        }
//    };
//
//    if let Event::Input(
//        Input::Button(ButtonArgs {
//            state: ButtonState::Press,
//            button: Button::Keyboard(Key::Q),
//            scancode: _,
//        }),
//        _,
//    ) = event
//    {
//        observer.ship_wide_zoom();
//    };
//
//    if let Event::Input(
//        Input::Button(ButtonArgs {
//            state: ButtonState::Press,
//            button: Button::Keyboard(Key::W),
//            scancode: _,
//        }),
//        _,
//    ) = event
//    {
//        observer.system_wide_zoom();
//    };
//
//    if let Event::Input(
//        Input::Button(ButtonArgs {
//            state: ButtonState::Press,
//            button: Button::Keyboard(Key::E),
//            scancode: _,
//        }),
//        _,
//    ) = event
//    {
//        let ship = &mut space.ships[0];
//        ship.thrust = ship.thrust
//            + Vector {
//                x: 0.0,
//                y: -10.0,
//                z: 0.0,
//            };
//    };
//
//    if let Event::Input(
//        Input::Button(ButtonArgs {
//            state: ButtonState::Press,
//            button: Button::Keyboard(Key::R),
//            scancode: _,
//        }),
//        _,
//    ) = event
//    {
//        let ship = &mut space.ships[0];
//        ship.thrust = Vector {
//            x: 0.0,
//            y: 0.0,
//            z: 0.0,
//        };
//    };
//
//    if let Event::Loop(Loop::Update(_)) = event {
//        space.tick(chrono::Duration::hours(1));
//        observer.look_at(space.bodies[observer.selected_body].position);
//    }
//}

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

struct VisualObject {
    node: Handle<Node>,
    label: Label,
}

/// Label of a body. For example, name of a planet.
struct Label {
    ui: UserInterface<(), StubNode>,
    render_target: rg3d::resource::texture::Texture,
    node: Handle<UiNode>,
}

impl Label {
    fn new(text: &str) -> Self {
        let (width, height) = (100, 100);
        let mut ui = UserInterface::<(), StubNode>::new(Vector2::new(width as f32, height as f32));
        let mut ctx = ui.build_ctx();
        let node = TextBuilder::new(WidgetBuilder::new())
            .with_text(text)
            .build(&mut ctx);
        Label {
            ui,
            render_target: Texture::new_render_target(width, height),
            node,
        }
    }

    fn render(&mut self, engine: &mut frameworks::GameEngine) {
        engine
            .renderer
            .render_ui_to_texture(self.render_target.clone(), &mut self.ui)
            .unwrap();
    }
}

/// Main bucket holding top-level game systems like physics and graphics engines.
struct Decay {
    space: Space<VisualObject>,
    scene: Handle<Scene>,
    camera: Handle<Node>,
    zooming: Option<Zooming>,
}

impl GameState for Decay {
    fn init(engine: &mut rg3d::engine::framework::GameEngine) -> Self
    where
        Self: Sized,
    {
        let (space, scene, camera) =
            rg3d::core::futures::executor::block_on(create_scene(&engine.resource_manager));

        Self {
            space: space,
            scene: engine.scenes.add(scene),
            camera: camera,
            zooming: None,
        }
    }

    fn on_tick(&mut self, engine: &mut GameEngine, _dt: f32, _: &mut ControlFlow) {
        let scene = &mut engine.scenes[self.scene];

        self.space.tick(chrono::Duration::hours(1), |body| {
            scene.graph[body.user_data.node]
                .local_transform_mut()
                .set_position(Vector3::new(
                    Distance::from_meters(body.position().x).as_au() as f32,
                    Distance::from_meters(body.position().y).as_au() as f32,
                    Distance::from_meters(body.position().z).as_au() as f32,
                ));
            println!("{:?}", body.position);
        });

        if let Some(zooming) = &self.zooming {
            scene.graph[self.camera]
                .local_transform_mut()
                .offset(Vector3::new(0.0, 0.0, zooming.multiplier()));
        }

        for body in &mut self.space.bodies {
            body.user_data.label.ui.send_message(TextMessage::text(
                body.user_data.label.node,
                MessageDirection::ToWidget,
                body.name.to_string(),
            ));
            body.user_data
                .label
                .ui
                .update(Vector2::new(100.0, 100.0), _dt);
            body.user_data.label.render(engine);

            while let Some(_) = body.user_data.label.ui.poll_message() {}
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

pub fn create_display_material(display_texture: Texture) -> Arc<Mutex<Material>> {
    let mut material = Material::standard();

    material
        .set_property(
            "diffuseTexture",
            PropertyValue::Sampler {
                value: Some(display_texture),
                fallback: SamplerFallback::White,
            },
        )
        .unwrap();

    Arc::new(Mutex::new(material))
}

async fn create_scene(
    resource_manager: &ResourceManager,
) -> (Space<VisualObject>, Scene, Handle<Node>) {
    let mut scene = Scene::new();

    scene.ambient_lighting_color = Color::opaque(200, 200, 200);

    let camera = CameraBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 0.0, -30.0))
                .build(),
        ),
    )
    .build(&mut scene.graph);

    let planet = resource_manager
        .request_model("data/ball.fbx", MaterialSearchOptions::RecursiveUp)
        .await;
    let planet = planet.unwrap();

    let create_graphic_object = || -> VisualObject {
        let scale = 0.001;
        let planet = planet.instantiate_geometry(&mut scene);
        scene.graph[planet]
            .local_transform_mut()
            .set_scale(Vector3::new(scale, scale, scale));

        let label = Label::new("");

        let label_node = MeshBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                    .with_local_position(Vector3::new(10.0, 100.0, -10.0))
                    .build(),
            ),
        )
        .with_surfaces(vec![SurfaceBuilder::new(Arc::new(RwLock::new(
            SurfaceData::make_quad(&Matrix4::new_scaling(1000.0)),
        )))
        .with_material(create_display_material(label.render_target.clone()))
        .build()])
        .with_cast_shadows(false)
        .with_render_path(RenderPath::Forward)
        .build(&mut scene.graph);

        scene.graph.link_nodes(label_node, planet);

        VisualObject {
            node: planet,
            label: label,
        }
    };

    let mut space = Space::solar_system(create_graphic_object);

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

    (space, scene, camera)
}

fn main() {
    Framework::<Decay>::new().unwrap().run();
}
