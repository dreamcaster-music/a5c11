#![allow(dead_code)]

pub mod rand {
    #[cfg(unix)]
    use std::io::Read;

    use super::terminal;
    use terminal::{size, Element};

    /// Generates a ```Vec``` of random numbers
    ///
    /// # Examples
    /// ```
    /// let random_chars = rand::<char>(200);
    /// let random_u8s: Vec<u8> = rand(25);
    /// ```
    pub fn vec<T: Default + Copy>(len: usize) -> Vec<T> {
        #[cfg(unix)]
        {
            let mut buffer = vec![0u8; len * size_of::<T>()]; // Allocate buffer for N elements of T
            std::fs::File::open("/dev/urandom")
                .expect("Failed to open /dev/urandom")
                .read_exact(&mut buffer)
                .expect("Failed to read random bytes");

            let mut result: Vec<T> = Vec::with_capacity(len);
            unsafe {
                for chunk in buffer.chunks_exact(size_of::<T>()) {
                    let mut element: T = T::default();
                    std::ptr::copy_nonoverlapping(
                        chunk.as_ptr(),
                        &mut element as *mut T as *mut u8,
                        size_of::<T>(),
                    );
                    result.push(element);
                }
            }

            result
        }

        #[cfg(windows)]
        {
            use winapi::um::wincrypt::{
                CryptAcquireContextA, CryptGenRandom, CryptReleaseContext, CRYPT_VERIFYCONTEXT,
                HCRYPTPROV, PROV_RSA_FULL,
            };
            {
                use std::mem::size_of;
                use std::ptr::null_mut;

                let mut h_provider: HCRYPTPROV = 0;
                unsafe {
                    if CryptAcquireContextA(
                        &mut h_provider,
                        null_mut(),
                        null_mut(),
                        PROV_RSA_FULL,
                        CRYPT_VERIFYCONTEXT,
                    ) == 0
                    {
                        panic!("Failed to acquire cryptographic context! NOT CYBER SECURE")
                    }
                }

                let mut buffer = vec![0u8; len * size_of::<T>()];
                unsafe {
                    if CryptGenRandom(h_provider, buffer.len() as u32, buffer.as_mut_ptr()) == 0 {
                        CryptReleaseContext(h_provider, 0);
                        panic!("FAILED TO GENERATE RANDOM BYTES");
                    }
                }

                unsafe {
                    CryptReleaseContext(h_provider, 0);
                }

                let mut result: Vec<T> = Vec::with_capacity(len);
                unsafe {
                    for chunk in buffer.chunks_exact(size_of::<T>()) {
                        let mut element: T = T::default();
                        std::ptr::copy_nonoverlapping(
                            chunk.as_ptr(),
                            &mut element as *mut T as *mut u8,
                            size_of::<T>(),
                        );
                        result.push(element);
                    }
                }
                return result;
            }
        }
    }

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
            use std::ptr::null_mut;
            use winapi::um::wincrypt::{
                CryptAcquireContextA, CryptGenRandom, CryptReleaseContext, CRYPT_VERIFYCONTEXT,
                HCRYPTPROV, PROV_RSA_FULL,
            };

            let mut h_provider: HCRYPTPROV = 0;
            unsafe {
                // Acquire a cryptographic provider context
                if CryptAcquireContextA(
                    &mut h_provider,
                    null_mut(),
                    null_mut(),
                    PROV_RSA_FULL,
                    CRYPT_VERIFYCONTEXT,
                ) == 0
                {
                    panic!("Failed to acquire cryptographic context!");
                }
            }

            let mut buffer = vec![0u8; std::mem::size_of::<T>()];
            unsafe {
                // Generate random bytes
                if CryptGenRandom(h_provider, buffer.len() as u32, buffer.as_mut_ptr()) == 0 {
                    CryptReleaseContext(h_provider, 0);
                    panic!("Failed to generate random bytes!");
                }
            }

            unsafe {
                // Release the cryptographic provider context
                CryptReleaseContext(h_provider, 0);
            }

            let mut result: T = T::default();
            unsafe {
                // Copy the random bytes into the result
                std::ptr::copy_nonoverlapping(
                    buffer.as_ptr(),
                    &mut result as *mut T as *mut u8,
                    std::mem::size_of::<T>(),
                );
            }

