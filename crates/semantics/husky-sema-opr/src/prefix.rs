#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub enum SemaPrefixOpr {
    Minus,
    Not,
    BitNot,
    LeashType,
    RefType,
    Option,
}
