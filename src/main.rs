use algebra::Vector;

mod algebra;
mod ephemeris;
mod graphics;
mod physics;
mod units;

use amethyst::assets::{PrefabLoader, PrefabLoaderSystemDesc, RonFormat};
use amethyst::core::transform::Transform;
use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::renderer::rendy::mesh::{Normal, Position, TexCoord};
use amethyst::renderer::{Camera, RenderShaded3D};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::utils::scene::BasicScenePrefab;
use amethyst::window::ScreenDimensions;
use physics::*;

use crate::graphics::{Frame, Observer};
use crate::units::Distance;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

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

struct State {
    space: Space,
}

impl State {
    fn new() -> State {
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

        State { space: space }
    }
}

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();

        initialise_camera(data.world);

        // Load our sprites and display them
        //let sprites = load_sprites(world);
        //init_sprites(world, &sprites, &dimensions);

        //create_ui_example(world);

        for body in &self.space.bodies[..1] {
            let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
                loader.load("prefab/sphere.ron", RonFormat, ())
            });

            let mut transform = Transform::default();
            transform.set_translation_xyz(
                    (body.position().x + 1000.0) as f32,
                    (body.position().y + 100000.0) as f32,
                    body.position().z as f32,
                );

            println!("{:?}", transform);

            data.world
                .create_entity()
                .with(handle)
                //.with(transform)
                .build();
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    pub const ARENA_HEIGHT: f32 = 600.0;
    pub const ARENA_WIDTH: f32 = 800.0;

    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn main() -> amethyst::Result<()> {
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

    let mut observer = Observer::default();

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display_config.ron");
    //let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        //.with_bundle(
        //    InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        //)?
        //.with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                //.with_plugin(RenderUi::default())
                //.with_plugin(RenderFlat2D::default()),
                .with_plugin(RenderShaded3D::default()),
        )?;

    let mut game = Application::new(resources, State::new(), game_data)?;
    game.run();

    Ok(())
}