            result
        }
    }

    /// Generates a ```Vec``` of random numbers within a range using ```rand()```
    ///
    /// # Examples
    /// ```
    /// let random_chars = rand_vec_range::<u8>(0, 100, 200) as char;
    /// let random_u8s: Vec<u8> = rand_vec_range(0, 10, 25);
    /// ```
    pub fn vec_range<
        T: Default
            + Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Rem<Output = T>
            + PartialOrd
            + PartialEq,
    >(
        min: T,
        max: T,
        len: usize,
    ) -> Vec<T> {
        if min == max {
            return vec![min; len];
        }

        vec::<T>(len)
            .into_iter()
            .map(|value| {
                // Compute the range size
                let range_size = max - min;

                // Ensure the random value is within the range
                let result = (value % range_size) + min;

                result
            })
            .collect()
    }

    /// Generates a random number within a range using ```rand()```
    ///
    /// # Examples
    /// ```
    /// let random_char = rand_range::<u8>(0, 100) as char;
    /// let random_u8: u8 = rand_range(0, 10);
    /// ```
    pub fn range<
        T: Default
            + Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Rem<Output = T>
            + PartialOrd
            + PartialEq,
    >(
        min: T,
        max: T,
    ) -> T {
        if min > max {
            panic!("min must be less than or equal to max");
        }
        if min == max {
            return min;
        }

        // Generate a random value using the custom `rand` function
        let value: T = rand();

        // Compute the range size
        let range_size = max - min;

        // Ensure the random value is within the range
        let result = (value % range_size) + min;

        result
    }

    /// Shuffles a vector using ```rand()```
    ///
    /// # Example
    /// ```
    /// let mut vec: Vec<_> = (0..5).collect();
    /// shuffle(&mut vec);
    /// ```
    pub fn shuffle<
        T: Default
            + Copy
            + std::ops::Sub
            + std::ops::Add
            + std::ops::Rem
            + std::convert::From<<T as std::ops::Sub>::Output>
            + std::convert::From<<T as std::ops::Add>::Output>
            + std::convert::From<<T as std::ops::Rem>::Output>
            + PartialEq,
    >(
        vec: &mut Vec<T>,
    ) {
        for i in 0..vec.len() {
            let j = range(0, vec.len() - 1);
            vec.swap(i, j);
        }
    }

    /// Represents a Perlin noise permutation which can be used to sample
    /// Perlin noise from.
    ///
    /// # Example
    /// ```
    /// let perlin = Perlin::new();
    /// let a = perlin.sample(0.5, 1.5, 0.0, 255.0);
    /// let b = perlin.sample(1.5, 3.5, 0.0, 255.0);
    /// ```
    pub struct Perlin {
        size: usize,
        permutation: Vec<usize>,
    }

    impl Perlin {
        /// Generate a new Perlin noise permutation which can be used to sample
        /// from.
        pub fn new() -> Self {
            let size = 256;

            let mut permutation: Vec<usize> = (0..size).collect();
            shuffle(&mut permutation);

            Self { size, permutation }
        }

        /// Sample Perlin noise at the specified coordinate, with the specified
        /// miniumum and maximum values.
        ///
        /// # Example
        /// ```
        /// let a = perlin.sample(0.5, 1.5, 0.0, 255.0);
        /// let b = perlin.sample(1.5, 3.5, 0.0, 255.0);
        /// ```
        pub fn sample(&self, x: f32, y: f32, min: f32, max: f32) -> f32 {
            let fade = |t: f32| t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
            let lerp = |a: f32, b: f32, t: f32| a + t * (b - a);
            let grad = |hash: usize, x: f32, y: f32| {
                let h = hash & 15;
                let u = if h < 8 { x } else { y };
                let v = if h < 4 {
                    y
                } else if h == 12 || h == 14 {
                    x
                } else {
                    0.0
                };
                (if (h & 1) == 0 { u } else { -u }) + (if (h & 2) == 0 { v } else { -v })
            };

            // Determine grid cell coordinates
            let x_floor = x.floor() as usize & 255;
            let y_floor = y.floor() as usize & 255;

            // Relative coordinates in grid cell
            let xf = x - x.floor();
            let yf = y - y.floor();

            // Fade curves for smooth interpolation
            let u = fade(xf);
            let v = fade(yf);

            // Hash coordinates of the 4 corners
            let aa = self.permutation[(x_floor + self.permutation[y_floor]) % self.size] as f32;
            let ab = self.permutation
                [(x_floor + self.permutation[(y_floor + 1) % self.size]) % self.size]
                as f32;
            let ba = self.permutation[(x_floor + 1 + self.permutation[y_floor]) % self.size] as f32;
            let bb = self.permutation
                [(x_floor + 1 + self.permutation[(y_floor + 1) % self.size]) % self.size]
                as f32;

            // Interpolate along x axis
            let x1 = lerp(
                grad(aa as usize, xf, yf),
                grad(ba as usize, xf - 1.0, yf),
                u,
            );
            let x2 = lerp(
                grad(ab as usize, xf, yf - 1.0),
                grad(bb as usize, xf - 1.0, yf - 1.0),
                u,
            );

            // Interpolate along y axis
            let value = lerp(x1, x2, v);
            Self::normalize(value, min, max)
        }

        /// Normalize a Perlin noise value between a minimum and a maximum
        fn normalize(value: f32, min: f32, max: f32) -> f32 {
            // Normalize the value from [-1, 1] to [0, 1]
            let normalized = (value + 1.0) / 2.0;

            // Scale the normalized value to [min, max]
            normalized * (max - min) + min
        }
    }
}

