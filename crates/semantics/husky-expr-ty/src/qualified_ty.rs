use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct QualifiedTypeIdx(LocalTermIdx);

impl Into<LocalTerm> for QualifiedTypeIdx {
    fn into(self) -> LocalTerm {
        self.0.into()
    }
}
