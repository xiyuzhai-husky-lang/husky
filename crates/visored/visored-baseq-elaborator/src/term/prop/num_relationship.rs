use crate::foundations::opr::separator::relation::comparison::VdBsqComparisonOpr;

use super::*;

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqNumRelationshipPropTerm<'sess>(VdBsqPropTermFld<'sess>);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqNumRelationshipPropTermData<'sess> {
    /// The left-hand side of the inequality minus the right-hand side.
    pub lhs_minus_rhs: VdBsqNumTerm<'sess>,
    pub opr: VdBsqComparisonOpr,
}

impl<'sess> VdBsqNumRelationshipPropTerm<'sess> {
    pub fn new(
        lhs: VdBsqNumTerm<'sess>,
        kind: VdBsqComparisonOpr,
        rhs: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> VdBsqPropTerm<'sess> {
        use husky_print_utils::*;
        let lhs_minus_rhs = lhs.sub(rhs, db);
        match lhs_minus_rhs {
            VdBsqNumTerm::Litnum(term) => {
                return VdBsqPropTerm::Trivial(term.compare_with_zero(kind))
            }
            VdBsqNumTerm::Comnum(term) => (),
        }
        VdBsqPropTerm::NumRelationship(Self(VdBsqPropTermFld::new(
            VdBsqPropTermData::NumRelationship(VdBsqNumRelationshipPropTermData {
                lhs_minus_rhs,
                opr: kind,
            }),
            db,
        )))
    }
}

impl<'sess> VdBsqPropTerm<'sess> {
    pub fn new_num_relationship(
        lhs: VdBsqNumTerm<'sess>,
        kind: VdBsqComparisonOpr,
        rhs: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqNumRelationshipPropTerm::new(lhs, kind, rhs, db)
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn new_num_relationship(
        lhs: VdBsqNumTerm<'sess>,
        kind: VdBsqComparisonOpr,
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

    pub fn lhs_minus_rhs(self) -> VdBsqNumTerm<'sess> {
        self.data().lhs_minus_rhs
    }

    pub fn kind(self) -> VdBsqComparisonOpr {
        self.data().opr
    }
}

impl<'sess> std::fmt::Debug for VdBsqNumRelationshipPropTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NumRelationshipPropTerm(`")?;
        self.show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqNumRelationshipPropTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.data().show_fmt(precedence_range, f)
    }
}

impl<'sess> VdBsqNumRelationshipPropTermData<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.lhs_minus_rhs.show_fmt(precedence_range, f)?;
        f.write_str(" ")?;
        match self.opr {
            VdBsqComparisonOpr::EQ => f.write_str("="),
            VdBsqComparisonOpr::NE => f.write_str("≠"),
            VdBsqComparisonOpr::LT => f.write_str("<"),
            VdBsqComparisonOpr::GT => f.write_str(">"),
            VdBsqComparisonOpr::LE => f.write_str("≤"),
            VdBsqComparisonOpr::GE => f.write_str("≥"),
        }?;
        f.write_str(" 0")
    }
}
