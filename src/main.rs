mod physics;

fn main() {
    println!("Bang!");
    let mut space = physics::Space::default();
    loop {
        space.tick(std::time::Duration::from_secs(1));
    }
}
