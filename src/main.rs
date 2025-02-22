//! Prints numbered command-line arguments to the console or terminal device.
//!
//! This is not equivalent to using stdout or stderr. Even if all standard
//! streams are redirected, this will still attempt to write to the console.

use std::{
    env, fs,
    io::{Error, Write},
};

#[cfg(windows)]
const DEVICE_PATH: &str = r"\\.\CON";

#[cfg(not(windows))]
const DEVICE_PATH: &str = "/dev/tty";

fn main() -> Result<(), Error> {
    let mut console = fs::OpenOptions::new().write(true).open(DEVICE_PATH)?;

    for (i, arg) in env::args().enumerate() {
        writeln!(console, "{i}: [{arg}]")?;
    }

    Ok(())
}
