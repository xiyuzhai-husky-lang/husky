use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynSuffixOpr {
    Incr, // ++
    Attr, // --
    /// there are two cases
    /// - index `$opd[$items]` where `$opd` can be indexed
    /// - compose with functor `Option` `$opd ?` where `$opd` is of type `Option _ -> S`
    /// the cases are determined by whether `$opd` is of curry type
    UnveilOrComposeWithOption,
    UnwrapOrComposeWithNot,
}

impl SynSuffixOpr {
    pub fn code(&self) -> Cow<'static, str> {
        match self {
            SynSuffixOpr::Incr => "++".into(),
            SynSuffixOpr::Attr => "--".into(),
            SynSuffixOpr::UnveilOrComposeWithOption => "?".into(),
            SynSuffixOpr::UnwrapOrComposeWithNot => "!".into(),
        }
    }
}
