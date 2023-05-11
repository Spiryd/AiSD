use lib::*;

fn main() {
    let mut b_tree = BinarySearchTree::new();
    let items = gen_list(50, Order::Random);
    for item in &items{
        b_tree.insert(*item);
    }
    b_tree.print();
    println!("Tree height: {}", b_tree.height());

    let mut rb_tree = RBTree::new();
    for item in &items{
        rb_tree.insert(*item);
    }
    rb_tree.print_tree();
    println!("Tree height: {}", rb_tree.height());
}
