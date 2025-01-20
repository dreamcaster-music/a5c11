use std::mem;

#[cfg(unix)]
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

#[cfg(windows)]
use winapi::um::{
    handleapi::INVALID_HANDLE_VALUE,
    processenv::GetStdHandle,
    winbase::STD_OUTPUT_HANDLE,
    wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO},
};

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
pub fn terminal_size() -> Option<(usize, usize)> {
    // Use libc on Unix-like systems to obtain the width and height.
    #[cfg(unix)]
    {
        let mut size: winsize = unsafe { mem::zeroed() };
        if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) } == 0 {
            return Some((size.ws_col as usize, size.ws_row as usize));
        }
    }

    // Put windows code here
    #[cfg(windows)]
    {
        let handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        if handle == INVALID_HANDLE_VALUE {
            return None;
        }

        let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = unsafe { mem::zeroed() };
        if unsafe { GetConsoleScreenBufferInfo(handle, &mut csbi) } != 0 {
            // Convert i16 to u16 using try_into()
            return Some((
                (csbi.srWindow.Right - csbi.srWindow.Left + 1).min(0) as usize,
                (csbi.srWindow.Bottom - csbi.srWindow.Top + 1).min(0) as usize,
            ));
        }
    }

    None
}
