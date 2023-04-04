use lib::*;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected n - length of list")
    }
    let length = args[1].parse::<u64>().unwrap();
    let mut rng = Pcg64::from_entropy();
    let list = gen_list(length, Order::Sorted);
    print!("{length}");
    print!(" {}", rng.gen_range(1..=length));
    print!("");
    print_to_std(list);
}

