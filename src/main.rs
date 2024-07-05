//! Prints numbered command-line arguments to the console or terminal device.
//!
//! This is not equivalent to using stdout or stderr. Even if all standard
//! streams are redirected, this will still attempt to write to the console.

use std::{env, fs, io::{Error, Write}};

fn main() -> Result<(), Error> {
    let mut console = fs::OpenOptions::new()
        .write(true)
        .open( if cfg!(windows) { r#"\\.\CON"# } else { "/dev/tty" })?;

    for (i, arg) in env::args().enumerate() {
        write!(console, "{i}: [{arg}]\n")?;
    }

    Ok(())
}
