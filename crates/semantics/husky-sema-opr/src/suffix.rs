#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemaSuffixOpr {
    Incr,
    Decr,
    Unveil,
    ComposeWithOption,
    Unwrap,
    ComposeWithNot,
}
