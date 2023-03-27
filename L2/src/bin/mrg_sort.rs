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
    merge_sort(&mut list);
    println!("{:?}", list);
}
