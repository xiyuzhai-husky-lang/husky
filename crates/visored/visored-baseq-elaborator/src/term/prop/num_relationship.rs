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

impl<'sess> VdBsqNumRelationshipPropTerm<'sess> {
    pub fn data(self) -> &'sess VdBsqNumRelationshipPropTermData<'sess> {
        match self.0.data() {
            VdBsqPropTermData::NumRelationship(data) => data,
            _ => unreachable!(),
        }
    }
}

impl<'sess> std::fmt::Debug for VdBsqNumRelationshipPropTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NumRelationshipPropTerm(")?;
        self.show_fmt(f)?;
        f.write_str(")")
    }
}

impl<'sess> VdBsqNumRelationshipPropTerm<'sess> {
    pub fn show_fmt(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data().show_fmt(f)
    }
}

impl<'sess> VdBsqNumRelationshipPropTermData<'sess> {
    pub fn show_fmt(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.lhs_minus_rhs.show_fmt(f)?;
        f.write_str(" ")?;
        match self.kind {
            VdBsqNumRelationshipPropTermKind::Eq => f.write_str("="),
            VdBsqNumRelationshipPropTermKind::Ne => f.write_str("≠"),
            VdBsqNumRelationshipPropTermKind::Lt => f.write_str("<"),
            VdBsqNumRelationshipPropTermKind::Gt => f.write_str(">"),
            VdBsqNumRelationshipPropTermKind::Le => f.write_str("≤"),
            VdBsqNumRelationshipPropTermKind::Ge => f.write_str("≥"),
        }?;
        f.write_str(" 0")
    }
}
