use std::{env, io::Write, thread, time::Duration};

fn main() {
    match env::args().nth(1).as_deref() {
        Some("success") => {
            println!("stdout-marker");
            eprintln!("stderr-marker");
        }
        Some("nonzero") => std::process::exit(23),
        Some("sleep") => {
            println!("started");
            std::io::stdout().flush().expect("flush stdout");
            thread::sleep(Duration::from_secs(30));
        }
        _ => std::process::exit(64),
    }
}
