mod core;

fn main() {
    let _ = core::terminal::init().unwrap();

    match core::terminal::size() {
        Some((width, height)) => {
            println!("Terminal size: {} columns and {} rows", width, height);
        }
        None => {
            println!("Error! Unable to retrieve terminal size...");
        }
    }

    std::thread::sleep(std::time::Duration::from_secs(3));
}
