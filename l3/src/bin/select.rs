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
        list = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
        line.clear();
    }
    let k = list[1];
    list = Vec::from(&list[2..]);
    let mut l_clone = list.clone();
    if l_clone.len() <= 50 {
        let val = select_with_stats(&mut list, k as usize, true);
        println!("after       = {:?}", list);
        println!("original    = {:?}", l_clone);
        println!("{k}th smallest element = {}", val.0);
        l_clone.sort();
        println!("list sorted = {:?}", l_clone);
        println!("cmps = {}, swaps = {}", val.1.0, val.1.1)
    } else {
        let val = select_with_stats(&mut list, k as usize, false);
        println!("cmps = {}, swaps = {}", val.1.0, val.1.1)
    }
}
