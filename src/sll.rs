struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn boxed(data: T, next: Option<Box<Node<T>>>) -> Box<Self> {
        Box::new(Node { data, next })
    }
}

pub struct LinkedList<'a, T> {
    front: Option<Box<Node<T>>>,
    back: Option<&'a mut Box<Node<T>>>,
    length: i64,
}

impl<'a, T> LinkedList<'a, T> {
    pub fn new() -> Self {
        LinkedList { front: None, back: None, length: 0 }
    }
    pub fn length(&self) -> i64 {
        self.length
    }
    pub fn push(&'a mut self, data: T) {
        let new_node = Node::boxed(data, self.front.take());
        self.front = Some(new_node);
        if self.back.is_none() {
            self.back = self.front.as_mut();
        }
        self.length += 1;
    }
    pub fn append(&'a mut self, data: T) {
        let new_node = Node::boxed(data, None);
        if let Some(back) = &mut self.back {
            back.next = Some(new_node);
            self.back = self.front.as_mut();
        } else {
            self.front = Some(new_node);
            self.back = self.front.as_mut();
        }
        self.length += 1;
    }
    pub fn pop(&'a mut self) -> Option<T> {
        self.length -= 1;
        self.front.take().map(|node| {
            self.front = node.next;
            if self.front.is_none() {
                self.back = None;
            }
            node.data
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.front.as_ref().map(|node| &node.data)
    }
}

pub fn sll() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("Peek: {:?}", list.peek()); // Output: Some(3)
    list.append(4);
    println!("Peek: {:?}", list.peek()); // Output: Some(3) - front remains unchanged
    println!("Pop: {:?}", list.pop());   // Output: Some(3)
    println!("Pop: {:?}", list.pop());   // Output: Some(2)
    println!("Pop: {:?}", list.pop());   // Output: Some(1)
    println!("Pop: {:?}", list.pop());   // Output: Some(4)
    println!("Pop: {:?}", list.pop());   // Output: None
}
