use interner::{DefaultItd, Interner};

#[derive(Debug)]
pub struct TermCurryContext {
    function: (),
    idx: usize,
}

pub type TermCurryContextItd = DefaultItd<TermCurryContext, TermCurryContext>;

pub type TermCurryContextInterner = Interner<TermCurryContextItd>;
