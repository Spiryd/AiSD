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
    if list.len() <= 50 {
        println!("{k}th smallest element = {:?}", randomized_select(&mut list, k as usize));
    } else {
        println!("{k}th smallest element = {:?}", randomized_select(&mut list, k as usize));
    }
}
