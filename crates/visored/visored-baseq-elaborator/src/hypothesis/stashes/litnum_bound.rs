use super::*;
use crate::{
    foundations::{
        num::VdBsqSign,
        opr::separator::relation::comparison::{
            VdBsqBoundBoundaryKind, VdBsqBoundOpr, VdBsqComparisonOpr,
        },
    },
    hypothesis::{
        stack::VdBsqHypothesisStack,
        stash::{
            upgrade::{IsVdBsqHypothesisUpgradeStashScheme, VdBsqHypothesisUpgradeStash},
            IsVdBsqHypothesisStashScheme,
        },
    },
    term::{
        builder::{product::VdBsqProductBuilder, sum::VdBsqSumBuilder},
        comnum::{sum::VdBsqSumComnumTerm, VdBsqComnumTerm, VdBsqMonomialCoefficients},
        litnum::VdBsqLitnumTerm,
        num::VdBsqNumTerm,
        prop::VdBsqPropTerm,
        VdBsqTerm,
    },
};
use floated_sequential::db::FloaterDb;
use husky_control_flow_utils::require;

pub type LitnumBoundStash<'sess> = VdBsqHypothesisUpgradeStash<'sess, VdBsqLitNumBoundScheme>;

pub struct VdBsqLitNumBoundScheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdBsqLitNumBoundKey<'sess> {
    normalized_monomials: VdBsqNumTerm<'sess>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VdBsqLitNumBoundValue<'sess> {
    lower_bound_litnum: VdBsqLitnumTerm<'sess>,
    boundary_kind: VdBsqBoundBoundaryKind,
}

impl IsVdBsqHypothesisStashScheme for VdBsqLitNumBoundScheme {
    type Key<'sess> = VdBsqLitNumBoundKey<'sess>;

    type Value<'sess> = VdBsqLitNumBoundValue<'sess>;
}

impl IsVdBsqHypothesisUpgradeStashScheme for VdBsqLitNumBoundScheme {
    fn is_new_value_an_upgrade<'sess>(old: &Self::Value<'sess>, new: &Self::Value<'sess>) -> bool {
        if old == new {
            return false;
        }
        use husky_print_utils::*;
        p!(old, new);
        todo!()
    }

    fn key_value_from_hypothesis<'sess>(
        record: VdBsqHypothesisStackRecord<'sess>,
        entry: &VdBsqHypothesisEntry<'sess>,
        db: &'sess FloaterDb,
    ) -> Option<(Self::Key<'sess>, Self::Value<'sess>)> {
        let VdBsqTerm::Prop(VdBsqPropTerm::NumRelationship(term)) = entry.expr().term() else {
            return None;
        };
        let VdBsqComparisonOpr::Bound(opr) = term.kind() else {
            return None;
        };
        require!(let VdBsqNumTerm::Comnum(VdBsqComnumTerm::Sum(lhs_minus_rhs)) = term.lhs_minus_rhs());
        let sign = match opr {
            VdBsqBoundOpr::Lt => VdBsqSign::Minus,
            VdBsqBoundOpr::Gt => VdBsqSign::Plus,
            VdBsqBoundOpr::Le => VdBsqSign::Minus,
            VdBsqBoundOpr::Ge => VdBsqSign::Plus,
        };
        let (_, (litnum, normalized_monomials)) =
            lhs_minus_rhs.split_fld(|f| f.with_sign(sign, db), db);
        let lower_bound_litnum = litnum.neg(db);
        let boundary_kind = match opr {
            VdBsqBoundOpr::Lt => VdBsqBoundBoundaryKind::Open,
            VdBsqBoundOpr::Gt => VdBsqBoundBoundaryKind::Open,
            VdBsqBoundOpr::Le => VdBsqBoundBoundaryKind::Closed,
            VdBsqBoundOpr::Ge => VdBsqBoundBoundaryKind::Closed,
        };
        Some((
            VdBsqLitNumBoundKey {
                normalized_monomials,
            },
            VdBsqLitNumBoundValue {
                lower_bound_litnum,
                boundary_kind,
            },
        ))
    }
}
