mod core;

fn main() {
    // initialize terminal
    let _ = core::terminal::init().unwrap();

    let (width, height) = core::terminal::size().unwrap();

    let mut page = Vec::new();
    let mut fg = 0;
    let mut bg = 0;
    for _ in 0..width {
        for _ in 0..height {
            let c = core::rand_range::<u8>(40, 100) as char;
            page.push(core::terminal::Element::new(c, fg, bg));
            fg += core::rand_range(0, 2);
            bg -= core::rand_range(0, 2);
        }
    }

    core::terminal::display(&page).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(3));
}
