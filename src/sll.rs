use std::{borrow::BorrowMut, rc::Rc};

struct List<T>
where
    T: std::fmt::Display + std::fmt::Debug,
{
    front: Option<Rc<ListNode<T>>>,
    back: Option<Rc<ListNode<T>>>,
    length: i64,
}

struct ListNode<T>
where
    T: std::fmt::Display + std::fmt::Debug,
{
    val: T,
    next: Option<Rc<ListNode<T>>>,
}

impl<T> List<T>
where
    T: std::fmt::Display + std::fmt::Debug,
{
    fn new() -> Self {
        List {
            front: None,
            back: None,
            length: 0,
        }
    }

    fn push(&mut self, val: T) {
        if self.length == 0 {
            self.front = Some(ListNode::new_ptr(val));
            self.back = Some(self.front.as_mut().unwrap().clone());
            return;
        }
        self.back.clone().unwrap().clone().next = Some(ListNode::new_ptr(val));
    }

    fn show(&self) {
        let mut ptr = &self.front;
        while ptr.is_some() {
            println!("{}", ptr.as_ref().unwrap().val);
            ptr = &ptr.as_ref().unwrap().next;
        }
    }
}

impl<T> ListNode<T>
where
    T: std::fmt::Display + std::fmt::Debug,
{
    fn new(val: T, next: Option<Rc<ListNode<T>>>) -> Self {
        ListNode { val, next }
    }

    fn pointered(node: ListNode<T>) -> Rc<ListNode<T>> {
        Rc::new(node)
    }

    fn new_ptr(val: T) -> Rc<ListNode<T>> {
        ListNode::pointered(ListNode::new(val, None))
    }

    fn append_after(&mut self, node: Option<Rc<ListNode<T>>>) {
        self.next = node;
    }
}

pub fn main() {
    let mut ls = List::new();
    ls.push(64);
    ls.push(95);
    ls.push(22);
    ls.show();
}
