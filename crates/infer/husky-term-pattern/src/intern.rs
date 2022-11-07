use crate::*;

pub struct TermPatternInterner {
    patterns: Vec<TermPattern>,
}

#[derive(Debug, Clone, Copy)]
pub enum TermPatternItd {
    Resolved(TermItd),
    Unresolved(UnresolvedTermIdx),
    Composite(TermPatternIdx),
}

#[derive(Debug, Clone, Copy)]
pub struct TermPatternIdx(usize);

impl TermPatternInterner {
    pub(crate) fn it(&mut self, patt: TermPattern) -> TermPatternItd {
        match patt {
            TermPattern::Resolved(term) => TermPatternItd::Resolved(term),
            TermPattern::Unresolved(term) => todo!(),
            _ => TermPatternItd::Composite(self.alloc(patt)),
        }
    }

    fn alloc(&mut self, patt: TermPattern) -> TermPatternIdx {
        let raw = self.patterns.len();
        self.patterns.push(patt);
        TermPatternIdx(raw)
    }
}
