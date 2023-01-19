use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct LocalTermSheet {
    local_term_interner: LocalTermInterner,
}

impl std::ops::Index<LocalTermCurryIdx> for LocalTermSheet {
    type Output = LocalTermCurry;

    fn index(&self, index: LocalTermCurryIdx) -> &Self::Output {
        &self.local_term_interner[index]
    }
}

impl std::ops::Index<LocalTermApplicationIdx> for LocalTermSheet {
    type Output = LocalTermApplication;

    fn index(&self, index: LocalTermApplicationIdx) -> &Self::Output {
        &self.local_term_interner[index]
    }
}

impl std::ops::Index<LocalTermAbstractionIdx> for LocalTermSheet {
    type Output = LocalTermAbstraction;

    fn index(&self, index: LocalTermAbstractionIdx) -> &Self::Output {
        &self.local_term_interner[index]
    }
}

impl std::ops::Index<LocalTermDurantIdx> for LocalTermSheet {
    type Output = LocalTermDurant;

    fn index(&self, index: LocalTermDurantIdx) -> &Self::Output {
        &self.local_term_interner[index]
    }
}

impl std::ops::Index<LocalTermSubentityIdx> for LocalTermSheet {
    type Output = LocalTermSubentity;

    fn index(&self, index: LocalTermSubentityIdx) -> &Self::Output {
        &self.local_term_interner[index]
    }
}

impl std::ops::Index<LocalTermAsTraitSubentityIdx> for LocalTermSheet {
    type Output = LocalTermAsTraitSubentity;

    fn index(&self, index: LocalTermAsTraitSubentityIdx) -> &Self::Output {
        &self.local_term_interner[index]
    }
}

impl std::ops::Index<LocalTermTraitConstraintIdx> for LocalTermSheet {
    type Output = LocalTermTraitConstraint;

    fn index(&self, index: LocalTermTraitConstraintIdx) -> &Self::Output {
        &self.local_term_interner[index]
    }
}
