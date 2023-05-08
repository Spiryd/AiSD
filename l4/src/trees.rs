use std::fmt::Write;

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

    fn insert(&mut self, value: u64) {
        if value < self.value {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(Node::new(value)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(Node::new(value)));
            }
        }
    }

    fn search(&self, value: u64) -> bool {
        if self.value == value {
            return true;
        }

        if value < self.value {
            if let Some(left) = &self.left {
                return left.search(value);
            } else {
                return false;
            }
        } else {
            if let Some(right) = &self.right {
                return right.search(value);
            } else {
                return false;
            }
        }
    }

    fn delete(&mut self, value: u64) {
        todo!()
    }

    fn traverse_nodes(&self, sb: &mut String, padding: &str, pointer: &str, has_right_sibling: bool) {
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
            left.traverse_nodes(sb, padding_for_both, pointer_left, self.right.is_some());
        }
        if let Some(right) = &self.right {
            right.traverse_nodes(sb, padding_for_both, pointer_right, false);
        }
    }
}

pub struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, value: u64) {
        if let Some(root) = &mut self.root {
            root.insert(value);
        } else {
            self.root = Some(Box::new(Node::new(value)));
        }
    }

    pub fn search(&self, value: u64) -> bool {
        if let Some(root) = &self.root {
            return root.search(value);
        } else {
            return false;
        }
    }

    pub fn print(&self) {
        if let Some(ref node) = self.root {
            let mut sb = String::new();
            node.traverse_nodes(&mut sb, "", "", false);
            println!("{}", sb);
        }
    }

    pub fn delete(&mut self, value: u64) {
        if let Some(root) = &mut self.root {
            root.delete(value);
        } 
    }
}
