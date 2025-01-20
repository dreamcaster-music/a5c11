use std::io::Read;

/// Generates a random number
///
/// I have no idea if this is cryptographically secure so please don't use it
/// for anything like that.
///
/// # Examples
/// ```
/// let random_char = rand::<char>();
/// let random_u8: u8 = rand();
/// ```
pub fn rand<T: Default + Copy>() -> T {
    #[cfg(unix)]
    {
        let mut buffer = vec![0u8; size_of::<T>()];
        std::fs::File::open("/dev/urandom")
            .expect("Failed to open /dev/urandom")
            .read_exact(&mut buffer)
            .expect("Failed to read random bytes");

        let mut result: T = T::default();
        unsafe {
            std::ptr::copy_nonoverlapping(
                buffer.as_ptr(),
                &mut result as *mut T as *mut u8,
                size_of::<T>(),
            );
        }
        return result;
    }

    #[cfg(windows)]
    {
        todo!()
    }
}

/// Controls functionality of the terminal.
///
/// Most of the functions in this module are just abtractions over platform
/// specific code.
pub mod terminal {
    #[cfg(unix)]
    pub const ESC: &'static str = "\x1b";

    #[cfg(windows)]
    pub const ESC: &'static str = "\\e";

    /// Represents an element on the screen.
    ///
    /// See https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797#256-colors
    /// for what colors are able to be used.
    pub struct Element {
        character: char,
        foreground_code: u8,
        background_code: u8,
    }

    #[allow(dead_code)]
    impl Element {
        pub fn new(character: char, foreground_code: u8, background_code: u8) -> Self {
            Self {
                character,
                foreground_code,
                background_code,
            }
        }
    }

    impl ToString for Element {
        fn to_string(&self) -> String {
            format!(
                "{ESC}[38;5;{foreground}m{ESC}[48;5;{background}m{character}",
                foreground = self.foreground_code,
                background = self.background_code,
                character = self.character
            )
        }
    }
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
            write!(stdout, "{ESC}[2j{ESC}[h").unwrap();
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
    ///
    /// # Example
    /// ```
    /// let _handle = init().expect("Failed to initialize terminal");
    /// ```
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
                if GetStdHandle(STD_OUTPUT_HANDLE) != INVALID_HANDLE_VALUE {
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
        }

        None
    }

    /// Displays an array of elements on the screen
    ///
    /// The array of elements is expected to be a flattened array arranged
    /// from left to right, then top to bottom, like a book.
    ///
    /// # Example
    ///
    /// ```
    /// display(&vec![]).unwrap();
    /// ```
    pub fn display(elements: &Vec<Element>) -> Result<(), &'static str> {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        // Clear screen and move cursor to top left
        write!(handle, "{ESC}[2j{ESC}[h").map_err(|_| "Failed to write to handle")?;

        // Get terminal size
        let (width, height) = size().ok_or("Failed to get display size")?;

        // Write elements to screen
        for y in 0..height {
            for x in 0..width {
                // Write the current element
                write!(
                    handle,
                    "{}",
                    elements
                        .get(x + y * width)
                        .ok_or("Index out of bounds")?
                        .to_string()
                )
                .map_err(|_| "Failed to write to handle")?;
            }

            // Next line
            writeln!(handle).map_err(|_| "Failed to write to handle")?;
        }

        // Flush handle
        handle.flush().map_err(|_| "Failed to flush handle")?;

        Ok(())
    }
}
