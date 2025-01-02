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

    fn print(&self) {
        if let Some(root_node) = &self.root {
            let mut tmp = Rc::clone(root_node);
            print!("{}", tmp.borrow().value);
            while let Some(node) = &tmp.clone().borrow().next {
                tmp = Rc::clone(node);
                print!(" -> {}", tmp.borrow().value);
            }
            println!("");
        } else {
            println!("Empty");
        }
    }

    fn merge(&self) -> LinkedList {
        let mut list = LinkedList::new();
        if let Some(root_node) = &self.root {
            let mut slower = Rc::clone(root_node);
            let mut faster = if let Some(node) = &root_node.borrow().next {
                Rc::clone(node)
            } else {
                list.insert(root_node.borrow().value);
                return list;
            };

            loop {
                slower = Rc::clone(slower.clone().borrow().next.as_ref().unwrap());
                if let Some(n1) = &faster.clone().borrow().next {
                    if let Some(n2) = &n1.borrow().next {
                        faster = Rc::clone(n2);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            let mid = slower.clone();
            let mut p1 = root_node.clone();
            let mut p2 = mid.clone();

            list.insert(p1.borrow().value);
            list.insert(p2.borrow().value);
            loop {
                if let (Some(n1), Some(n2)) = (&p1.clone().borrow().next, &p2.clone().borrow().next)
                {
                    if Rc::ptr_eq(&n1, &mid) {
                        break;
                    }

                    list.insert(n1.borrow().value);
                    list.insert(p2.borrow().value);
                    p1 = Rc::clone(n1);
                    p2 = Rc::clone(n2);
                } else {
                    break;
                }
            }

            if let Some(n2) = &p2.clone().borrow().next {
                list.insert(n2.borrow().value);
            }
        }
        return list;
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.insert(1);
    list.insert(3);
    list.insert(5);
    list.insert(7);
    list.insert(9);
    list.insert(11);
    list.insert(2);
    list.insert(4);
    list.insert(6);
    list.insert(8);
    list.insert(10);
    list.insert(12);
    list.insert(14);
    list.print();
    let list2 = list.merge();
    list2.print();
}
