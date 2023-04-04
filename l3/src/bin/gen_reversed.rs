use lib::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected n - length of list")
    }
    let length = args[1].parse::<u64>().unwrap();
    let list = gen_list(length, Order::Reverse);
    print!("{length}");
    print_to_std(list);
}
