pub mod category;
pub mod function;
pub mod set;
pub mod trai;
pub mod trai_item;

use self::{category::*, function::*, set::*, trai::*, trai_item::*};
use lisp_csv::expr::{LpCsvExpr, LpCsvExprData};

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdItemPath {
    Category(VdCategoryPath),
    Set(VdSetPath),
    Function(VdFunctionPath),
    Trait(VdTraitPath),
    TraitItem(VdTraitItemPath),
}

impl VdItemPath {
    // # categories
    pub const SET: Self = VdItemPath::Category(VdCategoryPath::SET);
    pub const PROPOSITION: Self = VdItemPath::Category(VdCategoryPath::PROPOSITION);
    // # sets
    pub const NAT: Self = VdItemPath::Set(VdSetPath::NATURAL_NUMBER);
    pub const RAT: Self = VdItemPath::Set(VdSetPath::RATIONAL_NUMBER);
    pub const INT: Self = VdItemPath::Set(VdSetPath::INTEGER);
    pub const REAL: Self = VdItemPath::Set(VdSetPath::REAL_NUMBER);
    pub const COMPLEX: Self = VdItemPath::Set(VdSetPath::COMPLEX_NUMBER);
    // # functions
    pub const SIN: Self = VdItemPath::Function(VdFunctionPath::SIN);
    pub const COS: Self = VdItemPath::Function(VdFunctionPath::COS);
    // # trait items
    pub const RING_ADD: Self = VdItemPath::TraitItem(VdTraitItemPath::RING_ADD);
    pub const RING_MUL: Self = VdItemPath::TraitItem(VdTraitItemPath::RING_MUL);
    pub const RING_POS: Self = VdItemPath::TraitItem(VdTraitItemPath::RING_POS);
    pub const RING_NEG: Self = VdItemPath::TraitItem(VdTraitItemPath::RING_NEG);
    pub const RING_SUB: Self = VdItemPath::TraitItem(VdTraitItemPath::RING_SUB);
    pub const FIELD_DIV: Self = VdItemPath::TraitItem(VdTraitItemPath::FIELD_DIV);
}

impl VdItemPath {
    pub fn from_lp_csv_expr(expr: &LpCsvExpr) -> Self {
        let LpCsvExprData::Ident(ref ident) = expr.data else {
            todo!("expected identifier but got {:#?}", expr.data)
        };
        match ident as &str {
            "ring_pos" => VdItemPath::RING_POS,
            "ring_neg" => VdItemPath::RING_NEG,
            "ring_add" => VdItemPath::RING_ADD,
            "ring_sub" => VdItemPath::RING_SUB,
            "ring_mul" => VdItemPath::RING_MUL,
            "ring_neg" => VdItemPath::RING_NEG,
            "field_div" => VdItemPath::FIELD_DIV,
            s => todo!("s = {s:?} not handled"),
        }
    }
}