/// Controls functionality of the terminal.
///
/// Most of the functions in this module are just abtractions over platform
/// specific code.
pub mod terminal {
    use std::{io::Write, mem};

    use super::Sprite;
    #[cfg(windows)]
    use winapi::um::{
        consoleapi::GetConsoleMode,
        handleapi::INVALID_HANDLE_VALUE,
        processenv::GetStdHandle,
        winbase::STD_OUTPUT_HANDLE,
        wincon::{
            GetConsoleCursorInfo, GetConsoleScreenBufferInfo, SetConsoleCursorInfo,
            CONSOLE_CURSOR_INFO, CONSOLE_SCREEN_BUFFER_INFO, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        },
        winnt::HANDLE,
        winuser::ShowCursor,
    };

    #[cfg(unix)]
    use {
        libc::{
            ioctl, tcgetattr, tcsetattr, winsize, ECHO, ICANON, IXOFF, IXON, STDOUT_FILENO,
            TCSANOW, TIOCGWINSZ, VMIN, VTIME,
        },
        std::os::fd::AsRawFd,
    };

    /// Represents an escape character
    pub const ESC: &'static str = "\x1b";

    pub const SCREEN_BUFFER_ALT: &'static str = "\x1b[?1049h\x1b[2J\x1b[H";
    pub const SCREEN_BUFFER_DEF: &'static str = "\x1b[2J\x1b[H\x1b[?10491";

    /// Represents an element on the screen.
    ///
    /// See https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797#256-colors
    /// for what colors are able to be used.
    #[derive(Clone)]
    pub struct Element {
        char: char,
        fg_code: u8,
        bg_code: u8,
    }

    impl Element {
        pub fn new(character: char, foreground_code: u8, background_code: u8) -> Self {
            Self {
                char: character,
                fg_code: foreground_code,
                bg_code: background_code,
            }
        }

        pub fn char(&self) -> char {
            self.char
        }

        pub fn fg(&self) -> String {
            format!("{ESC}[38;5;{}m", self.fg_code)
        }

        pub fn bg(&self) -> String {
            format!("{ESC}[48;5;{}m", self.bg_code)
        }
        // Method to reverse the foreground and background colors
        pub fn reverse_colors(&mut self) {
            std::mem::swap(&mut self.fg_code, &mut self.bg_code);
        }
    }

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

            // Switch to original screen buffer, clear screen and reset cursor
            let mut stdout = std::io::stdout();
            //write!(stdout, "{SCREEN_BUFFER_DEF}{ESC}[h").unwrap();
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
            println!("File descriptor: {}", file_descriptor);
            let mut termios = unsafe {
                let mut termios = std::mem::zeroed();
                if tcgetattr(file_descriptor, &mut termios) != 0 {
                    return Err("Failed to get terminal attributes");
                }
                termios
            };

            // Save the previous termios so that we can restore it later
            let original_termios = termios.clone();

            // Enable raw mode (disable canonical mode and echo)

            // Disable canonical mode and echo
            termios.c_lflag &= !(ICANON | ECHO);

            // Disable flow control
            termios.c_iflag &= !(IXON | IXOFF);

            termios.c_cc[VMIN] = 1; // Minimum number of characters for non-blocking reads
            termios.c_cc[VTIME] = 0; // Timeout for reads

            // Apply the raw mode settings
            if unsafe { tcsetattr(file_descriptor, TCSANOW, &termios) } != 0 {
                return Err("Failed to set terminal attributes");
            }

            let stdout = std::io::stdout();
            let mut handle = stdout.lock();

            // Switch to alternate screen buffer
            write!(handle, "{SCREEN_BUFFER_ALT}").map_err(|_| "Failed to write to handle")?;
            handle.flush().map_err(|_| "Failed to flush handle")?;

