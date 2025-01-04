pub mod inum;
pub mod rnum;

use self::{inum::*, rnum::*};
use crate::elaborator::VdBsqElaboratorInner;
use floated_sequential::db::FloaterDb;
use floated_sequential::floated;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;
use visored_mir_expr::{
    expr::{VdMirExprData, VdMirExprEntry},
    symbol::local_defn::{
        storage::VdMirSymbolLocalDefnStorage, VdMirSymbolLocalDefnHead, VdMirSymbolLocalDefnIdx,
    },
};

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    Inum(VdBsqInumTerm<'sess>),
    Prop,
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

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn calc_expr_term(
        &self,
        expr_entry: &VdMirExprEntry,
        symbol_local_defn_storage: &VdMirSymbolLocalDefnStorage,
    ) -> VdBsqTerm<'sess> {
        match *expr_entry.data() {
            VdMirExprData::Literal(vd_literal) => todo!(),
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
            } => todo!(),
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
            } => todo!(),
            VdMirExprData::ItemPath(vd_item_path) => todo!(),
        }
    }
}
