use lib::*;

fn main(){
    let mut list = gen_list(10, Order::Random);
    println!("{:?}", list);
    dual_pivot_quicksort(&mut list);
    println!("{:?}", list);
    print!("{:?}", is_sorted(&list));
}