            return Ok(Handle {
                original_termios,
                _termios: termios,
            });
        }

        #[cfg(windows)]
        const STD_OUTPUT_HANDLE: u32 = -11i32 as u32; // Constant for standard output handle.

        #[cfg(windows)]
        {
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();

            unsafe {
                let h_stdout: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
                if h_stdout == INVALID_HANDLE_VALUE {
                    return Err("Failed to get standard output handle...");
                }

                // Enable virtual terminal processing for better control
                let mut mode: u32 = 0;
                if GetConsoleMode(h_stdout, &mut mode) == 0 {
                    return Err("Failed to get console mode.");
                }

                if winapi::um::consoleapi::SetConsoleMode(
                    h_stdout,
                    mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING,
                ) == 0
                {
                    return Err("Failed to set console mode.");
                }
            }

            // Switch to alternate screen buffer
            write!(handle, "\x1b[?1049h").map_err(|_| "Failed to write to handle")?;
            handle.flush().map_err(|_| "Failed to flush handle")?;

            // Hide the cursor using ANSI escape codes
            write!(handle, "\x1b[?25l").map_err(|_| "Failed to write to handle")?;

            Ok(Handle {})
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
                let handle = GetStdHandle(STD_OUTPUT_HANDLE);
                if handle == INVALID_HANDLE_VALUE {
                    return None;
                }

                let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
                if GetConsoleScreenBufferInfo(handle, &mut csbi) != 0 {
                    return Some((
                        (csbi.srWindow.Right - csbi.srWindow.Left + 1).max(0) as usize,
                        (csbi.srWindow.Bottom - csbi.srWindow.Top + 1).max(0) as usize,
                    ));
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
    /// display_raw(&vec![]).unwrap();
    /// ```
    pub fn display_raw(elements: &Vec<Element>) -> Result<(), &'static str> {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        // Get terminal size
        let (width, height) = size().ok_or("Failed to get display size")?;

        // Use a buffer for efficient printing
        // Don't entirely know why I need to add 33 specifically but if I don't the buffer will resize
        let mut buf = String::with_capacity(height * width);

        // Clear screen
        buf.push_str("\x1b[2J\x1b[H");

        // Push elements to the buffer
        let mut last: Option<(u8, u8)> = None;
        for i in 0..(height * width) {
            let element = elements.get(i).ok_or("Index out of bounds")?;

            if last.is_none() || last.unwrap().0 != element.fg_code {
                buf.push_str(&element.fg());
            }

            if last.is_none() || last.unwrap().1 != element.bg_code {
                buf.push_str(&element.bg());
            }

            buf.push_str(&element.char().to_string());

            last = Some((element.fg_code, element.bg_code));
        }
        write!(handle, "{}", buf).map_err(|_| "Failed to write to handle")?;

        // Flush handle
        handle.flush().map_err(|_| "Failed to flush handle")?;

        Ok(())
    }

    /// Displays a list of ```Sprite```s on the screen.
    pub fn display(sprites: &mut Vec<Box<dyn super::Sprite>>) -> Result<(), &'static str> {
        let (width, height) = size().ok_or("Failed to get display size")?;
        let mut display = (0..(width * height))
            .map(|_| Element::new(' ', 0, 0))
            .collect::<Vec<_>>();

        for sprite in sprites.iter_mut() {
            for element in sprite.elements() {
                let (x, y) = (element.1 .0, element.1 .1);

                let x = x.floor();
                let y = y.floor();
                if x < 0.0 || y < 0.0 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;

                if x < width && y < height {
                    display[y * width + x] = element.0;
                }
            }

            sprite.next();
        }

        display_raw(&display)?;

        Ok(())
    }

    /// Converts a color from RGB to ASCII-256
    ///
    /// Colors must be between 0-5. Any values above this range will be
    /// set to 5 instead.
    ///
    /// # Examples
    /// ```
    /// let color = rgb(0, 1, 0); // dark green
    /// let color = rgb(5, 0, 5); // bright magenta
    /// let color = rgb(3, 3, 3); // grey
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> u8 {
        let r = r.clamp(0, 5);
        let g = g.clamp(0, 5);
        let b = b.clamp(0, 5);

        if r == g && r == b {
            if r == 0 {
                16
            } else if r == 6 {
                255
            } else {
                232 + r * 4
            }
        } else {
            const R: u8 = 36;
            const G: u8 = 6;
            const B: u8 = 1;

            let mut color = 0;
            color += b.min(G);
            color += g.min(R) * G;
            color += r * R;

            color + 16
        }
    }
}

pub mod keyboard {}

pub mod shapes {
    use super::terminal::{self};

    /// Struct to represent a square.
    pub struct Square {
        width: usize,
        height: usize,
        x: usize,
        y: usize,
        position: (usize, usize), // top-left by default
    }

    impl Square {
        /// Create a new square with specified width, height, and position.
        pub fn new(width: usize, height: usize, x: usize, y: usize) -> Self {
            Self {
                width,
                height,
                x,
                y,
                position: (x, y),
            }
        }

        /// Draw the square at its position in the terminal.
        pub fn draw(&self) {
            for i in 0..self.height {
                for j in 0..self.width {
                    // Print spaces for the top-left position
                    if i == 0 || i == self.height - 1 || j == 0 || j == self.width - 1 {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
        }
    }
}

/// Represents a position on the screen
pub struct Position(pub f32, pub f32);

/// Represents a sprite that can be displayed on the screen
pub trait Sprite {
    fn elements(&self) -> Vec<(terminal::Element, Position)>;
    fn next(&mut self);
}
