use crate::*;

pub struct TermPatternContext<'a> {
    db: &'a dyn TermPatternDb,
}

impl<'a> TermPatternContext<'a> {
    pub(crate) fn new(db: &'a dyn TermPatternDb) -> Self {
        Self { db }
    }
}
