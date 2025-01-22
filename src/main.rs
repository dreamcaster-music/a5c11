use core::rand::vec_range;

use sprites::Firework;

mod core;
mod sprites;

fn main() {
    // initialize terminal
    let handle = core::terminal::init().unwrap();

    let (width, height) = core::terminal::size().unwrap();

    let firework = Firework::new();
    let mut vec = vec![firework];
    loop {
        //core::terminal::display_raw(&page).unwrap();
        core::terminal::display(&mut vec).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(30));
    }

    drop(handle);
}
