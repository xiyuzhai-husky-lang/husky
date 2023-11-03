#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirSuffixOpr {
    Incr,
    Decr,
    Unveil,
    Unwrap,
}
