use core::{
    rand::{rand, range},
    shapes,
    terminal::display_raw,
};
use sprites::Firework;
use std::{thread, time::Duration};

mod core;
mod sprites;

fn main() {
    let handle = core::terminal::init().unwrap();
    let (width, height) = core::terminal::size().unwrap();

    let mut vec = vec![];

    for _ in 0..3 {
        let x = range(50, 100);
        let y = range(50, 100);
        let r = range(50, 100);
        let g = range(50, 100);
        let b = range(50, 100);

        let firework = Firework::new(x, y);
        vec.push(firework);
    }

    core::terminal::display(&mut vec).unwrap();

    // Sleep to create animation effect (adjust the duration to control speed)
    thread::sleep(Duration::from_secs(2));

    // Clean up and reset the terminal before exiting
    drop(handle);
}
