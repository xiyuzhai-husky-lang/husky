use super::*;

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqNumRelationshipPropTerm<'sess>(VdBsqPropTermFld<'sess>);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqNumRelationshipPropTermData<'sess> {
    /// The left-hand side of the inequality.
    /// The right-hand side is always reduced to `0`.
    pub lhs: VdBsqNumTerm<'sess>,
    pub kind: VdBsqNumRelationshipPropTermKind,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNumRelationshipPropTermKind {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

impl<'sess> VdBsqNumRelationshipPropTerm<'sess> {
    pub fn new(
        lhs: VdBsqNumTerm<'sess>,
        kind: VdBsqNumRelationshipPropTermKind,
        rhs: VdBsqNumTerm<'sess>,
    ) -> Self {
        todo!()
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn new_num_relationship(
        lhs: VdBsqNumTerm<'sess>,
        kind: VdBsqNumRelationshipPropTermKind,
        rhs: VdBsqNumTerm<'sess>,
    ) -> Self {
        todo!()
    }
}
