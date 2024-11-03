use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::process::ExitCode;
use std::env::args;

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();
    let message: &str = if args.len() > 1 { &args[1] } else { "Hello, Rust!" };
    let width = message.len();
    let writer = BufWriter::new(stdout().lock());
    if say(message, width, writer).is_err() {
        return ExitCode::from(1);
    }
    ExitCode::from(0)
}
