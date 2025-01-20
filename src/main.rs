mod core;

fn main() {
    // initialize terminal
    let _ = core::terminal::init().unwrap();

    let (width, height) = core::terminal::size().unwrap();

    let mut page = Vec::new();
    for _ in 0..width {
        for _ in 0..height {
            let c = core::rand::<u8>().clamp(40, 100) as char;
            let fg = core::rand();
            let bg = core::rand();
            page.push(core::terminal::Element::new(c, fg, bg));
        }
    }

    core::terminal::display(&page).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(3));
}
