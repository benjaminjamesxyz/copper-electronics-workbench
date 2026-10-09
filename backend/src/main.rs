use std::io::{self, BufRead, Write};
use pcb_calculator::{Engine, respond};
fn main() {
    let engine = match Engine::new() { Ok(e) => e, Err(e) => { eprintln!("Catalog error: {e}"); std::process::exit(1); } };
    let stdin = io::stdin(); let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let line = match line { Ok(l) => l, Err(e) => { eprintln!("Input error: {e}"); break; } };
        if writeln!(stdout, "{}", respond(&engine, &line)).and_then(|_| stdout.flush()).is_err() { break; }
    }
}
