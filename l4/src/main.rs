use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use lib::*;

fn main() {

    loop {
        let choices = ["Test BST", "Test RBTree", "Test SplayTree", "Collect Data", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("SELECT MODE")
            .items(&choices)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .expect("failed");
        if selection.is_none() {
            panic!("User did not select anything")
        }
    
        match selection.unwrap() {
            0 => test_bst(),
            1 => test_rb(),
            2 => test_splay(),
            3 => collect_data(),
            _ => break,
        }  
    }
}

fn test_bst() {
    let items = gen_list(50, Order::Sorted);
    let mut b_tree = BinarySearchTree::new();
    for item in &items{
        println!("Inserting: {}", item);
        b_tree.insert(*item);
        b_tree.print();
        println!("Tree height: {}", b_tree.height());
    }
    let items = gen_list(50, Order::Random);
    for item in &items{
        println!("Deleting: {}", item);
        b_tree.delete(*item);
        b_tree.print();
        println!("Tree height: {}", b_tree.height());
    }
    let items = gen_list(50, Order::Random);
    let mut b_tree = BinarySearchTree::new();
    for item in &items{
        println!("Inserting: {}", item);
        b_tree.insert(*item);
        b_tree.print();
        println!("Tree height: {}", b_tree.height());
    }
    let items = gen_list(50, Order::Random);
    for item in &items{
        println!("Deleting: {}", item);
        b_tree.delete(*item);
        b_tree.print();
        println!("Tree height: {}", b_tree.height());
    }
}

fn test_rb() {
    let items = gen_list(50, Order::Sorted);
    let mut rb_tree = RBTree::new();
    for item in &items{
        println!("Inserting: {}", item);
        rb_tree.insert(*item);
        rb_tree.print_tree();
        println!("Tree height: {}", rb_tree.height());
    }
    let items = gen_list(50, Order::Random);
    for item in &items{
        println!("Deleting: {}", item);
        rb_tree.delete(*item);
        rb_tree.print_tree();
        println!("Tree height: {}", rb_tree.height());
    }
    let items = gen_list(50, Order::Random);
    let mut rb_tree = RBTree::new();
    for item in &items{
        println!("Inserting: {}", item);
        rb_tree.insert(*item);
        rb_tree.print_tree();
        println!("Tree height: {}", rb_tree.height());
    }
    let items = gen_list(50, Order::Random);
    for item in &items{
        println!("Deleting: {}", item);
        rb_tree.delete(*item);
        rb_tree.print_tree();
        println!("Tree height: {}", rb_tree.height());
    }
}

fn test_splay() {
    let items = gen_list(50, Order::Sorted);
    let mut splay_tree = SplayTree::new();
    for item in &items{
        println!("Inserting: {}", item);
        splay_tree.insert(*item);
        splay_tree.print_tree();
        println!("Tree height: {}", splay_tree.height());
    }
    let items = gen_list(50, Order::Random);
    for item in &items{
        println!("Deleting: {}", item);
        splay_tree.delete(*item);
        splay_tree.print_tree();
        println!("Tree height: {}", splay_tree.height());
    }
    let items = gen_list(50, Order::Random);
    let mut splay_tree = SplayTree::new();
    for item in &items{
        println!("Inserting: {}", item);
        splay_tree.insert(*item);
        splay_tree.print_tree();
        println!("Tree height: {}", splay_tree.height());
    }
    let items = gen_list(50, Order::Random);
    for item in &items{
        println!("Deleting: {}", item);
        splay_tree.delete(*item);
        splay_tree.print_tree();
        println!("Tree height: {}", splay_tree.height());
    }
}

fn collect_data() {
    get_cmps();
}
