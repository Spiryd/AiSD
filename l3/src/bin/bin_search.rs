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
    let l_clone = list.clone();
    if l_clone.len() <= 50 {
        let res = bin_is_in(list[k as usize], list.clone());
        println!("Czy element {} jest w {:?} : {}", list[k as usize], list, res);
        let res = bin_is_in(2 * list.len() as u64, list.clone());
        println!("Czy element {} jest w {:?} : {}", 2*list.len(), list, res);
    }
}
