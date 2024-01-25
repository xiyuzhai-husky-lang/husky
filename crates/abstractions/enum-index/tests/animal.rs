use enum_index::*;

#[derive(Clone, Copy, PartialEq, Eq, IsEnumIndex)]
pub enum Animal {
    Dog,
    Cat,
    Turtle,
    Bat,
    Penguin,
}
