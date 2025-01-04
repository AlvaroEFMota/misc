fn main() {}

struct HeapNode {
    value: i32,
    left: Option<Box<HeapNode>>,
    right: Option<Box<HeapNode>>,
}

impl HeapNode {
    fn new(value: i32) -> Self {
        HeapNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            if let Some(node) = &mut self.left {
                node.insert(value);
            } else {
                self.left = Some(Box::new(HeapNode::new(value)));
            }
        } else {
            if let Some(node) = &mut self.right {
                node.insert(value);
            } else {
                self.right = Some(Box::new(HeapNode::new(value)));
            }
        }
    }
}
