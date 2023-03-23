use rand::prelude::*;
use rand_pcg::Pcg64;

mod sort;

#[derive(Debug)]
enum Order{
    Random,
    Sorted,
    Reverse
}

fn gen_list(n: u64, order: Order) -> Vec<u64>{
    let mut rng:Pcg64 = Pcg64::from_entropy();
    let mut vector: Vec<u64> = Vec::new();
    for _ in 0..n {
        vector.push(rng.gen_range(0..2*n));
    }
    match order {
        Order::Random => return vector,
        Order::Sorted => {
            vector.sort();
            return vector;
        },
        Order::Reverse => {
            vector.sort();
            vector.reverse();
            return vector;
        },
    }
}

fn special_print(list: Vec<u64>){
    todo!()
}

fn main() {
    let mut list = gen_list(25, Order::Random);
    println!("{:?}", list);
    sort::insertion_sort(&mut list);
    println!("{:?}", list);
    list = gen_list(25, Order::Random);
    println!("{:?}", list);
    sort::merge_sort(&mut list);
    println!("{:?}", list);
    list = gen_list(25, Order::Random);
    println!("{:?}", list);
    sort::quick_sort(&mut list);
    println!("{:?}", list);
}
