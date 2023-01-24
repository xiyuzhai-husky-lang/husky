use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnresolvedTermIdx(usize);

pub(crate) struct LocalTermTableEntry {
    unresolved_term: UnresolvedTerm,
    resolve_progress: LocalTerm,
}

pub(crate) struct LocalTermTable {
    entries: Vec<LocalTermTableEntry>,
}

impl LocalTermTable {
    pub(crate) fn unresolved_term(
        &self,
        unresolved_term_idx: UnresolvedTermIdx,
    ) -> &UnresolvedTerm {
        &self.entries[unresolved_term_idx.0].unresolved_term
    }

    pub(crate) fn intern(&mut self, unresolved_term: UnresolvedTerm) -> UnresolvedTermIdx {
        let position = self
            .entries
            .iter()
            .position(|entry| entry.unresolved_term == unresolved_term);
        match position {
            Some(idx) => UnresolvedTermIdx(idx),
            None => {
                let idx = self.entries.len();
                self.entries.push(LocalTermTableEntry {
                    unresolved_term,
                    resolve_progress: LocalTerm::Unresolved(UnresolvedTermIdx(idx)),
                });
                UnresolvedTermIdx(idx)
            }
        }
    }
}
