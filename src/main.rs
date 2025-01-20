use std::{
    io::{self, Write},
    mem,
    time::Duration,
};

use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

fn terminal_size() -> Option<(u16, u16)> {
    #[cfg(unix)]
    {
        let mut size: winsize = unsafe { mem::zeroed() };
        if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) } == 0 {
            return Some((size.ws_col, size.ws_row));
        }
    }
    None
}

fn main() {}
