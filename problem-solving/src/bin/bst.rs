#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value > self.value {
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(Node::new(value)));
            }
        } else {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(Node::new(value)));
            }
        }
    }
}

#[derive(Debug)]
struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: i32) {
        if let Some(root_node) = &mut self.root {
            root_node.insert(value);
        } else {
            self.root = Some(Box::new(Node::new(value)));
        }
    }
}

fn main() {
    let mut bst = BinarySearchTree::new();
    bst.insert(5);
    bst.insert(8);
    bst.insert(3);
    bst.insert(9);
    println!("{:?}", bst);
}
