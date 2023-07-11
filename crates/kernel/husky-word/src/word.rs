use crate::*;
use std::num::NonZeroUsize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Word(NonZeroUsize);

/// public methods
impl Word {
    pub fn text(self, db: &dyn WordDb) -> &str {
        vocabulary(db).word_text(self)
    }

    pub fn meanings(self, db: &dyn WordDb) -> &[WordMeaning] {
        vocabulary(db).word_meanings(self)
    }
}

/// protected methods
impl Word {
    #[inline(always)]
    pub(crate) fn index(self) -> usize {
        self.0.get()
    }
}

/// constants
impl Word {
    pub const LET: Word = Word(unsafe { NonZeroUsize::new_unchecked(1) });
    pub const BE: Word = Word(unsafe { NonZeroUsize::new_unchecked(2) });
    pub const SPACE: Word = Word(unsafe { NonZeroUsize::new_unchecked(3) });
    pub const TOPOLOGICAL: Word = Word(unsafe { NonZeroUsize::new_unchecked(4) });
    pub const SOME: Word = Word(unsafe { NonZeroUsize::new_unchecked(5) });
    pub const THAT: Word = Word(unsafe { NonZeroUsize::new_unchecked(6) });
}
