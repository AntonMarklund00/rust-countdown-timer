use std::{process, env};

fn main() {
    if let Err(e) = count_down::run(env::args().collect()){
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
