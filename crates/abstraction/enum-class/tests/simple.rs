#[enum_class::from_variants]
enum Animal {
    Dog(Dog),
    Cat(Cat),
}

struct Dog;
struct Cat;
