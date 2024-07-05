fn main() {
    for (i, arg) in std::env::args().enumerate() {
        eprintln!("{i}: [{arg}]");
    }
}
