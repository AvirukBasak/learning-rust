// Define the node of the linked list
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Define the linked list
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    // Create a new empty linked list
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Add a new element to the front of the linked list
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    // Insert a new element at the end of the linked list
    pub fn append(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });

        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(new_node);
                return;
            }
            current = &mut node.next;
        }

        // If the list is empty, set the new node as the head
        self.head = Some(new_node);
    }

    // Pop the element from the front of the linked list, if any
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    // Get a reference to the head of the linked list
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

pub fn sll() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    println!("Peek: {:?}", list.peek()); // Output: Some(3)

    list.append(4);
    println!("Peek: {:?}", list.peek()); // Output: Some(3) - head remains unchanged

    println!("Pop: {:?}", list.pop());   // Output: Some(3)
    println!("Pop: {:?}", list.pop());   // Output: Some(2)
    println!("Pop: {:?}", list.pop());   // Output: Some(1)
    println!("Pop: {:?}", list.pop());   // Output: Some(4)
    println!("Pop: {:?}", list.pop());   // Output: None
}
