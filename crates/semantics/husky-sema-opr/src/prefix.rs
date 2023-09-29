#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum SemaPrefixOpr {
    Minus,
    Not,
    BitNot,
    Leash,
    Ref,
    Option,
}
