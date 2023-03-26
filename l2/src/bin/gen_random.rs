use lib::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected n - length of list")
    }
    dbg!(&args[1]);
}
