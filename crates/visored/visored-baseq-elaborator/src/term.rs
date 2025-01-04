use floated_sequential::{db::FloaterDb, floated};
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;
use visored_mir_expr::{
    expr::{VdMirExprData, VdMirExprEntry},
    symbol::local_defn::{
        storage::VdMirSymbolLocalDefnStorage, VdMirSymbolLocalDefnHead, VdMirSymbolLocalDefnIdx,
    },
};

use crate::elaborator::VdBaseqElaboratorInner;

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdMirTerm<'sess> {
    RationalNumeric(RationalNumericVdBaseqTerm),
    IrrationalNumeric(IrrationalNumericVdBaseqTerm<'sess>),
    Prop,
}

impl<'sess> std::fmt::Debug for VdMirTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum RationalNumericVdBaseqTerm {
    Nat128(u128),
    Int128(i128),
    BigInt(/* TODO */),
    Rat128(i128, u128),
}

#[floated]
pub struct IrrationalNumericVdBaseqTerm<'sess> {
    #[return_ref]
    pub data: IrrationalNumericVdBaseqTermData<'sess>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum IrrationalNumericVdBaseqTermData<'sess> {
    Product {
        rational: RationalNumericVdBaseqTerm,
        irrational_atom_exponentials: IrrationalAtomExponentials<'sess>,
    },
    Sum {
        constant_term: RationalNumericVdBaseqTerm,
        irrational_monomial_coefficients: IrrationalMonomialCoefficients<'sess>,
    },
    Variable(VdMirSymbolLocalDefnIdx),
}

pub type IrrationalMonomialCoefficients<'sess> =
    IrrationalTermMap<'sess, RationalNumericVdBaseqTerm>;
pub type IrrationalAtomExponentials<'sess> =
    IrrationalTermMap<'sess, IrrationalNumericVdBaseqTerm<'sess>>;

pub type IrrationalTermMap<'sess, T> =
    OrderedSmallVecPairMap<IrrationalNumericVdBaseqTerm<'sess>, T, 4>;

impl<'sess> std::fmt::Debug for IrrationalNumericVdBaseqTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'sess> VdMirTerm<'sess> {
    pub fn new_numeric_variable(
        local_defn_idx: VdMirSymbolLocalDefnIdx,
        db: &'sess FloaterDb,
    ) -> Self {
        VdMirTerm::IrrationalNumeric(IrrationalNumericVdBaseqTerm::new(
            IrrationalNumericVdBaseqTermData::Variable(local_defn_idx),
            db,
        ))
    }
}

impl<'db, 'sess> VdBaseqElaboratorInner<'db, 'sess> {
    pub fn calc_expr_term(
        &self,
        expr_entry: &VdMirExprEntry,
        symbol_local_defn_storage: &VdMirSymbolLocalDefnStorage,
    ) -> VdMirTerm<'sess> {
        match *expr_entry.data() {
            VdMirExprData::Literal(vd_literal) => todo!(),
            VdMirExprData::Variable(local_defn_idx) => {
                let lx_math_letter =
                    match *symbol_local_defn_storage.defn_arena()[local_defn_idx].head() {
                        VdMirSymbolLocalDefnHead::Letter(lx_math_letter) => lx_math_letter,
                    };
                if expr_entry.ty().is_numeric(self.eterner_db()) {
                    VdMirTerm::new_numeric_variable(local_defn_idx, self.floater_db())
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
