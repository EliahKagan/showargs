use std::{env, fs, io::Write};

fn main() {
    let mut console = fs::OpenOptions::new()
        .write(true)
        .open(r#"\\.\CON"#)
        .expect("can open console device");

    for (i, arg) in env::args().enumerate() {
        let line = format!("{i}: [{arg}]\n");
        console.write(line.as_bytes()).expect("can write to console");
    }
}
