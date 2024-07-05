//! Prints numbered command-line arguments to the Windows console device.
//!
//! This is not equivalent to using stdout or stderr. Even if all standard
//! streams are redirected, this will still attempt to write to the console.
//!
//! Note that this implementation only works properly on Windows, because it
//! uses a device path for the console. This path, `\\.\CON`, like its legacy
//! short form `CON` and alternatives using `CONOUT$`, are specific to Windows.
//!
//! This could be made portable to Unix-like systems by using `/dev/tty`
//! instead of `\\.\CON` when the platform is not Windows.

use std::{env, fs::File, io::Write};

fn main() {
    let mut console = File::create(r#"\\.\CON"#).expect("can open console device");

    for (i, arg) in env::args().enumerate() {
        console
            .write(format!("{i}: [{arg}]\n").as_bytes())
            .expect("can write to console");
    }
}
