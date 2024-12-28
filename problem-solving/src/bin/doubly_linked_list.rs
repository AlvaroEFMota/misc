use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

struct DoublyLinkedList {
    start: Option<Rc<RefCell<Node>>>,
    end: Option<Rc<RefCell<Node>>>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            start: None,
            end: None,
        }
    }

    fn remove(&mut self, value: i32) {
        if let (Some(start_node), Some(end_node)) = (self.start.take(), self.end.take()) {
            if Rc::ptr_eq(&start_node, &end_node) && start_node.borrow().value == value {
                return;
            }

            let mut tmp = Rc::clone(&start_node);
            if tmp.clone().borrow().value == value {
                if let Some(next_node) = &tmp.borrow().next {
                    next_node.borrow_mut().prev.take();
                }
                self.start = tmp.borrow_mut().next.take();
                self.end = Some(end_node);
                return;
            }

            while let Some(node) = &tmp.clone().borrow().next {
                if node.borrow().value == value {
                    break;
                }
                tmp = Rc::clone(node);
            }

            let del_node = {
                let del_node_borrowed = tmp.borrow();
                del_node_borrowed.next.as_ref().unwrap().clone()
            };

            //let del_node = if let Some(del_node) = &tmp.borrow().next {
            //    del_node.clone()
            //} else {
            //    panic!("panic message")
            //};
            //
            if Rc::ptr_eq(&del_node, &end_node) {
                tmp.borrow_mut().next.take();
                self.end = Some(tmp.clone());
                self.start = Some(start_node);
                return;
            }

            if let Some(tmp1) = &del_node.borrow().next {
                tmp.borrow_mut().next = Some(Rc::clone(tmp1));
                tmp1.borrow_mut().prev = Some(tmp.clone());
            }

            self.start = Some(start_node);
            self.end = Some(end_node);
        }
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.insert(4);
    list.insert(3);
    list.insert(2);
    list.insert(1);
    list.insert(0);
    list.print();
    list.inv_print();
    list.remove(2);
    list.print();
    list.inv_print();
    list.remove(4);
    list.print();
    list.inv_print();
    list.remove(0);
    list.print();
    list.inv_print();
    list.remove(3);
    list.print();
    list.inv_print();
    list.remove(1);
    list.print();
    list.inv_print();
}

impl DoublyLinkedList {
    fn insert(&mut self, value: i32) {
        if let Some(start_node) = &self.start {
            let mut tmp = Rc::clone(start_node);
            while let Some(node) = &tmp.clone().borrow().next {
                tmp = Rc::clone(node);
            }
            let mut new_node = Node::new(value);
            new_node.prev = Some(Rc::clone(&tmp));
            let new_node_rc = Rc::new(RefCell::new(new_node));
            tmp.borrow_mut().next = Some(new_node_rc.clone());
            self.end = Some(new_node_rc);
        } else {
            let new_node = Rc::new(RefCell::new(Node::new(value)));
            self.start = Some(Rc::clone(&new_node));
            self.end = Some(new_node);
        }
    }

    fn print(&self) {
        if let Some(start_node) = &self.start {
            let mut tmp = Rc::clone(start_node);
            print!("{}", tmp.borrow().value);
            while let Some(node) = &tmp.clone().borrow().next {
                print!("<->{}", node.borrow().value);
                tmp = Rc::clone(node)
            }
            println!("");
        } else {
            println!("Empty");
        }
    }

    fn inv_print(&self) {
        if let Some(end_node) = &self.end {
            let mut tmp = Rc::clone(end_node);
            print!("{}", tmp.borrow().value);
            while let Some(node) = &tmp.clone().borrow().prev {
                print!("<->{}", node.borrow().value);
                tmp = Rc::clone(node);
            }
            println!("");
        } else {
            println!("Empty");
        }
    }
}
