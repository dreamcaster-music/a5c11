use core::terminal::rgb;
use std::{thread::sleep, time::Duration};

use sprites::Checkerboard;

mod core;
mod sprites;

fn main() {
    let handle = core::terminal::init().unwrap();
    let (width, height) = core::terminal::size().unwrap();

    let mut vec = vec![];
    let checkerboard = Checkerboard::new(
        width,
        height,
        rgb(255, 0, 0),
        rgb(5, 5, 5),
        rgb(5, 5, 5),
        rgb(255, 255, 255),
    );
    vec.push(checkerboard);

    loop {
        // // Read one byte from stdin (should be non-blocking and immediate in raw mode)
        // std::io::Read::read_exact(&mut std::io::stdin(), &mut input).expect("Failed to read input");

        // // Check the pressed key and break if it's 'q'
        // if input[0] == b'q' {
        //     println!("\nYou pressed 'q'. Exiting...");
        //     break;
        // }

        // if input[0] == b'f' {
        //     println!("\nYou pressed 'f'. Fireworking...");

        // let x = range(0, width);
        // let y = range(0, height);
        // let r = range(0, 5);
        // let g = range(0, 5);
        // let b = range(0, 5);

        // let firework = Firework::new(x, y, r, g, b);
        // vec.push(firework);
        // }

        // core::terminal::display_raw(&page).unwrap();
        core::terminal::display(&mut vec).unwrap();
        sleep(Duration::from_millis(500));

        // Print the raw key that was pressed (no echo in raw mode)
        // println!("You pressed: {}", input[0] as char);
    }

    drop(handle);
}
