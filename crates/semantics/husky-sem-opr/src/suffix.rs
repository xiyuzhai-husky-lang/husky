#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemaSuffixOpr {
    Incr,
    Decr,
    ComposeWithOption,
    ComposeWithNot,
}
