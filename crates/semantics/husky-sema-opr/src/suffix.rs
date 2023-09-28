#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemaSuffixOpr {
    Incr,
    Decr,
    Unveil,
    ComposeWithOption,
    Unwrap,
    ComposeWithNot,
}
