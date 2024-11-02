use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello, Rust!");
    let width = message.chars().count();
    let writer = BufWriter::new(stdout.lock());
    let _ = say("Hello. Rust!", width, writer);
}
