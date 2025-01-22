use core::{rand::vec_range, shapes};
use std::{thread, time::Duration};

use sprites::Firework;

mod core;
mod sprites;

fn main() {
    // initialize terminal
    // let handle = core::terminal::init().unwrap();

    // let (width, height) = core::terminal::size().unwrap();

    // let firework = Firework::new(100, 50);
    // let mut vec = vec![firework];

    // let mut input = [0u8; 1]; // Buffer for reading one byte at a time
    // loop {
    // // // Read one byte from stdin (should be non-blocking and immediate in raw mode)
    // std::io::Read::read_exact(&mut std::io::stdin(), &mut input).expect("Failed to read input");

    // // Check the pressed key and break if it's 'q'
    // if input[0] == b'q' {
    //     println!("\nYou pressed 'q'. Exiting...");
    //     break;
    // }

    // let mut r = 0;
    // let mut g = 0;
    // let mut b = 0;

    // let mut page = Vec::new();
    // let fg = 10;
    // let bg = 0;
    // let mut rng = vec_range::<u8>(40, 100, width * height).into_iter();
    // let perlin = core::rand::Perlin::new();
    // for x in 0..width {
    //     for y in 0..height {
    //         b += 1;
    //         if b >= 6 {
    //             b = 0;
    //             g += 1;
    //         }
    //         if g >= 6 {
    //             g = 0;
    //             r += 1;
    //         }
    //         if r >= 6 {
    //             r = 0;
    //             b = 0;
    //         }

    // let r = perlin.sample(x as f32 / 4.0, y as f32 / 4.0, 0.0, 255.0) as u8;
    // let g = perlin.sample(x as f32 / 4.0, (y as f32 + 10.0) / 4.0, 0.0, 255.0) as u8;
    // let b = perlin.sample((x as f32 + 10.0) / 4.0, y as f32 / 4.0, 0.0, 255.0) as u8;

    // let c = rng.next().unwrap() as char;
    // page.push(core::terminal::Element::new(c, b, 232));
    // fg += core::rand_range(0, 2);
    // bg -= core::rand_range(0, 2);

    // core::terminal::display_raw(&page).unwrap();
    //core::terminal::display(&mut vec).unwrap();

    // Print the raw key that was pressed (no echo in raw mode)
    // println!("You pressed: {}", input[0] as char);
    // std::thread::sleep(Duration::from_secs(3));

    //roman code:
    // Initialize terminal
    // Initialize terminal
    let handle = core::terminal::init().unwrap();
    let (width, height) = core::terminal::size().unwrap();
    let nice_pink = core::terminal::rgb(255, 182, 193);

    // Create the checkerboard with the terminal width and height
    let checkerboard = shapes::Checkerboard::new(width, height, 15, 0, nice_pink, 0);

    let mut reverse = false;

    loop {
        let mut page = Vec::new();

        // Generate the checkerboard pattern with possible reversal
        let mut elements = checkerboard.generate_elements();
        if reverse {
            // Reverse the colors of the checkerboard
            for element in elements.iter_mut() {
                element.reverse_colors();
            }
        }

        page = elements; // Store the elements in the page buffer

        // Display the elements on the terminal
        core::terminal::display_raw(&page).unwrap();

        // Toggle the reverse flag for the next loop iteration
        reverse = !reverse;

        // Sleep to create animation effect (adjust the duration to control speed)
        thread::sleep(Duration::from_secs(2));
    }

    // Clean up and reset the terminal before exiting
    drop(handle);
}
