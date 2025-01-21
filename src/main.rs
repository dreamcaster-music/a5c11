use core::rand_vec_range;

mod core;

fn main() {
    // initialize terminal
    let handle = core::terminal::init().unwrap();

    let (width, height) = core::terminal::size().unwrap();

    let mut input = [0u8; 1]; // Buffer for reading one byte at a time
    loop {
        // Read one byte from stdin (should be non-blocking and immediate in raw mode)
        std::io::Read::read_exact(&mut std::io::stdin(), &mut input).expect("Failed to read input");

        // Check the pressed key and break if it's 'q'
        if input[0] == b'q' {
            println!("\nYou pressed 'q'. Exiting...");
            break;
        }

        let mut page = Vec::new();
        let fg = 10;
        let bg = 0;
        let mut rng = rand_vec_range::<u8>(40, 100, width * height * 2).into_iter();
        for _ in 0..width {
            for _ in 0..height {
                let c = rng.next().unwrap() as char;
                page.push(core::terminal::Element::new(c, fg, bg));
                // fg += core::rand_range(0, 2);
                // bg -= core::rand_range(0, 2);
            }
        }

        core::terminal::display(&page).unwrap();

        // Print the raw key that was pressed (no echo in raw mode)
        println!("You pressed: {}", input[0] as char);
    }

    drop(handle);
}
