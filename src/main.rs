use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::process::ExitCode;
use std::env::args;

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();
    let message: &str = if args.len() > 1 { &args[1] } else { "Hello, Rust!" };
    let writer = BufWriter::new(stdout().lock());
    match say(message, message.len(), writer) {
        Ok(_) => { ExitCode::from(0) }
        Err(_) => { ExitCode::from(1) }
    }
}
