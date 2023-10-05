use enum_index::*;

#[derive(Clone, Copy, IsEnumIndex)]
pub enum Animal {
    Dog,
    Cat,
    Turtle,
    Bat,
    Penguin,
}
