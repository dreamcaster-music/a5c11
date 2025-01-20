mod core;

fn main() {
    match core::terminal_size() {
        Some((width, height)) => {
            println!("Terminal size: {} columns and {} rows", width, height);
        }
        None => {
            println!("Error! Unable to retrieve terminal size...");
        }
    }
}
