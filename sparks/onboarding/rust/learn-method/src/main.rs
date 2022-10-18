#[derive(Debug)]
struct Dog {
    name: String,
    weight: i32,
    height: i32,
}

impl Dog {
    fn say_hello(&self) {
        println!("Hello, world")
    }
    // bark
    fn bark(&self) {
        println!("wang")
    }
}

pub trait HasName {
    fn name(&self) -> &str;
}

// implement trait HasName for Dog
impl HasName for Dog {
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct Person {
    name: String,
}
// implement trait HasName for Person
impl HasName for Person {
    fn name(&self) -> &str /* string */ {
        &self.name
    }
}

fn main() {
    let a_dog = Dog {
        name: "sekiro".to_string(),
        height: 1,
        weight: 1,
    };
    let a_person = Person {
        name: "Alex".to_string(),
    };
    // apply function defined in line 24
    println!("A person's name is {}", a_person.name);
    println!("A dog's name is {}", a_dog.name());
    println!("A person's name is {}", a_person.name);
    println!("A person's name is {}", a_person.name());
}
