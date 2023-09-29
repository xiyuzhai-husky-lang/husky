#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub enum SemaPrefixOpr {
    Minus,
    Not,
    BitNot,
    Leash,
    Ref,
    Option,
}
