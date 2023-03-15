use rand::prelude::*;
use rand_pcg::Pcg64;

fn gen_list(n: u64){
    let mut rng:Pcg64 = Pcg64::from_entropy();
    let mut vector: Vec<u64> = Vec::new();
    for _ in 0..n {
        vector.push(rng.gen_range(0..2*n));
    }
    println!("{:?}", vector);
    vector.sort();
    println!("{:?}", vector);
    vector.reverse();
    println!("{:?}", vector);
}

fn insertion_sort(table: Vec<u64>){
    let length: usize = table.len();
    let mut i: usize = 1;
    while i < length {
        todo!();
    } 
}

fn main() {
    gen_list(10);
}
