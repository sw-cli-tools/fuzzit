use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.starts_with("panic") {
            eprintln!("thread 'main' panicked at 'deliberate panic for testing'");
            std::process::exit(101);
        }
        if line == "loop" {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
        writeln!(stdout, "{line}").unwrap();
    }
}
