struct Dog {
    weight: i32,
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

fn main() {
    let a_dog = Dog { weight: 1 };
    a_dog.bark()
}
