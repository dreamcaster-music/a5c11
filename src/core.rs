use std::mem;

#[cfg(unix)]
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

#[cfg(windows)]
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
#[cfg(windows)]
use winapi::um::processenv::GetStdHandle;
#[cfg(windows)]
use winapi::um::winbase::STD_OUTPUT_HANDLE;
#[cfg(windows)]
use winapi::um::wincon::GetConsoleScreenBufferInfo;
#[cfg(windows)]
use winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO;

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
    {
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            if handle == INVALID_HANDLE_VALUE {
                return None;
            }

            let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
            if GetConsoleScreenBufferInfo(handle, &mut csbi) != 0 {
                // Convert i16 to u16 using try_into()
                return Some((
                    (csbi.srWindow.Right - csbi.srWindow.Left + 1)
                        .try_into()
                        .unwrap(),
                    (csbi.srWindow.Bottom - csbi.srWindow.Top + 1)
                        .try_into()
                        .unwrap(),
                ));
            }
        }
    }

    None
}
