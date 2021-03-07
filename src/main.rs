mod physics;
use physics::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    println!("Bang!");

    let mut space = physics::Space::default();

    space.bodies.push(Body {
        position: Vector { x: 0.0, y: 0.0 },
        velocity: Vector { x: 0.0, y: 0.0 },
        mass: 100.0,
    });

    space.bodies.push(Body {
        position: Vector { x: 0.0, y: 100.0 },
        velocity: Vector { x: 0.0, y: 0.0 },
        mass: 100.0,
    });

    space.bodies.push(Body {
        position: Vector { x: 300.0, y: 300.0 },
        velocity: Vector { x: 0.0, y: 10.0 },
        mass: 100.0,
    });

    for _ in 0..50{
        println!("{:?}", space);
        space.tick(std::time::Duration::from_secs(1));
    }


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(255, 64, 255));
        for body in &space.bodies {
            //canvas.draw_point((1, 1)).unwrap();
            canvas.draw_point((body.position.x as i32, body.position.y as i32)).unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        println!("{:?}", space);
        space.tick(std::time::Duration::from_secs(1));
    }
}
