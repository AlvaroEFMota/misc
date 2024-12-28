use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }
}

#[derive(Debug)]
struct LinkedList {
    root: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { root: None }
    }

    fn insert(&mut self, item: i32) {
        if let Some(root_node) = &self.root {
            let mut tmp = Rc::clone(root_node);
            while let Some(node) = &tmp.clone().borrow().next {
                tmp = Rc::clone(node);
            }
            tmp.borrow_mut().next = Some(Rc::new(RefCell::new(Node::new(item))));
        } else {
            self.root = Some(Rc::new(RefCell::new(Node::new(item))));
        }
    }

    fn remove(&mut self, item: i32) {
        if let Some(root_node) = &self.root {
            let tmp = Rc::clone(root_node);
            if tmp.clone().borrow().value == item {
                self.root = tmp.borrow_mut().next.take();
            } else {
                let mut tmp = Rc::clone(root_node);

                while let Some(node) = &tmp.clone().borrow().next {
                    println!("comparando {} com {}", node.borrow().value, item);
                    if node.borrow().value == item {
                        break;
                    }
                    tmp = Rc::clone(node);
                }

                let mut prev = tmp.borrow_mut();
                println!("prev = {}", prev.value);
                let current = prev.next.take();
                let current = current.unwrap();
                prev.next = current.borrow_mut().next.take();
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.insert(9);
    list.insert(2);
    list.insert(6);
    list.insert(1);
    list.insert(0);
    list.insert(3);
    println!("{:?}", list);
    list.remove(6);
    println!("{:?}", list);
}
