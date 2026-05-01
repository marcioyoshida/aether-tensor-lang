/// Pretty error reporting — emits annotated source spans to stderr.

use std::fmt::Display;

pub fn report_and_exit<E: Display>(err: E) -> ! {
    eprintln!("\x1b[1;31merror\x1b[0m: {err}");
    std::process::exit(1);
}

pub fn warn<E: Display>(msg: E) {
    eprintln!("\x1b[1;33mwarning\x1b[0m: {msg}");
}

/// Attaches a file + line annotation to an error message.
pub fn with_span(file: &str, line: usize, col: usize, msg: &str) -> String {
    format!("{file}:{line}:{col}: {msg}")
}
