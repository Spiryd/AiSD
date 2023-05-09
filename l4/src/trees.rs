use std::cmp::Ordering;
use std::fmt::Write;

#[derive(Debug, Clone)]
struct Node {
    value: u64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: u64) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn search(&self, value: u64) -> bool {
        if self.value == value {
            return true;
        }
        if value < self.value {
            if let Some(left) = &self.left {
                left.search(value)
            } else {
                false
            }
        } else if let Some(right) = &self.right {
            return right.search(value);
        } else {
            false
        }
        
    }

    fn insert(&mut self, value: u64) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Equal => {}
        }
    }
 
    fn height(&self) -> u64 {
        let left_height = match &self.left {
            Some(left) => left.height(),
            None => 0,
        };
        let right_height = match &self.right {
            Some(right) => right.height(),
            None => 0,
        };
        1 + std::cmp::max(left_height, right_height)
    }

    fn print(&self, sb: &mut String, padding: &str, pointer: &str, has_right_sibling: bool) {
        writeln!(sb).unwrap();
        write!(sb, "{}", padding).unwrap();
        write!(sb, "{}", pointer).unwrap();
        write!(sb, "{}", self.value).unwrap();
        let mut padding_builder = String::from(padding);
        if has_right_sibling {
            padding_builder.push_str("│  ");
        } else {
            padding_builder.push_str("   ");
        }
        let padding_for_both = padding_builder.as_str();
        let pointer_right = "└──";
        let pointer_left = if self.right.is_some() { "├──" } else { "└──" };
        if let Some(left) = &self.left {
            left.print(sb, padding_for_both, pointer_left, self.right.is_some());
        }
        if let Some(right) = &self.right {
            right.print(sb, padding_for_both, pointer_right, false);
        }
    }

    fn delete(mut this: Box<Node>, target: &u64) -> Option<Box<Node>> {
        if target < &this.value {
            if let Some(left) = this.left.take() {
                this.left = Self::delete(left, target);
            }
            return Some(this);
        }

        if target > &this.value {
            if let Some(right) = this.right.take() {
                this.right = Self::delete(right, target);
            }
            return Some(this);
        }

        match (this.left.take(), this.right.take()) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(mut left), Some(right)) => {
                if let Some(mut rightmost) = left.rightmost_child() {
                    rightmost.left = Some(left);
                    rightmost.right = Some(right);
                    Some(rightmost)
                } else {
                    left.right = Some(right);
                    Some(left)
                }
            },
        }
    }

    fn rightmost_child(&mut self) -> Option<Box<Node>> {
        match self.right {
            Some(ref mut right) => {
                if let Some(t) = right.rightmost_child() {
                    Some(t)
                } else {
                    let mut r = self.right.take();
                    if let Some(ref mut r) = r {
                        self.right = std::mem::replace(&mut r.left, None);
                    }
                    r
                }
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: u64) {
        if let Some(ref mut root) = self.root {
            root.insert(value);
        } else {
            self.root = Some(Box::new(Node::new(value)));
        }
    }

    pub fn delete(&mut self, target: u64) {
        if let Some(root) = self.root.take() {
            self.root = Node::delete(root, &target);
        }
    }

    pub fn height(&self) -> u64 {
        match &self.root {
            Some(root) => root.height(),
            None => 0,
        }
    }

    pub fn print(&self) {
        if let Some(ref node) = self.root {
            let mut sb = String::new();
            node.print(&mut sb, "", "", false);
            println!("{}", sb);
        }
    }

    pub fn search(&self, value: u64) -> bool {
        if let Some(root) = &self.root {
            root.search(value)
        } else {
            false
        }
    }
}

