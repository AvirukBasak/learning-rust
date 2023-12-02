use std::fmt::Display;

type NodeRefPtr<'a, T> = Option<&'a mut DllNode<'a, T>>;
type NodeOwnedPtr<'a, T> = Option<Box<DllNode<'a, T>>>;

struct DllNode<'a, T>
    where T: std::fmt::Display
{
    val: T,
    prev: NodeRefPtr<'a, T>,
    next: NodeOwnedPtr<'a, T>,
}

impl<'a, T> DllNode<'a, T>
    where T: std::fmt::Display
{
    fn new(
        val: T,
        prev: NodeRefPtr<'a, T>,
        next: NodeOwnedPtr<'a, T>,
    ) -> DllNode<'a, T> {
        DllNode { val, prev, next }
    }
}

struct DLL<'a, T>
    where T: std::fmt::Display
{
    front: NodeOwnedPtr<'a, T>,
    back: NodeRefPtr<'a, T>,
    length: i64,
}

impl<'a, T> Display for DLL<'a, T>
    where T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a, T> DLL<'a, T>
    where T: std::fmt::Display
{
    fn new() -> Self {
        DLL {
            front: None,
            back: None,
            length: 0,
        }
    }

    fn size(&'a self) -> i64 {
        self.length
    }

    fn push_back(&'a mut self, val: T) {
        let mut node = DllNode::new(val, self.back.take(), None);
        if self.front.is_none() && self.back.is_none() {
            // empty list
            let boxed_node = Box::new(node);
            self.front = Some(boxed_node);
            self.back = self.front.as_deref_mut();
        } else {
            // set back and back->next
            let back = node.prev.as_deref_mut().unwrap().next;
            let boxed_node = Box::new(node);
            
        }
        self.length += 1;
    }

    fn pop_back(&'a mut self) -> Option<T> {
        if self.back.is_none() {
            return None;
        }
        todo!()
    }

    fn push_front(&'a mut self, val: T) {
        // let node = DllNode::new(val, None, self.front);
        // let boxed_node = Box::new(node);
        // if self.front.is_none() && self.back.is_none() {
        //     // empty list
        //     self.front = Some(boxed_node);
        //     self.back = self.front.as_deref();
        // } else {
        //     // set front and front->prev
        //     self.front.unwrap().prev = Some(boxed_node.as_ref());
        //     self.front = Some(boxed_node);
        // }
        // self.length += 1;
    }

    fn pop_front(&'a mut self) -> Option<T> {
        if self.front.is_none() {
            return None;
        }
        todo!()
    }

    fn get_ref(&'a self, index: i64) -> Option<&DllNode<T>> {
        // if index < 0 || index >= self.length {
        //     return None;
        // }
        // let mut i: i64 = 0;
        // let mut ptr: Option<&DllNode<_>> = self.front.as_deref();
        // while i < index && ptr.is_some() {
        //     i += 1;
        //     ptr = ptr.unwrap().next.as_deref();
        // }
        // match ptr {
        //     Some(p) => ptr,
        //     _ => None,
        // }
        todo!()
    }

    fn get_mut(&'a mut self, index: i64) -> Option<&mut DllNode<T>> {
        // if index < 0 || index >= self.length {
        //     return None;
        // }
        // let mut i: i64 = 0;
        // let mut ptr: Option<&mut DllNode<_>> = self.front.as_deref_mut();
        // while i < index && ptr.is_some() {
        //     i += 1;
        //     ptr = ptr.unwrap().next.as_deref_mut();
        // }
        // match ptr {
        //     Some(p) => ptr,
        //     _ => None,
        // }
        todo!()
    }

    fn insert_at(&'a mut self, index: i64, val: T) {
        todo!()
    }
}

pub fn main() {
    let mut dll = DLL::new();
    dll.push_back(5);
    dll.push_back(7);
    dll.push_back(7);
    println!("{}", dll);
}
