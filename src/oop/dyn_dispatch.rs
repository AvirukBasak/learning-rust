use crate::helperfn::get_filename;
use crate::helperfn::input;

trait Animal {
    fn get_name(&self) -> &str;
    fn print(&self);
}

struct Dog {
    name: String,
}

impl Dog {
    fn new(name: &str) -> Self {
        Dog {
            name: name.to_owned(),
        }
    }
}

impl Animal for Dog {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn print(&self) {
        println!("{} is a dog", self.get_name());
    }
}

struct Cat {
    name: String,
}

impl Cat {
    fn new(name: &str) -> Self {
        Cat {
            name: name.to_owned(),
        }
    }
}

impl Animal for Cat {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn print(&self) {
        println!("{} is a cat", self.get_name());
    }
}

fn display_animal_dynamic_ref(animal: &dyn Animal) {
    animal.print();
}

fn display_animal_dynamic_box_ref(animal: &Box<dyn Animal>) {
    animal.print();
}

fn display_animal_dynamic_box(animal: Box<dyn Animal>) {
    animal.print();
}

fn display_animal_dynamic_ptr(animal: *const dyn Animal) {
    // let animal = unsafe { animal.as_ref() };
    // animal.expect("invalid animal pointer").print();
    unsafe {
        (*animal).print();
    }
}

pub fn main() {
    let animal_type: String = input(&format!("{}: Enter animal type: ", get_filename(file!())));
    let animal_name: String = input(&format!("{}: Enter animal name: ", get_filename(file!())));
    let animal: /* *const dyn Animal */ Box<dyn Animal> = match &animal_type.to_lowercase()[..] {
        "dog" => /* &* */ Box::new(Dog::new(&animal_name)),
        "cat" => /* &* */ Box::new(Cat::new(&animal_name)),
        _ => panic!("unknown animal type '{}'", animal_type),
    };
    // display_animal_dynamic_ptr(animal);
    display_animal_dynamic_ref(&*animal);
    display_animal_dynamic_box_ref(&animal);
    display_animal_dynamic_ptr(&*animal);
    display_animal_dynamic_box(animal); // Box is moved here
}
