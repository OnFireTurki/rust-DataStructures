use std::{cell::RefCell, fmt::Display, ops::Deref, rc::Rc};
pub struct List<T: Display + Copy> {
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
    len: u64,
}

type NodePtr<T> = Rc<RefCell<Node<T>>>;

struct Node<T: Display + Copy> {
    pub value: T,
    pub next: Option<NodePtr<T>>,
}

impl<T: Display + Copy> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            value: val,
            next: None,
        }
    }
}

impl<T: Display + Copy> From<Node<T>> for Option<NodePtr<T>> {
    fn from(value: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(value)))
    }
}

impl<T: Display + Copy> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            len: 0,
        }
    }
    pub fn push_front(&mut self, val: T) {
        self.len += 1;
        let mut new_node = Node::new(val);
        match &mut self.head.take() {
            None => {
                self.head = new_node.into();
                self.tail = self.head.clone();
            }
            Some(curr_head) => {
                new_node.next = Some(curr_head.clone());
                self.head = new_node.into();
            }
        }
    }
    pub fn push_back(&mut self, val: T) {
        self.len += 1;
        let new_node = Node::new(val);
        match &mut self.tail.take() {
            None => {
                self.head = new_node.into();
                self.tail = self.head.clone();
            }
            Some(curr_tail) => {
                self.tail = new_node.into();
                curr_tail.borrow_mut().next = self.tail.clone();
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match &mut self.head.take() {
            None => None,
            Some(curr_head) => {
                self.len -= 1;
                let mut curr_head = curr_head.borrow_mut();
                let next = curr_head.next.take();
                match next {
                    None => {
                        self.tail.take();
                    }
                    Some(next) => {
                        self.head = Some(next);
                    }
                };
                Some(curr_head.value)
            }
        }
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn traverse(&self) {
        print!("List: [");
        let mut curr = self.head.as_ref().map(Rc::clone);
        while let Some(node) = curr {
            print!("{}", node.borrow().deref().value);
            curr = node.borrow().next.as_ref().map(Rc::clone);
            if curr.is_some() {
                print!(", ");
            }
        }
        print!("]; Len {}", self.len());
    }
}

impl<T: Display + Copy> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}
