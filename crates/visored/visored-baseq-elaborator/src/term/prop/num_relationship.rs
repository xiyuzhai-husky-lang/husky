use super::*;

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqNumRelationshipPropTerm<'sess>(VdBsqPropTermFld<'sess>);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqNumRelationshipPropTermData<'sess> {
    /// The left-hand side of the inequality minus the right-hand side.
    pub lhs_minus_rhs: VdBsqNumTerm<'sess>,
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
        db: &'sess FloaterDb,
    ) -> VdBsqPropTerm<'sess> {
        let lhs_minus_rhs = lhs.sub(rhs, db);
        match lhs_minus_rhs {
            VdBsqNumTerm::Rnum(term) => {
                return VdBsqPropTerm::Trivial(term.compare_with_zero(kind))
            }
            VdBsqNumTerm::Inum(term) => (),
        }
        VdBsqPropTerm::NumRelationship(Self(VdBsqPropTermFld::new(
            VdBsqPropTermData::NumRelationship(VdBsqNumRelationshipPropTermData {
                lhs_minus_rhs,
                kind,
            }),
            db,
        )))
    }
}

impl<'sess> VdBsqPropTerm<'sess> {
    pub fn new_num_relationship(
        lhs: VdBsqNumTerm<'sess>,
        kind: VdBsqNumRelationshipPropTermKind,
        rhs: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqNumRelationshipPropTerm::new(lhs, kind, rhs, db)
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn new_num_relationship(
        lhs: VdBsqNumTerm<'sess>,
        kind: VdBsqNumRelationshipPropTermKind,
        rhs: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqPropTerm::new_num_relationship(lhs, kind, rhs, db).into()
    }
}
