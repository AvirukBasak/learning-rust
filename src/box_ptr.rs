struct Child {
    data: i32,
}

struct Parent {
    child: Box<Child>,
}

impl Parent {
    fn modify_child_data(&mut self, new_data: i32) {
        self.child.data = new_data;
    }
}

pub fn box_ptr() {
    let mut parent = Parent {
        child: Box::new(Child { data: 42 }),
    };
    parent.modify_child_data(99);
    println!("Modified child data: {}", parent.child.data);
}
