use lib::*;

fn main() {
    let mut list = gen_list(25, Order::Random);
    println!("{:?}", list);
    insertion_sort(&mut list);
    println!("{:?}", list);
    list = gen_list(25, Order::Random);
    println!("{:?}", list);
    merge_sort(&mut list);
    println!("{:?}", list);
    list = gen_list(25, Order::Random);
    println!("{:?}", list);
    quick_sort(&mut list);
    println!("{:?}", list); 
}
