/// Controls functionality of the terminal.
///
/// Most of the functions in this module are just abtractions over platform
/// specific code.
pub mod terminal {
    use std::{io::Write, mem};

    #[cfg(unix)]
    use {
        libc::{ioctl, tcgetattr, tcsetattr, winsize, ICANON, STDOUT_FILENO, TCSANOW, TIOCGWINSZ},
        std::os::fd::AsRawFd,
    };

    #[cfg(windows)]
    use winapi::um::{
        handleapi::INVALID_HANDLE_VALUE,
        processenv::GetStdHandle,
        winbase::STD_OUTPUT_HANDLE,
        wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO},
    };

    /// Handle to hold terminal values.
    ///
    /// At its current state this is only used to return the terminal to its
    /// original state once the application is closed. This is done by keeping
    /// a copy of the previous state of the terminal and reverting to that
    /// state when ```Handle::drop()``` is called.
    pub struct Handle {
        #[cfg(unix)]
        original_termios: libc::termios,
        #[cfg(unix)]
        _termios: libc::termios,
    }

    #[cfg(unix)]
    impl Drop for Handle {
        fn drop(&mut self) {
            // Restore terminal attributes
            let file_descriptor = std::io::stdin().as_raw_fd();

            if unsafe { tcsetattr(file_descriptor, TCSANOW, &self.original_termios) } != 0 {
                eprintln!("Failed to restore terminal attributes")
            }

            // Clear screen and reset cursor
            let mut stdout = std::io::stdout();
            write!(stdout, "\x1b[2J\x1B[H").unwrap();
            stdout.flush().unwrap();
        }
    }

    /// Initializes the terminal for use by the engine
    ///
    /// This is what sets the terminal to use raw mode so that instead of
    /// writing lines in the terminal, we can instead use it as a screen.
    /// The terminal handle _must_ be owned in order for the application
    /// to function correctly, so the ```Handle``` should not be dropped
    /// until the engine is exiting.
    pub fn init() -> Result<Handle, &'static str> {
        #[cfg(unix)]
        {
            // Create a new termios
            let file_descriptor = std::io::stdin().as_raw_fd();
            let mut termios = unsafe {
                let mut termios = std::mem::zeroed();
                if tcgetattr(file_descriptor, &mut termios) != 0 {
                    return Err("Failed to get terminal attributes");
                }
                termios
            };

            // Save the previous termios so that we can restore it later
            let original_termios = termios;

            // Enable raw mode (disable canonical mode and echo)
            termios.c_lflag &= !(ICANON | TCSANOW as u64);

            if unsafe { tcsetattr(file_descriptor, TCSANOW, &termios) } != 0 {
                return Err("Failed to set terminal attributes");
            }

            return Ok(Handle {
                original_termios,
                _termios: termios,
            });
        }

        #[cfg(windows)]
        {
            return Ok(Handle {});
        }

        #[allow(unreachable_code)]
        Err("Platform not implemented")
    }

    /// Returns the size of the terminal window.
    ///
    /// More specifically, this will return the width and height of the terminal
    /// window measured in how many characters fit inside the console.
    /// Returns ```None``` if it is unable to obtain the width and height.
    ///
    /// # Example
    /// ```
    /// let (width, height) = size().expect("Failed to get the size of the terminal window");
    /// ```
    pub fn size() -> Option<(usize, usize)> {
        // Use libc on Unix-like systems
        #[cfg(unix)]
        {
            let mut size: winsize = unsafe { mem::zeroed() };
            if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) } == 0 {
                return Some((size.ws_col as usize, size.ws_row as usize));
            }
        }

        // Use winapi on Windows systems
        #[cfg(windows)]
        {
            unsafe {
                let handle = GetStdHandle(STD_OUTPUT_HANDLE);
                if handle == INVALID_HANDLE_VALUE {
                    return None;
                }

                let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
                if GetConsoleScreenBufferInfo(handle, &mut csbi) != 0 {
                    // Convert i16 to usize
                    return Some((
                        (csbi.srWindow.Right - csbi.srWindow.Left + 1).min(0) as usize,
                        (csbi.srWindow.Bottom - csbi.srWindow.Top + 1).min(0) as usize,
                    ));
                }
            }
        }

        None
    }
}
