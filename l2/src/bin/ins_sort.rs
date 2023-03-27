use lib::*;
use std::io::{self, Read};

fn main(){
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    let mut list: Vec<u64> = Vec::new();
    while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
        if n_bytes == 0 {
            break; 
        }
        list = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
        line.clear();
    }
    list = Vec::from(&list[1..]); 
    println!("{:?}", list);
    let stats: (u32, u32);
    if list.len() < 40 {
        stats = insertion_sort_with_stats(&mut list, true);
    }else {
        stats = insertion_sort_with_stats(&mut list, false);
    }
    println!("{:?}", list);
    println!("swaps = {}", stats.0);
    println!("checks = {}", stats.1);
}
