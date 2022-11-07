#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UnresolvedTermIdx(usize);

pub struct UnresolvedTermRegistry {
    terms: Vec<UnresolvedTerm>,
}

pub struct UnresolvedTerm {}

impl UnresolvedTermRegistry {
    fn issue(&mut self, term: UnresolvedTerm) -> UnresolvedTermIdx {
        let raw = self.terms.len();
        self.terms.push(term);
        UnresolvedTermIdx(raw)
    }
}
