use core::rand::vec_range;

use sprites::Firework;

mod core;
mod sprites;

fn main() {
    // initialize terminal
    let handle = core::terminal::init().unwrap();

    let (width, height) = core::terminal::size().unwrap();

    let firework = Firework::new(100, 50);
    let mut vec = vec![firework];

    let mut input = [0u8; 1]; // Buffer for reading one byte at a time
    loop {
        // // // Read one byte from stdin (should be non-blocking and immediate in raw mode)
        // std::io::Read::read_exact(&mut std::io::stdin(), &mut input).expect("Failed to read input");

        // // Check the pressed key and break if it's 'q'
        // if input[0] == b'q' {
        //     println!("\nYou pressed 'q'. Exiting...");
        //     break;
        // }

        // let mut page = Vec::new();
        // let fg = 10;
        // let bg = 0;
        // let mut rng = vec_range::<u8>(40, 100, width * height).into_iter();
        // let perlin = core::rand::Perlin::new();
        // for x in 0..width {
        //     for y in 0..height {
        //         let v = perlin.sample(x as f32 / 4.0, y as f32 / 4.0, 220.0, 255.0) as u8;

        //         let c = rng.next().unwrap() as char;
        //         page.push(core::terminal::Element::new(c, v, 232));
        //         // fg += core::rand_range(0, 2);
        //         // bg -= core::rand_range(0, 2);
        //     }
        // }

        //core::terminal::display_raw(&page).unwrap();
        core::terminal::display(&mut vec).unwrap();

        // Print the raw key that was pressed (no echo in raw mode)
        println!("You pressed: {}", input[0] as char);
    }

    drop(handle);
}
