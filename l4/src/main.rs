mod trees;
use trees::*;

fn main() {
    let mut tree = BST::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);
    tree.insert(4);
    tree.insert(2);

    assert_eq!(tree.search(5), true);

    tree.print();
    tree.delete(5);

    assert_eq!(tree.search(5), false);
    tree.print()

}
