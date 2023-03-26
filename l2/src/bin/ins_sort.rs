use lib::*;
use std::io::{self, Read};

fn main(){
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
        if n_bytes == 0 { break }
        print!("{}", line);
        line.clear();
    }
}
