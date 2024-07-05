fn main() {
    for (i, arg) in std::env::args().enumerate() {
        std::fs::write(r#"\\.\CON"#, format!("{i}: [{arg}]\n")).expect("can write to console");
    }
}
