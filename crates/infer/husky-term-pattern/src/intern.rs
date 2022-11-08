use std::borrow::{Borrow, Cow};

use crate::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TermPatternInterner {
    patterns: Vec<TermPattern>,
    unresolved_registry: UnresolvedTermRegistry,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TermPatternItd {
    Resolved(TermItd),
    Unresolved(UnresolvedTermIdx),
    Composite(TermPatternIdx),
}

impl From<TermItd> for TermPatternItd {
    fn from(term: TermItd) -> Self {
        TermPatternItd::Resolved(term)
    }
}

impl From<UnresolvedTermIdx> for TermPatternItd {
    fn from(term: UnresolvedTermIdx) -> Self {
        TermPatternItd::Unresolved(term)
    }
}

pub enum TermPatternRef<'a> {
    Owned(TermPattern),
    Borrowed(&'a TermPattern),
}

impl<'a> Borrow<TermPattern> for TermPatternRef<'a> {
    fn borrow(&self) -> &TermPattern {
        match self {
            TermPatternRef::Owned(patt) => patt,
            TermPatternRef::Borrowed(patt) => patt,
        }
    }
}

impl<'a> std::ops::Deref for TermPatternRef<'a> {
    type Target = TermPattern;

    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}

impl TermPatternItd {
    fn opt_patt_idx(self) -> Option<TermPatternIdx> {
        match self {
            TermPatternItd::Resolved(_) | TermPatternItd::Unresolved(_) => None,
            TermPatternItd::Composite(idx) => Some(idx),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TermPatternIdx(usize);

impl TermPattern {
    fn max_dependee_idx(&self, itr: &TermPatternInterner) -> Option<TermPatternIdx> {
        match self {
            TermPattern::Resolved(_) | TermPattern::Unresolved(_) => unreachable!(),
            TermPattern::Application(app) => app.m().opt_patt_idx().max(app.n().opt_patt_idx()),
            TermPattern::Curry(curry) => curry.x().opt_patt_idx().max(curry.y().opt_patt_idx()),
            TermPattern::Subentity(subentity) => subentity.parent().opt_patt_idx(),
            TermPattern::TraitImpl(trait_impl) => trait_impl
                .ty()
                .opt_patt_idx()
                .max(trait_impl.trai().opt_patt_idx()),
        }
    }
}

impl TermPatternInterner {
    #[inline(always)]
    pub fn it(&mut self, patt: TermPattern) -> TermPatternItd {
        match patt {
            TermPattern::Resolved(term) => TermPatternItd::Resolved(term),
            TermPattern::Unresolved(term) => TermPatternItd::Unresolved(term),
            _ => TermPatternItd::Composite(self.alloc(patt)),
        }
    }

    pub fn it_unresolved(&mut self, term: UnresolvedTerm) -> TermPatternItd {
        let term = self.unresolved_registry.issue(term);
        self.it(TermPattern::Unresolved(term))
    }

    fn alloc(&mut self, patt: TermPattern) -> TermPatternIdx {
        if let Some(max_dependee_idx) = patt.max_dependee_idx(self) {
            let start = max_dependee_idx.0;
            for (i, patt1) in self.patterns[start..].iter().enumerate() {
                if patt1 == &patt {
                    return TermPatternIdx(start + i);
                }
            }
        }
        let raw = self.patterns.len();
        self.patterns.push(patt);
        TermPatternIdx(raw)
    }

    fn get<'a>(&'a self, itd: TermPatternItd) -> TermPatternRef {
        match itd {
            TermPatternItd::Resolved(term) => TermPatternRef::Owned(TermPattern::Resolved(term)),
            TermPatternItd::Unresolved(term) => {
                TermPatternRef::Owned(TermPattern::Unresolved(term))
            }
            TermPatternItd::Composite(idx) => TermPatternRef::Borrowed(&self.patterns[idx.0]),
        }
    }
}

#[test]
fn test_option_max_works() {
    assert!(None < Some(1))
}

#[test]
fn intern_works() {
    // let db = TermPatternTestsDb::new();
    let mut interner = TermPatternInterner::default();

    // HELP ME
}
