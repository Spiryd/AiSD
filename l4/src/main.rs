mod trees;
use trees::*;

fn main() {
    let mut tree = BinarySearchTree::new(); 
    tree.insert(50);
    tree.insert(30);
    tree.insert(20);
    tree.insert(40);
    tree.insert(70);
    tree.insert(60);
    tree.insert(80);
    
    tree.print();

    println!("Tree height: {}", tree.height());
    println!("Contains 20: {}", tree.search(20));
    println!("Contains 10: {}", tree.search(10));

    tree.delete(30);
    tree.print();
}
