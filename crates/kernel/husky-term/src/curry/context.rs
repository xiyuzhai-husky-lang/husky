use interner::{InternedRefWrapper, Interner};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermCurryContext {
    function: (),
    idx: usize,
}

pub type TermCurryContextItd = InternedRefWrapper<TermCurryContext>;

pub type TermCurryContextInterner = Interner<TermCurryContextItd>;
