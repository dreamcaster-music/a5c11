use std::mem;

#[cfg(unix)]
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

/// Returns the size of the terminal window.
///
/// More specifically, this will return the width and height of the terminal
/// window measured in how many characters fit inside the console.
/// Returns ```None``` if it is unable to obtain the width and height.
///
/// # Example
/// ```
/// let (width, height) = terminal_size().expect("Failed to get the size of the terminal window");
/// ```
pub fn terminal_size() -> Option<(u16, u16)> {
    // Use libc on Unix-like systems to obtain the width and height.
    #[cfg(unix)]
    {
        let mut size: winsize = unsafe { mem::zeroed() };
        if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) } == 0 {
            return Some((size.ws_col, size.ws_row));
        }
    }

    // Put windows code here
    #[cfg(windows)]
    {}

    None
}
