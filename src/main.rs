use std::process;

fn main() {
    if let Err(e) = count_down::run(){
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
