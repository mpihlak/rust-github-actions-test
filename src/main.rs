use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--version" {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}", version);
    } else {
        println!("Hello, world!");
    }
}
