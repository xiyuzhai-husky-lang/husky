pub mod inum;
pub mod rnum;

use self::{inum::*, rnum::*};
use crate::elaborator::VdBsqElaboratorInner;
use floated_sequential::db::FloaterDb;
use floated_sequential::floated;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;
use visored_mir_expr::{
    expr::{application::VdMirFunc, VdMirExprData, VdMirExprEntry},
    symbol::local_defn::{
        storage::VdMirSymbolLocalDefnStorage, VdMirSymbolLocalDefnHead, VdMirSymbolLocalDefnIdx,
    },
};
use visored_term::term::literal::VdLiteralData;

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    Inum(VdBsqInumTerm<'sess>),
    Prop,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNumTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    Inum(VdBsqInumTerm<'sess>),
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn num(self) -> Option<VdBsqNumTerm<'sess>> {
        match self {
            VdBsqTerm::Rnum(rnum) => Some(VdBsqNumTerm::Rnum(rnum)),
            VdBsqTerm::Inum(inum) => Some(VdBsqNumTerm::Inum(inum)),
            VdBsqTerm::Prop => None,
        }
    }
}

impl<'sess> std::fmt::Debug for VdBsqTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'sess> std::fmt::Debug for VdBsqInumTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'sess> std::fmt::Debug for VdBsqNumTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn calc_expr_term(
        &self,
        expr_entry: &VdMirExprEntry,
        symbol_local_defn_storage: &VdMirSymbolLocalDefnStorage,
    ) -> VdBsqTerm<'sess> {
        match *expr_entry.data() {
            VdMirExprData::Literal(vd_literal) => match *vd_literal.data() {
                VdLiteralData::Nat128(n) => VdBsqTerm::Rnum(VdBsqRnumTerm::Nat128(n)),
                VdLiteralData::Int128(i) => VdBsqTerm::Rnum(VdBsqRnumTerm::Int128(i)),
                VdLiteralData::Float(_) => todo!(),
                VdLiteralData::SpecialConstant(vd_special_constant) => todo!(),
            },
            VdMirExprData::Variable(local_defn_idx) => {
                let lx_math_letter =
                    match *symbol_local_defn_storage.defn_arena()[local_defn_idx].head() {
                        VdMirSymbolLocalDefnHead::Letter(lx_math_letter) => lx_math_letter,
                    };
                if expr_entry.ty().is_numeric(self.eterner_db()) {
                    VdBsqTerm::new_numeric_variable(local_defn_idx, self.floater_db())
                } else {
                    todo!()
                }
            }
            VdMirExprData::Application {
                function,
                arguments,
            } => match function {
                VdMirFunc::NormalBasePrefixOpr(vd_base_prefix_opr_signature) => todo!(),
                VdMirFunc::NormalBaseSeparator(vd_base_separator_signature) => todo!(),
                VdMirFunc::NormalBaseBinaryOpr(vd_base_binary_opr_signature) => todo!(),
                VdMirFunc::Power(vd_power_signature) => {
                    assert_eq!(arguments.len(), 2);
                    let Some(base) = self.expr_fld(arguments.first().unwrap()).term().num() else {
                        todo!()
                    };
                    let Some(exponent) = self.expr_fld(arguments.last().unwrap()).term().num()
                    else {
                        todo!()
                    };
                    VdBsqTerm::new_power(base, exponent, self.floater_db())
                }
                VdMirFunc::InSet => todo!(),
                VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
                VdMirFunc::NormalBaseFrac(vd_base_binary_opr_signature) => todo!(),
            },
            VdMirExprData::FoldingSeparatedList {
                leader,
                ref followers,
            } => {
                todo!()
            }
            VdMirExprData::ChainingSeparatedList {
                leader,
                ref followers,
                joined_separator_and_signature,
            } => {
                todo!();
                todo!()
            }
            VdMirExprData::ItemPath(vd_item_path) => todo!(),
        }
    }
}
